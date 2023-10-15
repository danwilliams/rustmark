#![allow(non_snake_case)]

//		Tests

use super::super::*;
use axum::{
	http::StatusCode,
	response::IntoResponse,
};
use rubedo::http::{ResponseExt, UnpackedResponse, UnpackedResponseBody};

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


