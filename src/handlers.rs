//! Endpoint handlers for the application.



//		Packages

use crate::state::AppState;
use axum::{
	extract::State,
	http::{StatusCode, Uri},
	response::{Html, IntoResponse},
};
use rustmark::{Heading, self};
use std::{
	fs,
	sync::Arc,
};
use terracotta::{
	app::config::LoadingBehavior,
	assets::handlers::get_protected_static_asset,
};
use tera::Context;



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
	#[expect(clippy::case_sensitive_file_extension_comparisons, reason = "The extension should always be lowercase")]
	if !path.ends_with(".md") {
		return get_protected_static_asset(State(state), uri).await.into_response();
	}
	let local_path =  state.config.markdown.local_path.join(path);
	let is_local   =  match state.config.markdown.behavior {
		LoadingBehavior::Deny       => false,
		LoadingBehavior::Supplement => state.content_dir.get_file(path).is_none(),
		LoadingBehavior::Override   => local_path.exists(),
	};
	let possible_text = if is_local {
		local_path.exists().then(|| fs::read_to_string(local_path).ok()).flatten()
	} else {
		state.content_dir.get_file(path).and_then(|file| file.contents_utf8().map(ToOwned::to_owned))
	};
	possible_text.map_or_else(|| (StatusCode::NOT_FOUND).into_response(), |text| {
		let (title, toc, html)     = if is_local {
			let (title, toc, html) = rustmark::parse(
				&text,
				//	Remove the title from the index page, as it will have one added showing
				//	the application title.
				path == "content/index.md",
			);
			(title, toc, html.to_string())
		} else {
			let mut split = text.splitn(3, '\n');
			let title     = split.next().unwrap_or_default();
			let json      = split.next().unwrap_or_default();
			let html      = split.next().unwrap_or_default();
			let toc: Vec<Heading> = serde_json::from_str(json).unwrap();
			(title.to_owned(), toc, html.to_owned())
		};
		let mut context = Context::new();
		let template    = if path == "index.md" { "index" } else { "page" };
		let page_title  = if path == "index.md" {
			state.config.title.clone()
		} else {
			format!("{title} - {}", &state.config.title)
		};
		context.insert("Title",   &page_title);
		context.insert("ToC",     &toc);
		context.insert("Content", &html);
		(
			StatusCode::OK,
			Html(state.tera.render(template, &context).unwrap()),
		).into_response()
	})
}


