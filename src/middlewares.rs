//		Packages

use crate::utility::{AppState, ResponseTime};
use axum::{
	Extension,
	async_trait,
	extract::{FromRequestParts, State},
	http::{Request, request::Parts},
	middleware::Next,
	response::Response,
};
use chrono::{NaiveDateTime, Utc};
use std::sync::{Arc, atomic::Ordering};



//		Structs

//		StatsContext															
/// The statistics context.
/// 
/// This struct contains statistics information specific to the current request.
/// 
#[derive(Clone)]
pub struct StatsContext {
	//		Public properties													
	/// The date and time the request processing started.
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
	//		Preparation															
	//	Update requests counter
	appstate.Stats.requests.fetch_add(1, Ordering::Relaxed);
	
	//	Note start time
	let started_at             = Utc::now().naive_utc();
	
	//	Create statistics context
	let stats_cx               = StatsContext {
		started_at,
	};
	request.extensions_mut().insert(stats_cx.clone());
	
	//		Request																
	//	Process request
	let response               = next.run(request).await;
	
	//		Metrics																
	//	Lock response data
	let mut lock               = appstate.Stats.responses.lock();
	
	//	Update responses counter
	let status_code            = response.status();
	if let Some(counter)       = lock.counts.codes.get_mut(&status_code) {
		*counter              += 1;
	} else {
		lock.counts.untracked += 1;
	}
	lock.counts.total         += 1;
	
	//	Update response time stats
	let finished_at            = Utc::now().naive_utc();
	let time_taken             = (finished_at - stats_cx.started_at).num_microseconds().unwrap() as u64;
	let alpha                  = 1.0 / lock.counts.total as f64;
	lock.times.average         = lock.times.average * (1.0 - alpha) + time_taken as f64 * alpha;
	lock.times.count          += 1;
	if time_taken < lock.times.minimum || lock.times.count == 1 {
		lock.times.minimum     = time_taken;
	}
	if time_taken > lock.times.maximum {
		lock.times.maximum     = time_taken;
	}
	
	//	Unlock response data
	drop(lock);
	
	//	Add response time to the queue
	appstate.Queue.send(ResponseTime { started_at, time_taken }).expect("Failed to send response time");
	
	//		Response															
	response
}


