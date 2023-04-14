//		Modules

mod handlers;
mod utility;



//		Packages

use crate::{
	handlers::*,
	utility::*,
};
use axum::{
	Router,
	routing::get,
};
use figment::{
	Figment,
	providers::{Env, Format, Toml},
};
use std::{
	net::SocketAddr,
	sync::Arc,
	time::Duration,
};
use tracing::{Level, Span, info};
use tracing_appender::{self};
use tracing_subscriber::{
	fmt::writer::MakeWriterExt,
	layer::SubscriberExt,
	util::SubscriberInitExt,
};



//		Functions

//		main																	
#[tokio::main]
async fn main() {
	let config: Config = Figment::new()
		.merge(Toml::file("Config.toml"))
		.merge(Env::raw())
		.extract()
		.expect("Error loading config")
	;
	let (non_blocking_appender, _guard) = tracing_appender::non_blocking(
		tracing_appender::rolling::daily(&config.logdir, "general.log")
	);
	tracing_subscriber::registry()
		.with(
			tracing_subscriber::EnvFilter::try_from_default_env()
				.unwrap_or_else(|_| "terracotta=debug,tower_http=debug".into()),
		)
		.with(
			tracing_subscriber::fmt::layer()
				.with_writer(std::io::stdout.with_max_level(Level::DEBUG))
		)
		.with(
			tracing_subscriber::fmt::layer()
				.with_writer(non_blocking_appender.with_max_level(Level::INFO))
		)
		.init()
	;
	let addr          = SocketAddr::from(([127, 0, 0, 1], config.port));
	let shared_state  = Arc::new(AppState {
		Config:         config,
	});
	let app           = Router::new()
		.route("/", get(get_index))
		.with_state(shared_state)
		.layer(tower_http::trace::TraceLayer::new_for_http()
			.on_request(
				tower_http::trace::DefaultOnRequest::new()
					.level(Level::INFO)
			)
			.on_response(
				tower_http::trace::DefaultOnResponse::new()
					.level(Level::INFO)
					.latency_unit(tower_http::LatencyUnit::Micros)
			)
			.on_body_chunk(|chunk: &bytes::Bytes, _latency: Duration, _span: &Span| {
				tracing::debug!("Sending {} bytes", chunk.len())
			})
			.on_eos(|_trailers: Option<&axum::http::HeaderMap>, stream_duration: Duration, _span: &Span| {
				tracing::debug!("Stream closed after {:?}", stream_duration)
			})
			.on_failure(|_error: tower_http::classify::ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
				tracing::error!("Something went wrong")
			})
		)
	;
	info!("Listening on {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap()
	;
}


