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
	let addr          = SocketAddr::from(([127, 0, 0, 1], config.port));
	let shared_state  = Arc::new(AppState {
		Config:         config,
	});
	let app           = Router::new()
		.route("/", get(get_index))
		.with_state(shared_state)
	;
	println!("Listening on {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap()
	;
}


