//		Packages

use crate::utility::{AppState, Endpoint, ResponseMetrics};
use axum::{
	Extension,
	async_trait,
	extract::{FromRequestParts, State},
	http::{Request, request::Parts},
	middleware::Next,
	response::Response,
};
use chrono::{NaiveDateTime, Utc};
use smart_default::SmartDefault;
use std::sync::{Arc, atomic::Ordering};
use tikv_jemalloc_ctl::stats::allocated as Malloc;



//		Structs

//		StatsContext															
/// The statistics context.
/// 
/// This struct contains statistics information specific to the current request.
/// 
#[derive(Clone, SmartDefault)]
pub struct StatsContext {
	//		Public properties													
	/// The date and time the request processing started.
	#[default(Utc::now().naive_utc())]
	pub started_at: NaiveDateTime,
}

#[async_trait]
impl<State> FromRequestParts<State> for StatsContext
where State: Send + Sync {
	type Rejection = std::convert::Infallible;
	
	//		from_request_parts													
	/// Creates a statistics context from the request parts.
	/// 
	/// # Parameters
	/// 
	/// * `parts` - The request parts.
	/// * `state` - The application state.
	/// 
	async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
		let Extension(stats_cx): Extension<StatsContext> =
			Extension::from_request_parts(parts, state)
				.await
				.expect("Stats extension/layer missing")
		;
		Ok(stats_cx)
	}
}



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
	mut request:     Request<B>,
	next:            Next<B>,
) -> Response {
	//	Create statistics context
	let stats_cx    = StatsContext::default();
	request.extensions_mut().insert(stats_cx.clone());
	
	//	Check if statistics are enabled
	if !appstate.Config.stats.enabled {
		return next.run(request).await;
	}
	
	//	Obtain endpoint details
	let endpoint    = Endpoint {
		path:         request.uri().path().to_string(),
		method:       request.method().clone(),
	};
	
	//	Update requests counter
	appstate.Stats.requests.fetch_add(1, Ordering::Relaxed);
	appstate.Stats.connections.fetch_add(1, Ordering::Relaxed);
	
	//	Process request
	let response    = next.run(request).await;
	
	//	Add response time to the queue
	appstate.Queue.send(ResponseMetrics {
		endpoint,
		started_at:   stats_cx.started_at,
		time_taken:   (Utc::now().naive_utc() - stats_cx.started_at).num_microseconds().unwrap() as u64,
		status_code:  response.status(),
		connections:  appstate.Stats.connections.load(Ordering::Relaxed) as u64,
		memory:	      Malloc::read().unwrap() as u64,
	}).expect("Failed to send response time");
	
	appstate.Stats.connections.fetch_sub(1, Ordering::Relaxed);
	
	//	Return response
	response
}


