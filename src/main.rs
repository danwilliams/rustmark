//! Rustmark
//! 
//! Extensible web application for serving Markdown-based content.
//! 



//		Global configuration

//	Customisations of the standard linting configuration
#![allow(unreachable_pub,                 reason = "Not useful in binaries")]
#![allow(clippy::doc_markdown,            reason = "Too many false positives")]
#![allow(clippy::expect_used,             reason = "Acceptable in a binary crate")]
#![allow(clippy::multiple_crate_versions, reason = "Cannot resolve all these")]
#![allow(clippy::unwrap_used,             reason = "Somewhat acceptable in a binary crate")]



//		Modules

mod auth;
mod config;
mod handlers;
mod routes;
mod state;
mod utility;



//		Packages

use crate::{
	auth::User,
	config::Config,
	routes::{protected, public},
	state::AppState,
	utility::ApiDoc,
};
use std::sync::Arc;
use terracotta::{
	app::{
		create::{app_full as create_app, server as create_server},
		errors::AppError,
		init::{load_config, setup_logging},
		state::StateProvider,
	},
	stats::worker::start as start_stats_processor,
};
use tracing::info;
use utoipa::OpenApi;

#[cfg(not(windows))]
use tikv_jemallocator::Jemalloc;



//		Constants

/// The global allocator. This is changed to [`Jemalloc`] in order to obtain
/// memory usage statistics.
#[cfg(not(windows))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;



//		Functions

//		main																	
#[tokio::main]
async fn main() -> Result<(), AppError> {
	let config = load_config::<Config>()?;
	let _guard = setup_logging(&config.logdir);
	let state  = Arc::new(AppState::new(config));
	start_stats_processor(&state).await;
	let app    = create_app::<_, User, User>(&state, protected(), public(), ApiDoc::openapi());
	let server = create_server(app, &*state).await?;
	info!("Listening on {}", state.address().expect("Server address not set"));
	server.await.unwrap()
}


