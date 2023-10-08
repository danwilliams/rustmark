//		Packages

use crate::utility::AppState;
use axum::{
	extract::State,
	http::{Request, StatusCode},
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
	let response = next.run(request).await;
	let counter  = match response.status() {
		StatusCode::OK                    => &appstate.Stats.responses.OK,
		StatusCode::UNAUTHORIZED          => &appstate.Stats.responses.UNAUTHORIZED,
		StatusCode::NOT_FOUND             => &appstate.Stats.responses.NOT_FOUND,
		StatusCode::INTERNAL_SERVER_ERROR => &appstate.Stats.responses.INTERNAL_SERVER_ERROR,
		_                                 => &appstate.Stats.responses.untracked,
	};
	counter.fetch_add(1, Ordering::Relaxed);
	appstate.Stats.responses.total.fetch_add(1, Ordering::Relaxed);
	response
}


