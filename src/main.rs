//		Modules

mod auth;
mod errors;
mod handlers;
mod utility;



//		Packages

use crate::{
	auth::*,
	errors::*,
	handlers::*,
	utility::*,
};
use axum::{
	Router,
	middleware,
	routing::{get, post},
};
use axum_sessions::{
	SessionLayer,
	async_session::MemoryStore as SessionMemoryStore,
};
use figment::{
	Figment,
	providers::{Env, Format, Serialized, Toml},
};
use include_dir::{Dir, include_dir};
use rand::Rng;
use ring::hmac::{self, HMAC_SHA512};
use std::{
	net::SocketAddr,
	sync::Arc,
	time::Duration,
};
use tera::Tera;
use tower_http::catch_panic::CatchPanicLayer;
use tracing::{Level, Span, info};
use tracing_appender::{self};
use tracing_subscriber::{
	fmt::writer::MakeWriterExt,
	layer::SubscriberExt,
	util::SubscriberInitExt,
};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;



//		Constants

static TEMPLATE_DIR: Dir<'_> = include_dir!("html");
static ASSETS_DIR:   Dir<'_> = include_dir!("static");
static CONTENT_DIR:  Dir<'_> = include_dir!("$OUT_DIR");



//		Functions

//		main																	
#[tokio::main]
async fn main() {
	let config: Config = Figment::from(Serialized::defaults(Config::default()))
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
				.unwrap_or_else(|_| "rustmark=debug,tower_http=debug".into()),
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
	let addr          = SocketAddr::from((config.host, config.port));
	let mut templates = vec![];
	for file in TEMPLATE_DIR.find("**/*.tera.html").expect("Failed to read glob pattern") {
		templates.push((
			file.path().file_name().unwrap()
				.to_str().unwrap()
				.strip_suffix(".tera.html").unwrap()
				.to_owned(),
			TEMPLATE_DIR.get_file(file.path()).unwrap().contents_utf8().unwrap(),
		));
	}
	let mut tera      = Tera::default();
	tera.add_raw_templates(templates).expect("Error parsing templates");
	tera.autoescape_on(vec![".tera.html", ".html"]);
	let secret        = rand::thread_rng().gen::<[u8; 64]>();
	let session_store = SessionMemoryStore::new();
	let shared_state  = Arc::new(AppState {
		Config:         config,
		Secret:         secret,
		Key:            hmac::Key::new(HMAC_SHA512, &secret),
		Template:       tera,
	});
	//	Protected routes
	let app           = Router::new()
		.route("/",      get(get_index))
		.route("/*path", get(get_page))   //  Also handles get_protected_static_asset(uri)
		.route_layer(middleware::from_fn_with_state(Arc::clone(&shared_state), protect))
		.merge(
			//	Public routes
			Router::new()
				.route("/login",          post(post_login))
				.route("/logout",         get(get_logout))
				.route("/css/*path",      get(get_public_static_asset))
				.route("/img/*path",      get(get_public_static_asset))
				.route("/js/*path",       get(get_public_static_asset))
				.route("/webfonts/*path", get(get_public_static_asset))
		)
		.merge(SwaggerUi::new("/api-docs/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
		.merge(Redoc::with_url("/api-docs/redoc", ApiDoc::openapi()))
		.merge(RapiDoc::new("/api-docs/openapi.json").path("/api-docs/rapidoc"))
		.fallback(no_route)
		.layer(CatchPanicLayer::new())
		.layer(middleware::from_fn_with_state(Arc::clone(&shared_state), graceful_error_layer))
		.layer(middleware::from_fn_with_state(Arc::clone(&shared_state), auth_layer))
		.layer(SessionLayer::new(session_store, &secret).with_secure(false))
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
		.layer(CatchPanicLayer::new())
		.layer(middleware::from_fn(final_error_layer))
	;
	info!("Listening on {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap()
	;
}


