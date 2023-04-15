//		Packages

use crate::{
	auth::{User, get_login},
	utility::*,
};
use axum::{
	Extension,
	extract::State,
	http::{Request, StatusCode, Uri},
	middleware::Next,
	response::{Html, IntoResponse, Response},
};
use std::sync::Arc;
use tera::Context;



//		Functions

//		get_login																
pub async fn no_route() -> impl IntoResponse {
	(
		StatusCode::NOT_FOUND,
		[
			("protected", "protected"),
		],
	).into_response()
}

//		graceful_error_layer													
pub async fn graceful_error_layer<B>(
	State(state):    State<Arc<AppState>>,
	Extension(user): Extension<Option<User>>,
	uri:             Uri,
	request:         Request<B>,
	next:            Next<B>,
) -> Response {
	let response          = next.run(request).await;
	let (mut parts, body) = response.into_parts();
	match parts.status {
		//	404
		StatusCode::NOT_FOUND             => {
			parts.headers.remove("content-length");
			parts.headers.remove("content-type");
			if parts.headers.contains_key("protected") {
				parts.headers.remove("protected");
				if user.is_none() {
					parts.status = StatusCode::UNAUTHORIZED;
					return (
						parts,
						get_login(State(state), uri).await,
					).into_response();
				}
			}
			let context = Context::new();
			(
				parts,
				Html(state.Template.render("404-notfound", &context).unwrap()),
			).into_response()
		},
		//	500
		StatusCode::INTERNAL_SERVER_ERROR => {
			let context = Context::new();
			parts.headers.remove("content-length");
			parts.headers.remove("content-type");
			parts.headers.insert("error-handled", "gracefully".parse().unwrap());
			(
				parts,
				Html(state.Template.render("500-error", &context).unwrap()),
			).into_response()
		},
		_                                 => {
			(
				parts,
				body,
			).into_response()
		},
	}
}

//		final_error_layer														
pub async fn final_error_layer<B>(
	request:  Request<B>,
	next:     Next<B>,
) -> Response {
	let response = next.run(request).await;
	match response.status() {
		StatusCode::INTERNAL_SERVER_ERROR => {
			let (mut parts, body) = response.into_parts();
			if parts.headers.contains_key("error-handled") {
				parts.headers.remove("error-handled");
				return (parts, body).into_response();
			}
			parts.headers.remove("content-length");
			parts.headers.remove("content-type");
			(
				parts,
				Html(r#"<h1>Internal server error</h1>"#),
			).into_response()
		},
		_                                 => response,
	}
}


