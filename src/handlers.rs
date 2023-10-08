//		Modules

#[cfg(test)]
#[path = "tests/handlers.rs"]
mod tests;



//		Packages

use crate::{
	ASSETS_DIR,
	CONTENT_DIR,
	middlewares::StatsContext,
	utility::*,
};
use rustmark::{Heading, self};

use axum::{
	Extension,
	Json,
	body::Body,
	extract::State,
	http::{HeaderValue, StatusCode, Uri, header},
	response::{IntoResponse, Response},
};
use chrono::{NaiveDateTime, Utc};
use mime_guess::{self};
use serde::Serialize;
use serde_json::{self};
use std::{
	collections::HashMap,
	fs,
	sync::{Arc, atomic::Ordering},
};
use tera::Context;
use tokio::{
	fs::File,
	io::{AsyncReadExt, BufReader},
};
use tokio_util::io::ReaderStream;
use utoipa::ToSchema;



//		Enums

//		AssetContext															
/// The protection contexts for static assets.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AssetContext {
	/// Public files.
	Public,
	
	/// Protected files.
	Protected,
}



//		Structs

//		StatsResponse															
/// The application statistics returned by the `/api/stats` endpoint.
#[derive(Serialize, ToSchema)]
pub struct StatsResponse {
	//		Public properties													
	/// The date and time the application was started.
	pub started_at: NaiveDateTime,
	
	/// The amount of time the application has been running.
	pub uptime:     u64,
	
	/// The number of requests that have been handled.
	pub requests:   u64,
	
	/// The number of responses that have been handled, along with the average,
	/// maximum, and minimum response times by time period.
	pub responses:  StatsResponseResponses,
}

//		StatsResponseResponses													
/// Counts and times of responses.
#[derive(Serialize, ToSchema)]
pub struct StatsResponseResponses {
	//		Public properties													
	/// The counts of responses.
	pub counts: StatsResponseResponseCounts,
	
	/// The times of responses.
	pub times:  StatsResponseResponseTimes,
}

//		StatsResponseResponseCounts												
/// Counts of response status codes.
#[derive(Serialize, ToSchema)]
pub struct StatsResponseResponseCounts {
	//		Public properties													
	/// The total number of responses that have been handled.
	pub total:     u64,
	
	/// The number of responses that have been handled, by status code.
	#[serde(serialize_with = "serialize_status_codes")]
	pub codes:     HashMap<StatusCode, u64>,
	
	/// The number of untracked responses that have been handled, i.e. where the
	/// code does not match any of the ones in this struct.
	pub untracked: u64,
}

//		StatsResponseResponseTimes												
/// Response times in microseconds.
#[derive(Serialize, ToSchema)]
pub struct StatsResponseResponseTimes {
	//		Public properties													
	/// The response time of the current request.
	pub current: u64,
	
	/// Average since the application started.
	pub average: f64,
	
	/// Maximum since the application started.
	pub maximum: u64,
	
	/// Minimum since the application started.
	pub minimum: u64,
}



//		Functions

//		get_index																
/// Shows the index page.
/// 
/// # Parameters
/// 
/// * `state` - The application state.
/// 
pub async fn get_index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	get_page(State(state), Uri::from_static("/index.md")).await
}

//		get_page																
/// Shows a rendered Markdown page.
/// 
/// # Parameters
/// 
/// * `state` - The application state.
/// * `uri`   - The URI of the page to show.
/// 
pub async fn get_page(
	State(state): State<Arc<AppState>>,
	uri:          Uri,
) -> impl IntoResponse {
	let path       =  uri.path().trim_start_matches('/');
	if !path.ends_with(".md") {
		return get_protected_static_asset(State(state), uri).await.into_response();
	}
	let local_path =  state.Config.local_paths.markdown.join(path);
	let is_local   =  match state.Config.local_loading.markdown {
		LoadingBehavior::Deny       => false,
		LoadingBehavior::Supplement => CONTENT_DIR.get_file(path).is_none(),
		LoadingBehavior::Override   => local_path.exists(),
	};
	let text       =  if is_local {
		local_path.exists().then(|| fs::read_to_string(local_path).ok()).flatten()
	} else {
		CONTENT_DIR.get_file(path).and_then(|file| file.contents_utf8().map(|text| text.to_string()))
	};
	match text {
		None       => (StatusCode::NOT_FOUND).into_response(),
		Some(text) => {
			let (title, toc, html)     = if is_local {
				let (title, toc, html) = rustmark::parse(
					&text,
					//	Remove the title from the index page, as it will have one added showing
					//	the application title.
					path == "content/index.md",
				);
				(title, toc, html.to_string())
			} else {
				let (title, html)      = text.split_once('\n').unwrap();
				let (json,  html)      = html.split_once('\n').unwrap();
				let toc: Vec<Heading>  = serde_json::from_str(json).unwrap();
				(title.to_owned(), toc, html.to_owned())
			};
			let mut context = Context::new();
			let template    = if path == "index.md" { "index" } else { "page" };
			let title       = if path == "index.md" {
				state.Config.title.clone()
			} else {
				format!("{} - {}", title, &state.Config.title)
			};
			context.insert("Title",   &title);
			context.insert("ToC",     &toc);
			context.insert("Content", &html);
			(
				StatusCode::OK,
				render(state, template, context),
			).into_response()
		},
	}
}

//		get_protected_static_asset												
/// Serves protected static assets.
/// 
/// # Parameters
/// 
/// * `state` - The application state.
/// * `uri`   - The URI of the asset.
/// 
pub async fn get_protected_static_asset(
	State(state): State<Arc<AppState>>,
	uri:          Uri,
) -> impl IntoResponse {
	get_static_asset(state, uri, AssetContext::Protected).await
}

//		get_public_static_asset													
/// Serves public static assets.
/// 
/// # Parameters
/// 
/// * `state` - The application state.
/// * `uri`   - The URI of the asset.
/// 
pub async fn get_public_static_asset(
	State(state): State<Arc<AppState>>,
	uri:          Uri,
) -> impl IntoResponse {
	get_static_asset(state, uri, AssetContext::Public).await
}

//		get_static_asset														
/// Serves static assets.
/// 
/// # Parameters
/// 
/// * `state`   - The application state.
/// * `uri`     - The URI of the asset.
/// * `context` - The protection context of the asset to serve.
/// 
async fn get_static_asset(
	state:   Arc<AppState>,
	uri:     Uri,
	context: AssetContext
) -> impl IntoResponse {
	let path       =  uri.path().trim_start_matches('/');
	let mime_type  =  mime_guess::from_path(path).first_or_text_plain();
	let (basedir, local_path, behavior) = match context {
		AssetContext::Public    => (
			&ASSETS_DIR,
			state.Config.local_paths.public_assets.join(path),
			&state.Config.local_loading.public_assets
		),
		AssetContext::Protected => (
			&CONTENT_DIR,
			state.Config.local_paths.protected_assets.join(path),
			&state.Config.local_loading.protected_assets
		),
	};
	let is_local   =  match behavior {
		LoadingBehavior::Deny       => false,
		LoadingBehavior::Supplement => basedir.get_file(path).is_none(),
		LoadingBehavior::Override   => local_path.exists(),
	};
	if !(
			( is_local && local_path.exists())
		||	(!is_local && basedir.get_file(path).is_some())
	) {
		return Err((StatusCode::NOT_FOUND, ""));
	}
	let body = if is_local {
		let mut file   = File::open(local_path).await.ok().unwrap();
		let config     =  &state.Config.static_files;
		if file.metadata().await.unwrap().len() as usize > 1024 * config.stream_threshold {
			let reader = BufReader::with_capacity(1024 * config.read_buffer, file);
			let stream = ReaderStream::with_capacity(reader, 1024 * config.stream_buffer);
			Body::wrap_stream(stream)
		} else {
			let mut contents = vec![];
			file.read_to_end(&mut contents).await.unwrap();
			Body::from(contents)
		}
	} else {
		Body::from(basedir.get_file(path).unwrap().contents())
	};
	Ok(Response::builder()
		.status(StatusCode::OK)
		.header(
			header::CONTENT_TYPE,
			HeaderValue::from_str(mime_type.as_ref()).unwrap(),
		)
		.body(body)
		.unwrap()
	)
}

//		get_ping																
/// Availability check.
/// 
/// This endpoint is designed for use with uptime monitors. It simply returns
/// a 200 code and no content.
/// 
#[utoipa::path(
	get,
	path = "/api/ping",
	tag  = "health",
	responses(
		(status = 200, description = "Availability check")
	)
)]
pub async fn get_ping() {}

//		get_stats																
/// Produces various statistics about the service.
/// 
/// This endpoint returns a JSON object containing the following information:
/// 
///   - `started_at` - The date and time the application was started, in ISO
///                    8601 format.
///   - `uptime`     - The amount of time the application has been running, in
///                    seconds.
///   - `requests`   - The number of requests that have been handled.
///   - `responses`  - The counts and times of responses that have been handled.
///                    The total should match the number of requests, but is
///                    broken down by status code. The times are the average,
///                    maximum, and minimum response times since the application
///                    last started.
/// 
/// # Parameters
/// 
/// * `state`    - The application state.
/// * `stats_cx` - The statistics context.
/// 
#[utoipa::path(
	get,
	path = "/api/stats",
	tag  = "health",
	responses(
		(status = 200, description = "Application statistics", body = StatsResponse)
	)
)]
pub async fn get_stats(
	State(state):        State<Arc<AppState>>,
	Extension(stats_cx): Extension<StatsContext>,
) -> Json<StatsResponse> {
	//	Lock source data
	let lock      = state.Stats.responses.lock().expect("Failed to lock response stats");
	let response  = Json(StatsResponse {
		started_at: state.Stats.started_at,
		uptime:     (Utc::now().naive_utc() - state.Stats.started_at).num_seconds() as u64,
		requests:   state.Stats.requests.load(Ordering::Relaxed) as u64,
		responses:  StatsResponseResponses {
			counts: StatsResponseResponseCounts {
				total:       lock.counts.total,
				codes:       lock.counts.codes.clone(),
				untracked:   lock.counts.untracked,
			},
			times:  StatsResponseResponseTimes {
				current:     stats_cx.started_at.elapsed().as_micros() as u64,
				average:     lock.times.average,
				maximum:     lock.times.maximum,
				minimum:     lock.times.minimum,
			},
		},
	});
	//	Unlock source data
	drop(lock);
	response
}


