//! Utility functions and types for the application.



//		Packages

use terracotta::{health, stats};
use utoipa::OpenApi;



//		Structs

//		ApiDoc																	
/// The OpenAPI documentation for the API.
#[derive(OpenApi)]
#[openapi(
	paths(
		health::handlers::get_ping,
		health::handlers::get_version,
		stats::handlers::get_stats,
		stats::handlers::get_stats_history,
		stats::handlers::get_stats_feed,
	),
	components(
		schemas(
			health::responses::HealthVersionResponse,
			stats::requests::MeasurementType,
			stats::responses::StatsResponse,
			stats::responses::StatsResponseForPeriod,
			stats::responses::StatsHistoryResponse,
		),
	),
	tags(
		(name = "health", description = "Health check endpoints"),
	),
)]
pub struct ApiDoc;


