//		Packages

use crate::utility::AppState;
use axum::{
	extract::State,
	http::Request,
	middleware::Next,
	response::Response,
};
use std::sync::{Arc, atomic::Ordering};



//		Functions

//		stats_layer																
/// A middleware to collect statistics about requests and responses.
/// 
/// This middleware sits in the request-response chain and collects statistics
/// about requests and responses, storing them in the application state.
/// 
/// # Parameters
/// 
/// * `appstate` - The application state.
/// * `request`  - The request.
/// * `next`     - The next middleware.
/// 
pub async fn stats_layer<B>(
	State(appstate): State<Arc<AppState>>,
	request:         Request<B>,
	next:            Next<B>,
) -> Response {
	appstate.Stats.requests.fetch_add(1, Ordering::Relaxed);
	next.run(request).await
}


