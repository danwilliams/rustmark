//! Configuration for the application.



//		Packages

use core::net::IpAddr;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::collections::HashMap;
use std::path::PathBuf;
use terracotta::{
	app::config::{HtmlTemplates, LoadingBehavior},
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
	pub host:     IpAddr,
	
	/// The port to listen on.
	#[default = 8000]
	pub port:     u16,
	
	/// The directory to store log files in.
	#[default = "log"]
	pub logdir:   String,
	
	/// The title of the application.
	#[default = "Rustmark"]
	pub title:    String,
	
	/// Loading configuration for HTML templates.
	#[serde(rename = "html_templates")]
	pub html:     HtmlTemplates,
	
	/// Loading configuration for Markdown content.
	pub markdown: MarkdownContent,
	
	/// The configuration options for serving static assets.
	pub assets:   AssetsConfig,
	
	/// The configuration options for gathering and processing statistics.
	pub stats:    StatsConfig,
	
	/// A list of users and their passwords.
	#[default(HashMap::new())]
	pub users:    HashMap<String, String>,
}

//		MarkdownContent															
/// Loading configuration for Markdown content.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub struct MarkdownContent {
	//		Public properties													
	/// The loading behaviour for local, non-baked-in Markdown content. This
	/// allows local Markdown content to be used to complement the baked-in
	/// templates.
	#[default(LoadingBehavior::Deny)]
	pub behavior:   LoadingBehavior,
	
	/// The path to the local, non-baked-in Markdown content.
	#[default = "html"]
	pub local_path: PathBuf,
}


