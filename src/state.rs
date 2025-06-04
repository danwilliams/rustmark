//! Application state functionality.



//		Packages																										

use crate::config::Config;
use core::net::{IpAddr, SocketAddr};
use include_dir::{Dir, include_dir};
use parking_lot::RwLock;
use std::{
	collections::HashMap,
	sync::Arc,
};
use tera::{Context, Tera};
use terracotta::{
	app::{
		config::HtmlTemplates,
		errors::AppError,
		init::setup_tera,
		state::StateProvider as AppStateProvider,
		utility::render,
	},
	assets::{
		config::Config as AssetsConfig,
		state::StateProvider as AssetsStateProvider,
	},
	auth::state::StateProvider as AuthStateProvider,
	stats::{
		config::Config as StatsConfig,
		state::{State as StatsState, StateProvider as StatsStateProvider},
	},
};
use tokio::sync::RwLock as AsyncRwLock;



//		Structs																											

//		AppState																
/// The application state.
/// 
/// This is used to store global state information that is shared between
/// requests.
/// 
#[derive(Debug)]
pub struct AppState {
	//		Public properties													
	/// The address the server is running on.
	pub address:     RwLock<Option<SocketAddr>>,
	
	/// The directory containing the static assets.
	pub assets_dir:  Arc<Dir<'static>>,
	
	/// The application configuration.
	pub config:      Config,
	
	/// The directory containing the Markdown content.
	pub content_dir: Arc<Dir<'static>>,
	
	/// The application statistics.
	pub stats:       AsyncRwLock<StatsState>,
	
	/// The Tera template engine.
	pub tera:        Tera,
}

//󰭅		AppState																
impl AppState {
	//		new																	
	/// Creates a new application state.
	/// 
	/// # Parameters
	/// 
	/// * `config` - The application configuration.
	/// 
	/// # Returns
	/// 
	/// The new application state.
	/// 
	pub fn new(config: Config) -> Self {
		Self {
			config,
			..Default::default()
		}
	}
}

//󰭅		AppStateProvider														
impl AppStateProvider for AppState {
	//		address																
	fn address(&self) -> Option<SocketAddr> {
		*self.address.read()
	}
	
	//		host																
	fn host(&self) -> IpAddr {
		self.config.host
	}
	
	//		html_templates_config												
	fn html_templates_config(&self) -> &HtmlTemplates {
		&self.config.html
	}
	
	//		port																
	fn port(&self) -> u16 {
		self.config.port
	}
	
	//		render																
	async fn render<T: AsRef<str> + Send>(&self, template: T, context: &Context) -> Result<String, AppError> {
		render(self, template.as_ref(), context).await
	}
	
	//		set_address															
	fn set_address(&self, address: Option<SocketAddr>) {
		*self.address.write() = address;
	}
	
	//		tera																
	fn tera(&self) -> &Tera {
		&self.tera
	}
	
	//		title																
	fn title(&self) -> &String {
		&self.config.title
	}
}

//󰭅		AssetsStateProvider														
impl AssetsStateProvider for AppState {
	//		config																
	fn config(&self) -> &AssetsConfig {
		&self.config.assets
	}
	
	//		assets_dir															
	fn assets_dir(&self) -> Arc<Dir<'static>> {
		Arc::clone(&self.assets_dir)
	}
	
	//		content_dir															
	fn content_dir(&self) -> Arc<Dir<'static>> {
		Arc::clone(&self.content_dir)
	}
}

//󰭅		AuthStateProvider														
impl AuthStateProvider for AppState {
	//		users																
	fn users(&self) -> &HashMap<String, String> {
		&self.config.users
	}
}

//󰭅		Default																	
impl Default for AppState {
	//		default																
	fn default() -> Self {
		Self {
			address:     RwLock::new(None),
			assets_dir:  Arc::new(include_dir!("static")),
			config:      Config::default(),
			content_dir: Arc::new(include_dir!("$OUT_DIR")),
			stats:       AsyncRwLock::new(StatsState::default()),
			tera:        setup_tera(&Arc::new(include_dir!("html")))
				.expect("Error loading templates")
			,
		}
	}
}

//󰭅		StatsStateProvider														
impl StatsStateProvider for AppState {
	//		config																
	fn config(&self) -> &StatsConfig {
		&self.config.stats
	}
	
	//		state																
	fn state(&self) -> &AsyncRwLock<StatsState> {
		&self.stats
	}
}


