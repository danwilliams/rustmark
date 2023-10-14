#![allow(non_snake_case)]

//		Tests

//		healthcheck																
#[cfg(test)]
mod healthcheck {
	use super::super::*;
	use crate::utility::{AppStats, AppStatsResponses, AppStatsResponseCounts, Config};
	use assert_json_diff::assert_json_eq;
	use axum::http::{Method, StatusCode};
	use chrono::Duration;
	use figment::{Figment, providers::Serialized};
	use flume::{self};
	use parking_lot::Mutex;
	use rand::Rng;
	use ring::hmac::{HMAC_SHA512, self};
	use rubedo::{
		http::{ResponseExt, UnpackedResponse, UnpackedResponseBody, UnpackedResponseHeader},
		sugar::s,
	};
	use std::sync::atomic::AtomicUsize;
	use serde_json::json;
	use tera::Tera;
	use velcro::hash_map;
	
	//		ping																
	#[tokio::test]
	async fn ping() {
		let unpacked = get_ping().await.into_response().unpack().unwrap();
		let crafted  = UnpackedResponse {
			status:    StatusCode::OK,
			headers:   vec![],
			body:      UnpackedResponseBody::default(),
		};
		assert_eq!(unpacked, crafted);
	}
	
	//		stats																
	#[tokio::test]
	async fn stats() {
		//	There is a very small possibility that this test will fail if the
		//	test is run at the exact moment that the date changes.
		let start           = Utc::now().naive_utc() - Duration::seconds(99);
		let (sender, _)     = flume::unbounded();
		let secret          = rand::thread_rng().gen::<[u8; 64]>();
		let state           = Arc::new(AppState {
			Config:           Figment::from(Serialized::defaults(Config::default())).extract().unwrap(),
			Stats:            AppStats {
				started_at:   start,
				requests:     AtomicUsize::new(10),
				responses:    Mutex::new(AppStatsResponses {
					counts:        AppStatsResponseCounts {
						total:     15,
						codes:     hash_map!{
							StatusCode::OK:                    5,
							StatusCode::UNAUTHORIZED:          4,
							StatusCode::NOT_FOUND:             3,
							StatusCode::INTERNAL_SERVER_ERROR: 2,
						},
						untracked: 1,
					},
					times:         Default::default(),
					endpoints:     hash_map!{
						Endpoint {
							method:     Method::GET,
							path:       s!("/api/stats"),
						}:              AppStatsForPeriod {
							started_at: start,
							average:    500.0,
							maximum:    1000,
							minimum:    100,
							count:      10,
							sum:        5000,
						}
					}
				}),
				memory:       Default::default(),
				..Default::default()
			},
			Queue:            sender,
			Secret:           secret,
			Key:              hmac::Key::new(HMAC_SHA512, &secret),
			Template:         Tera::default(),
		});
		let unpacked        = get_stats(State(state)).await.into_response().unpack().unwrap();
		let crafted         = UnpackedResponse {
			status:           StatusCode::OK,
			headers:          vec![
				//	Axum automatically adds a content-type header.
				UnpackedResponseHeader {
					name:     s!("content-type"),
					value:    s!("application/json"),
				},
			],
			body:             UnpackedResponseBody::new(json!({
				"started_at": start,
				"uptime":     99,
				"requests":   10,
				"responses":  {
					"counts": {
						"total":                         15,
						"codes":                         {
							"200 OK":                    5,
							"401 Unauthorized":          4,
							"404 Not Found":             3,
							"500 Internal Server Error": 2,
						},
						"untracked":                     1,
					},
					"times":  {
						"second":      {
							"average": 0.0,
							"maximum": 0,
							"minimum": 0,
							"count":   0,
						},
						"minute":      {
							"average": 0.0,
							"maximum": 0,
							"minimum": 0,
							"count":   0,
						},
						"hour":        {
							"average": 0.0,
							"maximum": 0,
							"minimum": 0,
							"count":   0,
						},
						"day":         {
							"average": 0.0,
							"maximum": 0,
							"minimum": 0,
							"count":   0,
						},
						"all":         {
							"average": 0.0,
							"maximum": 0,
							"minimum": 0,
							"count":   0,
						},
					},
					"endpoints": {
						"GET /api/stats": {
							"average":    500.0,
							"maximum":    1000,
							"minimum":    100,
							"count":      10,
						},
					},
				},
				"memory": {
					"second":      {
						"average": 0.0,
						"maximum": 0,
						"minimum": 0,
						"count":   0,
					},
					"minute":      {
						"average": 0.0,
						"maximum": 0,
						"minimum": 0,
						"count":   0,
					},
					"hour":        {
						"average": 0.0,
						"maximum": 0,
						"minimum": 0,
						"count":   0,
					},
					"day":         {
						"average": 0.0,
						"maximum": 0,
						"minimum": 0,
						"count":   0,
					},
					"all":         {
						"average": 0.0,
						"maximum": 0,
						"minimum": 0,
						"count":   0,
					},
				},
			})),
		};
		assert_json_eq!(unpacked, crafted);
	}
}


