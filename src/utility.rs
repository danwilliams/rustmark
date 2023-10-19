#![allow(non_snake_case)]

//		Packages

use crate::{
	handlers,
	stats::{AppStats, ResponseMetrics, AllStatsForPeriod, self},
};
use axum::{
	http::{Uri, Method},
	response::Html,
};
use flume::Sender;
use ring::hmac;
use serde::{Deserialize, Serialize, Serializer};
use smart_default::SmartDefault;
use std::{
	collections::HashMap,
	fs,
	net::IpAddr,
	path::PathBuf,
	sync::Arc,
};
use tera::{Context, Tera};
use tokio::sync::broadcast::Sender as Broadcaster;
use url::form_urlencoded;
use utoipa::OpenApi;



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
	
	/// The configuration options for gathering and processing statistics.
	pub stats:         StatsOptions,
	
	/// The time periods to report statistics for. These will default to second,
	/// minute, hour, and day, and refer to the last such period of time from
	/// the current time, measured back from the start of the current second.
	/// They will be used to calculate the average, maximum, and minimum values
	/// for each period, and the number of values in each period. In addition,
	/// the statistics since the application started will always be reported.
	/// Note that any defaults specified here would be augmented by items added
	/// to config, and not replaced by them, so the desired periods NEED to be
	/// placed in the application config file. If omitted, there will be no
	/// registered periods.
	pub stats_periods: HashMap<String, usize>,
	
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

//		StatsOptions															
#[derive(Deserialize, Serialize, SmartDefault)]
/// The configuration options for gathering and processing statistics.
pub struct StatsOptions {
	//		Public properties													
	/// Whether to enable statistics gathering and processing. If enabled, there
	/// is a very small CPU overhead for each request, plus an
	/// individually-configurable amount of memory used to store the
	/// [response time buffer](StatsOptions.timing_buffer_size), the
	/// [connection count buffer](StatsOptions.connection_buffer_size), and the
	/// [memory usage buffer](StatsOptions.memory_buffer_size) (default 4.8MB
	/// per buffer). If disabled, the [statistics processing thread](stats::start_stats_processor())
	/// will not be started, the buffers' capacities will not be reserved, and
	/// the [statistics middleware](stats::stats_layer()) will do nothing.
	/// Under usual circumstances the statistics thread should easily be able to
	/// keep up with the incoming requests, even on a system with hundreds of
	/// CPU cores.
	#[default = true]
	pub enabled:                bool,
	
	/// The size of the buffer to use for storing response times, in seconds.
	/// Each entry (i.e. for one second) will take up 56 bytes, so the default
	/// of 86,400 seconds (one day) will take up around 4.8MB of memory. This
	/// seems like a reasonable default to be useful but not consume too much
	/// memory. Notably, the statistics output only looks at a maximum of the
	/// last day's-worth of data, so if a longer period than this is required
	/// the [`get_stats()`](stats::get_stats()) code would need to be
	/// customised.
	#[default = 86_400]
	pub timing_buffer_size:     usize,
	
	/// The size of the buffer to use for storing connection data, in seconds.
	/// Each entry (i.e. for one second) will take up 56 bytes, so the default
	/// of 86,400 seconds (one day) will take up around 4.8MB of memory. This
	/// seems like a reasonable default to be useful but not consume too much
	/// memory. Notably, the statistics output only looks at a maximum of the
	/// last day's-worth of data, so if a longer period than this is required
	/// the [`get_stats()`](stats::get_stats()) code would need to be
	/// customised.
	#[default = 86_400]
	pub connection_buffer_size: usize,
	
	/// The size of the buffer to use for storing memory usage data, in seconds.
	/// Each entry (i.e. for one second) will take up 56 bytes, so the default
	/// of 86,400 seconds (one day) will take up around 4.8MB of memory. This
	/// seems like a reasonable default to be useful but not consume too much
	/// memory. Notably, the statistics output only looks at a maximum of the
	/// last day's-worth of data, so if a longer period than this is required
	/// the [`get_stats()`](stats::get_stats()) code would need to be
	/// customised.
	#[default = 86_400]
	pub memory_buffer_size:     usize,
	
	/// The interval at which to send ping messages to WebSocket clients, in
	/// seconds. This is used to check the connection is still alive.
	#[default = 60]
	pub ws_ping_interval:       usize,
	
	/// The timeout for WebSocket ping messages, in seconds. If a pong message
	/// is not received in reply to the outgoing ping message within this time,
	/// the connection will be closed.
	#[default = 10]
	pub ws_ping_timeout:        usize,
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
	pub Queue:    Sender<ResponseMetrics>,
	
	/// The statistics broadcast channel that period-based statistics are added
	/// to. This is the receiver side only. Each interested party can subscribe
	/// to this channel to receive the latest statistics for a given period on
	/// a real-time basis.
	pub Broadcast: Broadcaster<AllStatsForPeriod>,
	
	/// The application secret.
	pub Secret:   [u8; 64],
	
	/// The HMAC key used to sign and verify sessions.
	pub Key:      hmac::Key,
	
	/// The Tera template engine.
	pub Template: Tera,
}

//		Endpoint																
/// A formalised definition of an endpoint for identification.
#[derive(Clone, Eq, Hash, PartialEq, SmartDefault)]
pub struct Endpoint {
	//		Public properties													
	/// The path of the endpoint, minus any query parameters. As this is just
	/// the path, it does not contain scheme or authority (host), and hence is
	/// not a full URI.
	pub path:   String,
	
	/// The HTTP verb of the endpoint.
	pub method: Method,
}

impl Serialize for Endpoint {
	//		serialize															
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&format!("{} {}", self.method, self.path))
	}
}

//		ApiDoc																	
/// The OpenAPI documentation for the API.
#[derive(OpenApi)]
#[openapi(
	paths(
		handlers::get_ping,
		stats::get_stats,
		stats::get_stats_history,
		stats::get_stats_feed,
	),
	components(
		schemas(
			stats::StatsResponse,
			stats::StatsResponseForPeriod,
			stats::StatsHistoryResponse,
		),
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


