#![allow(non_snake_case)]

//		Modules

#[cfg(test)]
#[path = "tests/stats.rs"]
mod tests;



//		Packages

use crate::utility::{AppState, Endpoint};
use axum::{
	Extension,
	Json,
	async_trait,
	extract::{FromRequestParts, Query, State},
	extract::ws::{Message, WebSocketUpgrade, WebSocket},
	http::{Request, StatusCode, request::Parts},
	middleware::Next,
	response::{Response},
};
use chrono::{Duration, NaiveDateTime, Timelike, Utc};
use flume::{Receiver, Sender};
use indexmap::IndexMap;
use itertools::Itertools;
use parking_lot::{Mutex, RwLock};
use rubedo::{
	std::IteratorExt,
	sugar::s,
};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;
use smart_default::SmartDefault;
use std::{
	collections::{BTreeMap, HashMap, VecDeque},
	str::FromStr,
	sync::{Arc, atomic::AtomicUsize, atomic::Ordering},
	time::Instant,
};
use tikv_jemalloc_ctl::stats::allocated as Malloc;
use tokio::{
	select,
	spawn,
	sync::broadcast::Sender as Broadcaster,
	time::{interval, sleep},
};
use tracing::{info, warn};
use utoipa::{IntoParams, ToSchema};
use velcro::{btree_map, hash_map};



//		Enums

//		MeasurementType															
/// The type of measurement to get statistics for.
#[derive(Copy, Clone, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum MeasurementType {
	/// Response times.
	Times,
	
	/// Active connections.
	Connections,
	
	/// Memory usage.
	Memory,
}

impl FromStr for MeasurementType {
	type Err = ();
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"times"       => Ok(MeasurementType::Times),
			"connections" => Ok(MeasurementType::Connections),
			"memory"      => Ok(MeasurementType::Memory),
			_             => Err(()),
		}
	}
}



//		Structs

//		AppStateStats															
/// Statistics-related central constructs to be stored in application state.
/// 
/// This is used to store global state information that is shared between
/// requests, specific to what is used for statistics purposes.
/// 
pub struct AppStateStats {
	//		Public properties													
	/// The application statistics data.
	pub Data:      AppStats,
	
	/// The statistics queue that response times are added to. This is the
	/// sender side only. A queue is used so that each request-handling thread's
	/// stats middleware can send its metrics into the queue instead of updating
	/// a central, locked data structure. This avoids the need for locking and
	/// incineration routines, as the stats-handling thread can constantly
	/// process the queue and there will theoretically never be a large build-up
	/// of data in memory that has to be dealt with all at once.
	pub Queue:     Sender<ResponseMetrics>,
	
	/// The statistics broadcast channel that period-based statistics are added
	/// to. This is the receiver side only. Each interested party can subscribe
	/// to this channel to receive the latest statistics for a given period on
	/// a real-time basis.
	pub Broadcast: Broadcaster<AllStatsForPeriod>,
}

//		AppStats																
/// Various application statistics.
#[derive(SmartDefault)]
pub struct AppStats {
	//		Public properties													
	/// The date and time the application was started.
	pub started_at:  NaiveDateTime,
	
	/// The latest second period that has been completed.
	pub last_second: RwLock<NaiveDateTime>,
	
	/// The current number of open connections, i.e. requests that have not yet
	/// been responded to.
	pub connections: AtomicUsize,
	
	/// The number of requests that have been made. The number of responses will
	/// be incremented only when the request has been fully handled and a
	/// response generated.
	pub requests:    AtomicUsize,
	
	/// The average, maximum, minimum, and count for each area sampled. The data
	/// is wrapped inside a [`Mutex`] because it is important to update the
	/// count, use that exact count to calculate the average, and then store
	/// that average all in one atomic operation while blocking any other
	/// process from using the data. A [`parking_lot::Mutex`] is used instead of
	/// a [`std::sync::Mutex`] because it is theoretically faster in highly
	/// contended situations, but the main advantage is that it is infallible,
	/// and it does not have mutex poisoning.
	pub totals:      Mutex<AppStatsTotals>,
	
	/// Circular buffers of average, maximum, minimum, and count per second for
	/// each area sampled, for the individually-configured periods. The buffers
	/// are stored inside a [`RwLock`] because they are only ever written to a
	/// maximum of once per second. A [`parking_lot::RwLock`] is used instead of
	/// a [`std::sync::RwLock`] because it is theoretically faster in highly
	/// contended situations.
	pub buffers:     RwLock<AppStatsBuffers>,
}

//		AppStatsTotals															
/// The all-time application statistics totals for each area sampled.
#[derive(SmartDefault)]
pub struct AppStatsTotals {
	//		Public properties													
	/// The number of responses that have been handled, by status code.
	#[default(hash_map!{
		StatusCode::OK:                    0,
		StatusCode::UNAUTHORIZED:          0,
		StatusCode::NOT_FOUND:             0,
		StatusCode::INTERNAL_SERVER_ERROR: 0,
	})]
	pub codes:       HashMap<StatusCode, u64>,
	
	/// The average, maximum, and minimum response times since the application
	/// last started.
	pub times:       StatsForPeriod,
	
	/// The average, maximum, and minimum response times by endpoint since the
	/// application last started. These statistics are stored in a [`HashMap`]
	/// for ease.
	pub endpoints:   HashMap<Endpoint, StatsForPeriod>,
	
	/// The average, maximum, and minimum open connections by time period.
	pub connections: StatsForPeriod,
	
	/// The average, maximum, and minimum memory usage by time period.
	pub memory:      StatsForPeriod,
}

//		AppStatsBuffers															
/// Buffers for storing application statistics data.
#[derive(SmartDefault)]
pub struct AppStatsBuffers {
	//		Public properties													
	/// A circular buffer of response time stats per second for the configured
	/// period.
	pub responses:   VecDeque<StatsForPeriod>,
	
	/// A circular buffer of connection stats per second for the configured
	/// period.
	pub connections: VecDeque<StatsForPeriod>,
	
	/// A circular buffer of memory usage stats per second for the configured
	/// period.
	pub memory:      VecDeque<StatsForPeriod>,
}

//		StatsForPeriod															
/// Average, maximum, minimum, and count of values for a period of time.
#[derive(Clone, Debug, Serialize, SmartDefault)]
pub struct StatsForPeriod {
	//		Public properties													
	/// The date and time the period started.
	#[default(Utc::now().naive_utc())]
	pub started_at: NaiveDateTime,
	
	/// Average value.
	pub average:    f64,
	
	/// Maximum value.
	pub maximum:    u64,
	
	/// Minimum value.
	pub minimum:    u64,
	
	/// The total number of values.
	pub count:      u64,
}

impl StatsForPeriod {
	//		initialize															
	/// Initialises the stats based on a single starting value.
	/// 
	/// # Parameters
	/// 
	/// * `value` - The single value to start with. This will be applied to the
	///             average, maximum, and minimum values, and the count will be
	///             set to 1.
	/// 
	pub fn initialize(value: u64) -> Self {
		Self {
			average: value as f64,
			maximum: value,
			minimum: value,
			count:   1,
			..Default::default()
		}
	}
	
	//		update																
	/// Updates the stats with new data.
	/// 
	/// This function will compare the new data with the existing data and
	/// update the stats accordingly. The maximum and minimum values will be
	/// updated if the new data is higher or lower than the existing values,
	/// and the count will be the combined count of the existing and new data.
	/// 
	/// Of particular note is the treatment of the average value. This is
	/// calculated using a weighted average, combining the existing and new
	/// averages using the count of each set of data as a weighting factor.
	/// This means that the average value will be closer to the average of the
	/// new data if the existing data is much larger than the new data, and vice
	/// versa.
	/// 
	/// The start time will not be updated.
	/// 
	/// # Parameters
	/// 
	/// * `stats` - The stats to update with.
	/// 
	pub fn update(&mut self, stats: &StatsForPeriod) {
		if (stats.minimum < self.minimum && stats.count > 0) || self.count == 0 {
			self.minimum = stats.minimum;
		}
		if stats.maximum > self.maximum {
			self.maximum = stats.maximum;
		}
		self.count      += stats.count;
		if self.count > 0  && stats.count > 0 {
			let weight   = stats.count as f64 / self.count as f64;
			self.average = self.average * (1.0 - weight) + stats.average * weight;
		}
	}
}

//		AllStatsForPeriod														
/// Average, maximum, minimum, and count of values for a period of time, for all
/// areas being measured.
#[derive(Clone, Debug, Default, Serialize, ToSchema)]
pub struct AllStatsForPeriod {
	//		Public properties													
	/// The average, maximum, and minimum response times in microseconds, plus
	/// sample count, for the most recent second.
	pub times:       StatsForPeriod,
	
	/// The average, maximum, and minimum open connections, plus sample count,
	/// for the most recent second.
	pub connections: StatsForPeriod,
	
	/// The average, maximum, and minimum memory usage in bytes, plus sample
	/// count, for the most recent second.
	pub memory:      StatsForPeriod,
}

//		ResponseMetrics															
/// Metrics for a single response.
/// 
/// This is used by the statistics queue in [`AppState.Stats.Queue`].
/// 
#[derive(SmartDefault)]
pub struct ResponseMetrics {
	//		Public properties													
	/// The endpoint that was requested.
	pub endpoint:    Endpoint,
	
	/// The date and time the request started.
	#[default(Utc::now().naive_utc())]
	pub started_at:  NaiveDateTime,
	
	/// The time the response took to be generated, in microseconds.
	pub time_taken:  u64,
	
	/// The status code of the response.
	pub status_code: StatusCode,
	
	/// The number of open connections at the time the response was generated.
	pub connections: u64,
	
	/// The amount of memory allocated at the time the response was generated,
	/// in bytes.
	pub memory:      u64,
}

//		GetStatsHistoryParams													
/// The parameters for the [`get_stats_history()`] handler.
#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct GetStatsHistoryParams {
	//		Public properties													
	/// The buffer to get the statistics for. The buffer items are returned in
	/// order of most-recent first.
	pub buffer: Option<MeasurementType>,
	
	/// The date and time to get the statistics from. This will apply from the
	/// given point in time until now, i.e. the check is, "is the time of the
	/// response item newer than or equal to the given time?". The expected
	/// format is `YYYY-MM-DDTHH:MM:SS`, e.g. `2023-10-18T06:08:34`.
	pub from:   Option<NaiveDateTime>,
	
	/// The number of buffer entries, i.e. the number of seconds, to get the
	/// statistics for. This will apply from now backwards, i.e. the count will
	/// start with the most-recent item and return up to the given number of
	/// items. If used with [`GetStatsHistoryParams::from`], this may seem
	/// somewhat counter-intuitive, as the item identified by that parameter may
	/// not be included in the results, but the items closest to the current
	/// time are the ones of most interest, and so asking for a maximum number
	/// of items is most likely to mean the X most-recent items rather than the
	/// X oldest items. Because the most-recent items are always returned first,
	/// the [`last_second`](StatsResponse::last_second)/[`last_second`](StatsHistoryResponse::last_second)
	/// property of the response will always be the time of the first item in
	/// the list.
	pub limit:  Option<usize>,
}

//		GetStatsFeedParams														
/// The parameters for the [`get_stats_feed()`] handler.
#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct GetStatsFeedParams {
	//		Public properties													
	/// The type of measurement to subscribe to statistics for.
	pub r#type: Option<MeasurementType>,
}

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

//		StatsResponse															
/// The application statistics returned by the `/api/stats` endpoint.
#[derive(Serialize, ToSchema)]
pub struct StatsResponse {
	//		Public properties													
	/// The date and time the application was started.
	pub started_at:  NaiveDateTime,
	
	/// The latest second period that has been completed.
	pub last_second: NaiveDateTime,
	
	/// The amount of time the application has been running, in seconds.
	pub uptime:      u64,
	
	/// The current number of open connections, i.e. requests that have not yet
	/// been responded to.
	pub active:      u64,
	
	/// The number of requests that have been made. The number of responses will
	/// be incremented only when the request has been fully handled and a
	/// response generated.
	pub requests:    u64,
	
	/// The number of responses that have been handled, by status code.
	#[serde(serialize_with = "serialize_status_codes")]
	pub codes:       HashMap<StatusCode, u64>,
	
	/// The average, maximum, and minimum response times in microseconds, plus
	/// sample count, grouped by time period.
	pub times:       IndexMap<String, StatsResponseForPeriod>,
	
	/// The average, maximum, and minimum response times in microseconds, plus
	/// sample count, grouped by endpoint, since the application last started.
	pub endpoints:   HashMap<Endpoint, StatsResponseForPeriod>,
	
	/// The average, maximum, and minimum open connections, plus sample count,
	/// grouped by time period.
	pub connections: IndexMap<String, StatsResponseForPeriod>,
	
	/// The average, maximum, and minimum memory usage in bytes, plus sample
	/// count, grouped by time period.
	pub memory:      IndexMap<String, StatsResponseForPeriod>,
}

//		StatsHistoryResponse													
/// The application statistics returned by the `/api/stats/history` endpoint.
#[derive(Default, Serialize, ToSchema)]
pub struct StatsHistoryResponse {
	//		Public properties													
	/// The latest second period that has been completed.
	pub last_second: NaiveDateTime,
	
	/// The average, maximum, and minimum response times in microseconds, plus
	/// sample count, per second for every second since the application last
	/// started, or up until the end of the [configured buffer](StatsOptions.timing_buffer_size).
	pub times:       Vec<StatsResponseForPeriod>,
	
	/// The average, maximum, and minimum open connections, plus sample count,
	/// per second for every second since the application last started, or up
	/// until the end of the [configured buffer](StatsOptions.connection_buffer_size).
	pub connections: Vec<StatsResponseForPeriod>,
	
	/// The average, maximum, and minimum memory usage in bytes, plus sample
	/// count, per second for every second since the application last started,
	/// or up until the end of the [configured buffer](StatsOptions.memory_buffer_size).
	pub memory:      Vec<StatsResponseForPeriod>,
}

//		StatsResponseForPeriod													
/// Average, maximum, minimum, and count of values for a period of time.
#[derive(Serialize, ToSchema)]
pub struct StatsResponseForPeriod {
	//		Public properties													
	/// Average value.
	pub average: f64,
	
	/// Maximum value.
	pub maximum: u64,
	
	/// Minimum value.
	pub minimum: u64,
	
	/// The total number of values.
	pub count:   u64,
}

impl From<&StatsForPeriod> for StatsResponseForPeriod {
	//		from																
	fn from(stats: &StatsForPeriod) -> Self {
		Self {
			average: stats.average,
			maximum: stats.maximum,
			minimum: stats.minimum,
			count:   stats.count,
		}
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
	let stats_cx = StatsContext::default();
	request.extensions_mut().insert(stats_cx.clone());
	
	//	Check if statistics are enabled
	if !appstate.Config.stats.enabled {
		return next.run(request).await;
	}
	
	//	Obtain endpoint details
	let endpoint = Endpoint {
		path:      request.uri().path().to_string(),
		method:    request.method().clone(),
	};
	
	//	Update requests counter
	appstate.Stats.Data.requests.fetch_add(1, Ordering::Relaxed);
	appstate.Stats.Data.connections.fetch_add(1, Ordering::Relaxed);
	
	//	Process request
	let response = next.run(request).await;
	
	//	Add response time to the queue
	appstate.Stats.Queue.send(ResponseMetrics {
		endpoint,
		started_at:  stats_cx.started_at,
		time_taken:  (Utc::now().naive_utc() - stats_cx.started_at).num_microseconds().unwrap() as u64,
		status_code: response.status(),
		connections: appstate.Stats.Data.connections.load(Ordering::Relaxed) as u64,
		memory:	     Malloc::read().unwrap() as u64,
	}).expect("Failed to send response time");
	
	appstate.Stats.Data.connections.fetch_sub(1, Ordering::Relaxed);
	
	//	Return response
	response
}

//		start_stats_processor													
/// Starts the statistics processor.
/// 
/// This function starts a thread that will process the statistics queue in
/// [`AppState.Stats.Queue`]. It will run until the channel is disconnected.
/// 
/// The processing of the statistics is done in a separate thread so that the
/// request-handling threads can continue to handle requests without being
/// blocked by the statistics processing. This way, none of them are ever
/// affected more than others. The stats-handling thread blocks on the queue, so
/// it will only process a response time when one is available.
/// 
/// The thread will also wake up every second to ensure that the period that has 
/// just ended gets wrapped up. This is necessary because the thread otherwise
/// only wakes up when the queue has data in it, and if there is a period of
/// inactivity then the current period will not be completed until the next
/// request comes in. This can lead to a long delay until the statistics are
/// updated, which is undesirable because the buffer will be stuck at the
/// position of the last period to be completed.
/// 
/// Although this periodic wake-up does incur a very slight overhead, it is
/// extremely small, and ensures that the statistics are always up-to-date.
/// 
/// # Parameters
/// 
/// * `receiver` - The receiving end of the queue.
/// * `appstate` - The application state.
/// 
pub async fn start_stats_processor(receiver: Receiver<ResponseMetrics>, appstate: Arc<AppState>) {
	//	Fixed time period of the current second
	let mut current_second = Utc::now().naive_utc().with_nanosecond(0).unwrap();
	//	Cumulative stats for the current second
	let mut timing_stats   = StatsForPeriod::default();
	let mut conn_stats     = StatsForPeriod::default();
	let mut memory_stats   = StatsForPeriod::default();
	
	//	Initialise circular buffers. We reserve the capacities here right at the
	//	start so that the application always uses exactly the same amount of
	//	memory for the buffers, so that any memory-usage issues will be spotted
	//	immediately. For instance, if someone set the config value high enough
	//	to store a year's worth of data (around 1.8GB) and the system didn't
	//	have enough memory it would fail right away, instead of gradually
	//	building up to that point which would make it harder to diagnose.
	{
		let mut buffers    = appstate.Stats.Data.buffers.write();
		buffers.responses  .reserve(appstate.Config.stats.timing_buffer_size);
		buffers.connections.reserve(appstate.Config.stats.connection_buffer_size);
		buffers.memory     .reserve(appstate.Config.stats.memory_buffer_size);
	}
	
	//	Wait until the start of the next second, to align with it so that the
	//	tick interval change happens right after the second change, to wrap up
	//	the data for the period that has just ended.
	sleep((current_second + Duration::seconds(1) - Utc::now().naive_utc()).to_std().unwrap()).await;
	
	//	Queue processing loop
	let mut timer = interval(Duration::seconds(1).to_std().unwrap());
	spawn(async move { loop { select!{
		_ = timer.tick()      => {
			//	Ensure last period is wrapped up
			(timing_stats, conn_stats, memory_stats, current_second) = stats_processor(
				Arc::clone(&appstate), None, timing_stats, conn_stats, memory_stats, current_second
			);
		}
		//	Wait for message - this is a blocking call
		message = receiver.recv_async() => { match message {
			Ok(response_time) => {
				//	Process response time
				(timing_stats, conn_stats, memory_stats, current_second) = stats_processor(
					Arc::clone(&appstate), Some(response_time), timing_stats, conn_stats, memory_stats, current_second
				);
			},
			Err(_)            => {
				eprintln!("Channel has been disconnected, exiting thread.");
				break;
			},
		}}
	}}});
}

//		stats_processor															
/// Processes a single response time.
/// 
/// This function processes a single response metrics sample, updating the
/// calculated statistics accordingly.
/// 
/// # Parameters
/// 
/// * `appstate`       - The application state.
/// * `metrics`        - The response metrics to process, received from the
///                      statistics queue in [`AppState.Stats.Queue`]. If
///                      [`None`], then no stats will be added or altered, and
///                      no counters will be incremented, but the most-recent
///                      period will be checked and wrapped up if not already
///                      done.
/// * `timing_stats`   - The cumulative timing stats for the current second.
/// * `conn_stats`     - The cumulative connection stats for the current second.
/// * `memory_stats`   - The cumulative memory stats for the current second.
/// * `current_second` - The current second.
/// 
fn stats_processor(
	appstate:           Arc<AppState>,
	metrics:            Option<ResponseMetrics>,
	mut timing_stats:   StatsForPeriod,
	mut conn_stats:     StatsForPeriod,
	mut memory_stats:   StatsForPeriod,
	mut current_second: NaiveDateTime
) -> (StatsForPeriod, StatsForPeriod, StatsForPeriod, NaiveDateTime) {
	//		Helper functions													
	fn update_buffer(
		buffer:             &mut VecDeque<StatsForPeriod>,
		buffer_size:        usize,
		mut stats:          StatsForPeriod,
		current_second:     NaiveDateTime,
		elapsed:            i64,
		message:            &mut AllStatsForPeriod,
		mut update_message: impl FnMut(StatsForPeriod, &mut AllStatsForPeriod),
	) -> StatsForPeriod {
		for i in 0..elapsed {
			if buffer.len() == buffer_size {
				buffer.pop_back();
			}
			stats.started_at = current_second + Duration::seconds(i);
			buffer.push_front(stats.clone());
			update_message(stats, message);
			stats            = StatsForPeriod::default();
		}
		stats
	}
	
	//		Preparation															
	let new_second: NaiveDateTime;
	if let Some(metrics) = metrics {
		//	Prepare new stats
		let new_timing_stats = StatsForPeriod::initialize(metrics.time_taken);
		let new_conn_stats   = StatsForPeriod::initialize(metrics.connections);
		let new_memory_stats = StatsForPeriod::initialize(metrics.memory);
		
		//	Increment cumulative stats
		timing_stats.update(&new_timing_stats);
		conn_stats  .update(&new_conn_stats);
		memory_stats.update(&new_memory_stats);
		
	//		Update statistics													
		//	Lock source data
		let mut totals = appstate.Stats.Data.totals.lock();
		
		//	Update responses counter
		*totals.codes.entry(metrics.status_code).or_insert(0) += 1;
		
		//	Update response time stats
		totals.times.update(&new_timing_stats);
		
		//	Update endpoint response time stats
		totals.endpoints
			.entry(metrics.endpoint)
			.and_modify(|ep_stats| ep_stats.update(&new_timing_stats))
			.or_insert(new_timing_stats)
		;
		
		//	Update connections usage stats
		totals.connections.update(&new_conn_stats);
		
		//	Update memory usage stats
		totals.memory.update(&new_memory_stats);
		
		//	Unlock source data
		drop(totals);
		
	//		Check time period													
		new_second = metrics.started_at.with_nanosecond(0).unwrap();
	} else {
		new_second = Utc::now().naive_utc().with_nanosecond(0).unwrap();
	};
	
	//	Check to see if we've moved into a new time period. We want to increment
	//	the request count and total response time until it "ticks" over into
	//	another second. At this point it will calculate an average and add this
	//	data (average, min, max) to a fixed-length circular buffer of seconds.
	//	This way, the last period's data can be calculated by looking through
	//	the circular buffer of seconds.
	if new_second > current_second {
		let elapsed     = (new_second - current_second).num_seconds();
		let mut buffers = appstate.Stats.Data.buffers.write();
		let mut message = AllStatsForPeriod::default();
		//	Timing stats buffer
		timing_stats = update_buffer(
			&mut buffers.responses,
			appstate.Config.stats.timing_buffer_size,
			timing_stats,
			current_second,
			elapsed,
			&mut message,
			|stats, message| { message.times = stats; },
		);
		//	Connections stats buffer
		conn_stats   = update_buffer(
			&mut buffers.connections,
			appstate.Config.stats.connection_buffer_size,
			conn_stats,
			current_second,
			elapsed,
			&mut message,
			|stats, message| { message.connections = stats; },
		);
		//	Memory stats buffer
		memory_stats = update_buffer(
			&mut buffers.memory,
			appstate.Config.stats.memory_buffer_size,
			memory_stats,
			current_second,
			elapsed,
			&mut message,
			|stats, message| { message.memory = stats; },
		);
		*appstate.Stats.Data.last_second.write() = current_second;
		current_second = new_second;
		appstate.Stats.Broadcast.send(message).expect("Failed to broadcast stats");
	}
	
	(timing_stats, conn_stats, memory_stats, current_second)
}

//		get_stats																
/// Application statistics overview.
/// 
/// This endpoint produces various statistics about the application. It returns
/// a JSON object containing the following information:
/// 
///   - `started_at`  - The date and time the application was started, in ISO
///                     8601 format.
///   - `last_second` - The latest second period that has been completed.
///   - `uptime`      - The amount of time the application has been running, in
///                     seconds.
///   - `requests`    - The number of requests that have been handled since the
///                     application last started.
///   - `active`      - The number of current open connections.
///   - `codes`       - The counts of responses that have been handled, broken
///                     down by status code, since the application last started.
///   - `times`       - The average, maximum, and minimum response times, plus
///                     sample count, for the [configured periods](StatsOptions.stats_periods),
///                     and since the application last started.
///   - `endpoints`   - The counts of responses that have been handled, broken
///                     down by endpoint, since the application last started.
///   - `connections` - The average, maximum, and minimum open connections, plus
///                     sample count, for the [configured periods](StatsOptions.stats_periods),
///                     and since the application last started.
///   - `memory`      - The average, maximum, and minimum memory usage, plus
///                     sample count, for the [configured periods](StatsOptions.stats_periods),
///                     and since the application last started.
/// 
/// # Parameters
/// 
/// * `state` - The application state.
/// 
#[utoipa::path(
	get,
	path = "/api/stats",
	tag  = "health",
	responses(
		(status = 200, description = "Application statistics overview", body = StatsResponse),
	)
)]
pub async fn get_stats(State(state): State<Arc<AppState>>) -> Json<StatsResponse> {
	//		Helper functions													
	fn initialize_map(
		periods: &HashMap<String, usize>,
		buffer:  &VecDeque<StatsForPeriod>,
	) -> IndexMap<String, StatsForPeriod> {
		let mut output: IndexMap<String, StatsForPeriod> = periods
			.iter()
			.sorted_by(|a, b| a.1.cmp(b.1))
			.map(|(name, _)| (name.clone(), StatsForPeriod { ..Default::default() }))
			.collect()
		;
		//	Loop through the circular buffer and calculate the stats
		for (i, stats) in buffer.iter().enumerate() {
			for (name, period) in periods.iter() {
				if i < *period {
					output.get_mut(name).unwrap().update(stats);
				}
			}
		}
		output
	}
	
	fn convert_map(
		input: IndexMap<String, StatsForPeriod>,
		all:   &StatsForPeriod
	) -> IndexMap<String, StatsResponseForPeriod> {
		let mut output: IndexMap<String, StatsResponseForPeriod> = input
			.into_iter()
			.map(|(key, value)| (key, StatsResponseForPeriod::from(&value)))
			.collect()
		;
		output.insert(s!("all"), StatsResponseForPeriod::from(all));
		output
	}
	
	//		Preparation															
	//	Lock source data
	let buffers      = state.Stats.Data.buffers.read();
	
	//	Create pots for each period and process stats buffers
	let timing_input = initialize_map(&state.Config.stats_periods, &buffers.responses);
	let conn_input   = initialize_map(&state.Config.stats_periods, &buffers.connections);
	let memory_input = initialize_map(&state.Config.stats_periods, &buffers.memory);
	
	//	Unlock source data
	drop(buffers);
	
	//		Process stats														
	//	Lock source data
	let totals        = state.Stats.Data.totals.lock();
	
	//	Convert the input stats data into the output stats data
	let timing_output = convert_map(timing_input, &totals.times);
	let conn_output   = convert_map(conn_input,   &totals.connections);
	let memory_output = convert_map(memory_input, &totals.memory);
	
	//		Build response data													
	let now        = Utc::now().naive_utc();
	let response   = Json(StatsResponse {
		started_at:  state.Stats.Data.started_at.with_nanosecond(0).unwrap(),
		last_second: *state.Stats.Data.last_second.read(),
		uptime:      (now - state.Stats.Data.started_at).num_seconds() as u64,
		active:      state.Stats.Data.connections.load(Ordering::Relaxed) as u64,
		requests:    state.Stats.Data.requests.load(Ordering::Relaxed) as u64,
		codes:       totals.codes.clone(),
		times:       timing_output,
		endpoints:   HashMap::from_iter(
			totals.endpoints.clone()
				.into_iter()
				.map(|(key, value)| (key, StatsResponseForPeriod::from(&value)))
		),
		connections: conn_output,
		memory:      memory_output,
	});
	//	Unlock source data
	drop(totals);
	
	//		Response															
	response
}

//		get_stats_history														
/// Historical application statistics interval data.
/// 
/// This endpoint provides access to historical application statistics interval
/// data available from the statistics buffers. It returns a JSON object
/// containing the following information:
/// 
///   - `last_second` - The latest second period that has been completed.
///   - `times`       - The average, maximum, and minimum response times, plus
///                     sample count, per second for every second since the
///                     application last started, or up until the end of the
///                     [configured buffer](StatsOptions.timing_buffer_size).
///   - `connections` - The average, maximum, and minimum open connections, plus
///                     sample count, per second for every second since the
///                     application last started, or up until the end of the
///                     [configured buffer](StatsOptions.connection_buffer_size).
///   - `memory`      - The average, maximum, and minimum memory usage, plus
///                     sample count, per second for every second since the
///                     application last started, or up until the end of the
///                     [configured buffer](StatsOptions.memory_buffer_size).
/// 
/// # Parameters
/// 
/// * `state`  - The application state.
/// * `params` - The parameters for the request.
/// 
#[utoipa::path(
	get,
	path = "/api/stats/history",
	tag  = "health",
	params(
		GetStatsHistoryParams,
	),
	responses(
		(status = 200, description = "Historical application statistics interval data", body = StatsHistoryResponse),
	)
)]
pub async fn get_stats_history(
	State(state):  State<Arc<AppState>>,
	Query(params): Query<GetStatsHistoryParams>,
) -> Json<StatsHistoryResponse> {
	//		Helper function														
	fn process_buffer(
		buffer: &VecDeque<StatsForPeriod>,
		from:   Option<NaiveDateTime>,
		limit:  Option<usize>,
	) -> Vec<StatsResponseForPeriod> {
		buffer.iter().take_while(|entry| {
			match from {
				Some(from) => entry.started_at >= from,
				None       => true,
			}
		})
			.limit(limit)
			.map(StatsResponseForPeriod::from)
			.collect()
	}
	
	//		Prepare response data												
	//	Lock source data
	let buffers      = state.Stats.Data.buffers.read();
	let mut response = StatsHistoryResponse {
		last_second:   *state.Stats.Data.last_second.read(),
		..Default::default()
	};
	//	Convert the statistics buffers
	match params.buffer {
		Some(MeasurementType::Times)       => {
			response.times       = process_buffer(&buffers.responses,   params.from, params.limit);
		},
		Some(MeasurementType::Connections) => {
			response.connections = process_buffer(&buffers.connections, params.from, params.limit);
		},
		Some(MeasurementType::Memory)      => {
			response.memory      = process_buffer(&buffers.memory,      params.from, params.limit);
		},
		None                               => {
			response.times       = process_buffer(&buffers.responses,   params.from, params.limit);
			response.connections = process_buffer(&buffers.connections, params.from, params.limit);
			response.memory      = process_buffer(&buffers.memory,      params.from, params.limit);
		},
	}
	//	Unlock source data
	drop(buffers);
	Json(response)
}

//		get_stats_feed															
/// Application statistics event feed.
/// 
/// This endpoint returns an open WebSocket connection for a feed of statistics
/// events. It will establish a handshake with the [`WebSocket`] and then pass
/// over to [`ws_stats_feed()`] to handle the connection. This function will
/// then return a [`Response`] with a status code of `101 Switching Protocols`
/// and the `Connection` header set to `Upgrade`.
/// 
/// # Parameters
/// 
/// * `state`  - The application state.
/// * `params` - The parameters for the request.
/// * `ws_req` - The websocket request.
/// 
#[utoipa::path(
	get,
	path = "/api/stats/feed",
	tag  = "health",
	params(
		GetStatsFeedParams,
	),
	responses(
		(status = 200, description = "Application statistics event feed"),
	),
)]
pub async fn get_stats_feed(
	State(state):  State<Arc<AppState>>,
	Query(params): Query<GetStatsFeedParams>,
	ws_req:        WebSocketUpgrade,
) -> Response {
	//	Establish a handshake with the WebSocket
	ws_req.on_upgrade(move |socket| ws_stats_feed(Arc::clone(&state), socket, params.r#type))
}

//		ws_stats_feed															
/// WebSocket feed of application statistics events.
/// 
/// This endpoint returns a feed of application statistics over a WebSocket
/// connection established by [`get_stats_feed()`]. Statistics events are sent
/// as they are received from the broadcast channel. The events are
/// [`StatsForPeriod`] instances, sent as JSON objects.
/// 
/// Notably, if not filtered by measurement type, all measurement types will
/// have their statistics returned in a JSON object, with the type names as keys
/// and the statistics data in sub-objects. However, when filtered by type, only
/// the statistics object for that one type will be returned. This is in order
/// to keep the transmitted data as efficient as possible.
/// 
/// # Parameters
/// 
/// * `state` - The application state.
/// * `ws`    - The websocket stream.
/// * `scope` - The type of measurement statistics to send.
/// 
pub async fn ws_stats_feed(
	state:  Arc<AppState>,
	mut ws: WebSocket,
	scope:  Option<MeasurementType>,
) {
	//		Preparation															
	info!("WebSocket connection established");
	//	Subscribe to the broadcast channel
	let mut rx        = state.Stats.Broadcast.subscribe();
	//	Set up a timer to send pings at regular intervals
	let mut timer     = interval(Duration::seconds(state.Config.stats.ws_ping_interval as i64).to_std().unwrap());
	let mut timeout   = interval(Duration::seconds(state.Config.stats.ws_ping_timeout  as i64).to_std().unwrap());
	let mut last_ping = None;
	let mut last_pong = Instant::now();
	
	//	Message processing loop
	loop { select! {
		//		Ping															
		//	Send a ping at regular intervals
		_ = timer.tick() => {
			if let Err(err) = ws.send(Message::Ping(Vec::new())).await {
				warn!("Failed to send ping over WebSocket: {}", err);
				break;
			}
			last_ping = Some(Instant::now());
		},
		//		Ping/pong timeout												
		//	Check for ping timeout (X seconds since the last ping without a pong)
		_ = timeout.tick() => {
			if let Some(ping_time) = last_ping {
				let limit = Duration::seconds(state.Config.stats.ws_ping_timeout as i64).to_std().unwrap();
				if last_pong < ping_time && ping_time.elapsed() > limit {
					warn!("WebSocket ping timed out");
					break;
				}
			}
		},
		//		Incoming message												
		//	Handle incoming messages from the WebSocket
		Some(msg) = ws.recv() => {
			match msg {
				Ok(Message::Ping(ping)) => {
					if let Err(err) = ws.send(Message::Pong(ping)).await {
						warn!("Failed to send pong over WebSocket: {}", err);
						break;
					}
				}
				Ok(Message::Pong(_))    => {
					last_pong = Instant::now();
				}
				Ok(Message::Close(_))   => {
					info!("WebSocket connection closed");
					break;
				}
				Ok(Message::Text(_))    => {
					warn!("Unexpected WebSocket text message");
				}
				Ok(Message::Binary(_))  => {
					warn!("Unexpected WebSocket binary message");
				}
				Err(err)                => {
					warn!("WebSocket error: {}", err);
					break;
				}
				#[allow(unreachable_patterns)]
				_                       => {
					//	At present there are no other message types, but this is here to catch
					//	any future additions.
					warn!("Unknown WebSocket message type");
				}
			}	
		}
		//		Send stats data													
		//	Handle new data from the broadcast channel
		Ok(data) = rx.recv() => {
			let response = match scope {
				Some(MeasurementType::Times)       => {
					json!{StatsResponseForPeriod::from(&data.times)}
				},
				Some(MeasurementType::Connections) => {
					json!{StatsResponseForPeriod::from(&data.connections)}
				},
				Some(MeasurementType::Memory)      => {
					json!{StatsResponseForPeriod::from(&data.memory)}
				},
				None                               => {
					json!{btree_map!{
						"times":       StatsResponseForPeriod::from(&data.times),
						"connections": StatsResponseForPeriod::from(&data.connections),
						"memory":      StatsResponseForPeriod::from(&data.memory),
					}}
				},
			};
			if let Err(err) = ws.send(Message::Text(response.to_string())).await {
				warn!("Failed to send data over WebSocket: {}", err);
				break;
			}
		}
	}}
}

//		serialize_status_codes													
/// Returns a list of serialised status code entries and their values.
/// 
/// This function is used by [`serde`] to serialise a list of status codes and
/// their associated values. It returns the list sorted by status code.
/// 
/// # Parameters
/// 
/// * `status_codes` - The status codes to serialise, as keys, against values.
/// * `serializer`   - The serialiser to use.
/// 
pub fn serialize_status_codes<S>(status_codes: &HashMap<StatusCode, u64>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let codes: BTreeMap<String, u64> = status_codes
		.iter()
		.map(|(key, value)| (key.to_string(), *value))
		.collect()
	;
	codes.serialize(serializer)
}


