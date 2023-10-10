#![allow(non_snake_case)]

//		Packages

use crate::handlers;
use axum::{
	http::{StatusCode, Uri},
	response::Html,
};
use chrono::NaiveDateTime;
use flume::{Receiver, Sender};
use parking_lot::Mutex;
use ring::hmac;
use serde::{Deserialize, Serialize, Serializer};
use smart_default::SmartDefault;
use std::{
	collections::{BTreeMap, HashMap},
	fs,
	net::IpAddr,
	path::PathBuf,
	sync::{Arc, atomic::AtomicUsize},
	thread::spawn,
	time::Instant,
};
use tera::{Context, Tera};
use tracing::info;
use url::form_urlencoded;
use utoipa::OpenApi;
use velcro::hash_map;



//		Enums

//		LoadingBehavior															
/// The possible options for loading local, non-baked-in resources.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum LoadingBehavior {
	/// Deny loading of local resources.
	Deny,
	
	/// Load local resources if the baked-in resources are not present.
	Supplement,
	
	/// Load local resources if they exist, otherwise load baked-in resources.
	Override,
}



//		Structs

//		Config																	
/// The main configuration options for the application.
#[derive(Deserialize, Serialize, SmartDefault)]
pub struct Config {
	//		Public properties													
	/// The host to listen on.
	#[default(IpAddr::from([127, 0, 0, 1]))]
	pub host:          IpAddr,
	
	/// The port to listen on.
	#[default = 8000]
	pub port:          u16,
	
	/// The directory to store log files in.
	#[default = "log"]
	pub logdir:        String,
	
	/// The title of the application.
	#[default = "Rustmark"]
	pub title:         String,
	
	/// The loading behaviour for local, non-baked-in resources. This allows
	/// local resources to be used to complement the baked-in resources.
	pub local_loading: LocalLoading,
	
	/// The paths for local, non-baked-in resources.
	pub local_paths:   LocalPaths,
	
	/// The configuration options for serving static files.
	pub static_files:  StaticFiles,
	
	/// A list of users and their passwords.
	#[default(HashMap::new())]
	pub users:         HashMap<String, String>,
}

//		LocalLoading															
/// The loading behaviour for local, non-baked-in resources.
#[derive(Deserialize, Serialize, SmartDefault)]
pub struct LocalLoading {
	//		Public properties													
	/// The loading behaviour for HTML templates.
	#[default(LoadingBehavior::Deny)]
	pub html:             LoadingBehavior,
	
	/// The loading behaviour for Markdown content.
	#[default(LoadingBehavior::Deny)]
	pub markdown:         LoadingBehavior,
	
	/// The loading behaviour for protected static assets.
	#[default(LoadingBehavior::Deny)]
	pub protected_assets: LoadingBehavior,
	
	/// The loading behaviour for public static assets.
	#[default(LoadingBehavior::Deny)]
	pub public_assets:    LoadingBehavior,
}

//		LocalPaths																
/// The local paths for non-baked-in resources.
#[derive(Deserialize, Serialize, SmartDefault)]
pub struct LocalPaths {
	//		Public properties													
	/// The path to the HTML templates.
	#[default = "html"]
	pub html:             PathBuf,
	
	/// The path to the Markdown content.
	#[default = "content"]
	pub markdown:         PathBuf,
	
	/// The path to the protected static assets.
	#[default = "content"]
	pub protected_assets: PathBuf,
	
	/// The path to the public static assets.
	#[default = "static"]
	pub public_assets:    PathBuf,
}

//		StaticFiles																
#[derive(Deserialize, Serialize, SmartDefault)]
/// The configuration options for serving static files.
pub struct StaticFiles {
	//		Public properties													
	/// The file size at which to start streaming, in KB. Below this size, the
	/// file will be read into memory and served all at once.
	#[default = 1000]
	pub stream_threshold: usize,
	
	/// The size of the stream buffer to use when streaming files, in KB.
	#[default = 256]
	pub stream_buffer:    usize,
	
	/// The size of the read buffer to use when streaming files, in KB.
	#[default = 128]
	pub read_buffer:      usize,
}

//		AppState																
/// The application state.
/// 
/// This is used to store global state information that is shared between
/// requests.
/// 
#[allow(dead_code)]
pub struct AppState {
	//		Public properties													
	/// The application configuration.
	pub Config:   Config,
	
	/// The application statistics.
	pub Stats:    AppStats,
	
	/// The statistics queue that response times are added to. This is the
	/// sender side only. A queue is used so that each request-handling thread's
	/// stats middleware can send its metrics into the queue instead of updating
	/// a central, locked data structure. This avoids the need for locking and
	/// incineration routines, as the stats-handling thread can constantly
	/// process the queue and there will theoretically never be a large build-up
	/// of data in memory that has to be dealt with all at once.
	pub Queue:    Sender<ResponseTime>,
	
	/// The application secret.
	pub Secret:   [u8; 64],
	
	/// The HMAC key used to sign and verify sessions.
	pub Key:      hmac::Key,
	
	/// The Tera template engine.
	pub Template: Tera,
}

//		AppStats																
/// Various application statistics.
#[derive(SmartDefault)]
pub struct AppStats {
	//		Public properties													
	/// The date and time the application was started.
	pub started_at: NaiveDateTime,
	
	/// The number of requests that have been handled.
	pub requests:   AtomicUsize,
	
	/// The number of responses that have been handled, along with the average,
	/// maximum, and minimum response times by time period. This data is grouped
	/// together inside a [`Mutex`] because it is important to update the count,
	/// use that exact count to calculate the average, and then store that
	/// average all in one atomic operation while blocking any other process
	/// from using the data. A [`parking_lot::Mutex`] is used instead of a
	/// [`std::sync::Mutex`] because it is theoretically faster in highly
	/// contended situations, but the main advantage is that it is infallible,
	/// and it does not have mutex poisoning.
	pub responses:  Mutex<AppStatsResponses>,
}

//		AppStatsResponses														
/// Counts and times of responses.
#[derive(SmartDefault)]
pub struct AppStatsResponses {
	//		Public properties													
	/// The counts of responses.
	#[default(AppStatsResponseCounts::new())]
	pub counts: AppStatsResponseCounts,
	
	/// The times of responses.
	pub times:  AppStatsResponseTimes,
}

//		AppStatsResponseCounts													
/// Counts of response status codes.
#[derive(SmartDefault)]
pub struct AppStatsResponseCounts {
	//		Public properties													
	/// The total number of responses that have been handled.
	pub total:     u64,
	
	/// The number of responses that have been handled, by status code.
	pub codes:     HashMap<StatusCode, u64>,
	
	/// The number of untracked responses that have been handled, i.e. where the
	/// code does not match any of the ones in this struct.
	pub untracked: u64,
}

impl AppStatsResponseCounts {
	//		new																	
	/// Creates a new instance of the struct.
	pub fn new() -> Self {
		Self {
			total:     0,
			codes:     hash_map!{
				StatusCode::OK:                    0,
				StatusCode::UNAUTHORIZED:          0,
				StatusCode::NOT_FOUND:             0,
				StatusCode::INTERNAL_SERVER_ERROR: 0,
			},
			untracked: 0,
		}
	}
}

//		AppStatsResponseTimes													
/// Response times in microseconds.
#[derive(SmartDefault)]
pub struct AppStatsResponseTimes {
	//		Public properties													
	/// The average, maximum, and minimum response times for the past minute.
	pub minute: AppStatsForPeriod,
	
	/// The average, maximum, and minimum response times for the past hour.
	pub hour:   AppStatsForPeriod,
	
	/// The average, maximum, and minimum response times for the past day.
	pub day:    AppStatsForPeriod,
	
	/// The average, maximum, and minimum response times since the application
	/// last started.
	pub all:    AppStatsForPeriod,
}

//		AppStatsForPeriod														
/// Average, maximum, and minimum values for a period of time.
#[derive(SmartDefault)]
pub struct AppStatsForPeriod {
	//		Public properties													
	/// The date and time the period started.
	#[default(Instant::now())]
	pub started_at: Instant,
	
	/// Average response time in microseconds.
	pub average:    f64,
	
	/// Maximum response time in microseconds.
	pub maximum:    u64,
	
	/// Minimum response time in microseconds.
	pub minimum:    u64,
}

//		ResponseTime															
/// Metrics for a single response.
/// 
/// This is used by the statistics queue in [`AppState.Queue`].
/// 
#[derive(SmartDefault)]
pub struct ResponseTime {
	//		Public properties													
	/// The date and time the request started.
	#[default(Instant::now())]
	pub started_at: Instant,
	
	/// The time the response took to be generated.
	pub time_taken: u64,
}

//		ApiDoc																	
/// The OpenAPI documentation for the API.
#[derive(OpenApi)]
#[openapi(
	paths(
		handlers::get_ping,
		handlers::get_stats,
	),
	components(
		schemas(handlers::StatsResponse),
	),
	tags(
		(name = "health", description = "Health check endpoints"),
	)
)]
pub struct ApiDoc;



//		Functions

//		extract_uri_query_parts													
/// Extracts the query parts from a URI.
/// 
/// Extracts the query parts of a [`Uri`] and returns them as a [`HashMap`].
/// 
/// # Parameters
/// 
/// * `uri` - The URI to extract the query parts from.
/// 
pub fn extract_uri_query_parts(uri: Uri) -> HashMap<String, String> {
	uri
		.query()
		.map(|v| {
			form_urlencoded::parse(v.as_bytes())
				.into_owned()
				.collect()
		})
		.unwrap_or_else(HashMap::new)
}

//		build_uri																
/// Builds a URI from a path and a set of query parameters.
/// 
/// # Parameters
/// 
/// * `path`   - The path to build the URI from.
/// * `params` - The query parameters to add to the URI.
/// 
pub fn build_uri(path: String, params: HashMap<String, String>) -> Uri {
	Uri::builder()
		.path_and_query(format!("{}?{}",
			path,
			params
				.iter()
				.map(|(k, v)| format!("{}={}", k, v))
				.collect::<Vec<String>>()
				.join("&")
		))
		.build()
		.unwrap()
}

//		render																	
/// Renders a template.
/// 
/// Renders a template with the given context and returns the result.
/// 
/// If the application has been configured to allow template overrides, the
/// local filesystem will be searched, and any matching templates found will be
/// used in preference to the baked-in ones.
/// 
/// # Parameters
/// 
/// * `state`    - The application state.
/// * `template` - The name of the template to render.
/// * `context`  - The context to render the template with.
/// 
pub fn render(
	state:    Arc<AppState>,
	template: &str,
	context:  Context,
) -> Html<String> {
	let local_template = state.Config.local_paths.html.join(format!("{}.tera.html", template));
	let local_layout   = state.Config.local_paths.html.join("layout.tera.html");
	let mut tera       = state.Template.clone();
	if state.Config.local_loading.html == LoadingBehavior::Override {
		if local_layout.exists() {
			tera.add_raw_template("layout", &fs::read_to_string(local_layout).ok().unwrap()).unwrap();
		};
		if local_template.exists() {
			tera.add_raw_template(template, &fs::read_to_string(local_template).ok().unwrap()).unwrap();
		};
	};
	Html(tera.render(template, &context).unwrap())
}

//		serialize_status_codes													
/// Returns a list of serialised status code entries and their values.
/// 
/// This function is used by [`serde`] to serialise a list of status codes and
/// their associated values. It returns the list sorted by status code.
/// 
/// # Parameters
/// 
/// * `status_codes` - The status codes to serialise, as keys, against values.
/// * `serializer`   - The serialiser to use.
/// 
pub fn serialize_status_codes<S>(status_codes: &HashMap<StatusCode, u64>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let codes: BTreeMap<String, u64> = status_codes
		.iter()
		.map(|(key, value)| (key.to_string(), *value))
		.collect()
	;
	codes.serialize(serializer)
}

//		start_stats_processor													
/// Starts the statistics processor.
/// 
/// This function starts a thread that will process the statistics queue in
/// [`AppState.Queue`]. It will run until the channel is disconnected.
/// 
/// The processing of the statistics is done in a separate thread so that the
/// request-handling threads can continue to handle requests without being
/// blocked by the statistics processing. This way, none of them are ever
/// affected more than others. The stats-handling thread blocks on the queue, so
/// it will only process a response time when one is available.
/// 
/// # Parameters
/// 
/// * `receiver` - The receiving end of the queue.
/// 
pub fn start_stats_processor(receiver: Receiver<ResponseTime>) {
	//	Queue processing loop
	spawn(move || loop {
		//	Wait for message - this is a blocking call
		match receiver.recv() {
			Ok(response_time) => {
				info!("Response time: {}µs", response_time.time_taken);
			},
			Err(_)            => {
				eprintln!("Channel has been disconnected, exiting thread.");
				break;
			},
		}
	});
}


