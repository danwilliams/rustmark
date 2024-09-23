//! Configuration for the application.



//		Packages

use core::net::IpAddr;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::collections::HashMap;
use terracotta::{
	app::config::HtmlTemplates,
	assets::config::Config as AssetsConfig,
	stats::config::Config as StatsConfig,
};



//		Structs

//		Config																	
/// The main configuration options for the application.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub struct Config {
	//		Public properties													
	/// The host to listen on.
	#[default(IpAddr::from([127, 0, 0, 1]))]
	pub host:   IpAddr,
	
	/// The port to listen on.
	#[default = 8000]
	pub port:   u16,
	
	/// The directory to store log files in.
	#[default = "log"]
	pub logdir: String,
	
	/// The title of the application.
	#[default = "Terracotta"]
	pub title:  String,
	
	/// Loading configuration for HTML templates.
	#[serde(rename = "html_templates")]
	pub html:   HtmlTemplates,
	
	/// The configuration options for serving static assets.
	pub assets: AssetsConfig,
	
	/// The configuration options for gathering and processing statistics.
	pub stats:  StatsConfig,
	
	/// A list of users and their passwords.
	#[default(HashMap::new())]
	pub users:  HashMap<String, String>,
}


