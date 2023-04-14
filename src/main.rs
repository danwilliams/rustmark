//		Modules

mod handlers;



//		Packages

use crate::{
	handlers::*,
};
use axum::{
	Router,
	routing::get,
};
use std::net::SocketAddr;



//		Functions

//		main																	
#[tokio::main]
async fn main() {
	let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
	let app  = Router::new()
		.route("/", get(get_index))
	;
	println!("Listening on {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap()
	;
}


