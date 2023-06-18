#![allow(non_snake_case)]

//		Packages

use axum::http::Uri;
use ring::hmac;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use tera::Tera;
use url::form_urlencoded;



//		Structs

//		Config																	
/// The main configuration options for the application.
#[derive(Deserialize, Serialize, SmartDefault)]
pub struct Config {
	/// The port to listen on.
	#[default = 8000]
	pub port:   u16,
	/// The directory to store log files in.
	#[default = "log"]
	pub logdir: String,
	/// The title of the application.
	#[default = "Rustmark"]
	pub title:  String,
	/// A list of users and their passwords.
	#[default(HashMap::new())]
	pub users:  HashMap<String, String>,
}

//		AppState																
/// The application state.
/// 
/// This is used to store global state information that is shared between
/// requests.
#[allow(dead_code)]
pub struct AppState {
	/// The application configuration.
	pub Config:   Config,
	/// The application secret.
	pub Secret:   [u8; 64],
	/// The HMAC key used to sign and verify sessions.
	pub Key:      hmac::Key,
	/// The Tera template engine.
	pub Template: Tera,
}



//		Functions

//		extract_uri_query_parts													
/// Extracts the query parts from a URI.
/// 
/// Extracts the query parts of a URI and returns them as a HashMap.
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


