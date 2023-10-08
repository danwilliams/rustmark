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
	//		Preparation															
	//	Update requests counter
	appstate.Stats.requests.fetch_add(1, Ordering::Relaxed);
	
	//		Request																
	//	Process request
	let response               = next.run(request).await;
	
	//		Metrics																
	//	Update responses counter
	let status_code            = response.status();
	if let Some(counter)       = appstate.Stats.responses.counts.codes.get(&status_code) {
		counter.fetch_add(1, Ordering::Relaxed);
	} else {
		appstate.Stats.responses.counts.untracked.fetch_add(1, Ordering::Relaxed);
	}
	appstate.Stats.responses.counts.total.fetch_add(1, Ordering::Relaxed);
	
	//		Response															
	response
}


