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

//		no_route																
/// Handles non-existent routes.
/// 
/// This function is called as a fallback when a route is not found. It returns
/// a 404 status code.
pub async fn no_route() -> impl IntoResponse {
	(
		StatusCode::NOT_FOUND,
		[
			("protected", "protected"),
		],
	).into_response()
}

//		graceful_error_layer													
/// Handles errors gracefully.
/// 
/// This function is called when an error occurs. It returns a 500 status code
/// and a page with the error message.
/// 
/// If the error is a 404, it returns a 404 status code and a page with a link
/// to the login page.
/// 
/// # Parameters
/// 
/// * `state`   - The application state.
/// * `user`    - The user, if any.
/// * `uri`     - The URI of the request.
/// * `request` - The request.
/// * `next`    - The next middleware.
/// 
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
			let mut context = Context::new();
			context.insert("Title", &state.Config.title);
			(
				parts,
				Html(state.Template.render("404-notfound", &context).unwrap()),
			).into_response()
		},
		//	500
		StatusCode::INTERNAL_SERVER_ERROR => {
			let mut context = Context::new();
			context.insert("Title", &state.Config.title);
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
/// Catches unhandled errors.
/// 
/// This function is called when an error occurs in the
/// [`graceful_error_layer()`] handler. It returns a 500 status code and an
/// error message.
/// 
/// # Parameters
/// 
/// * `request` - The request.
/// * `next`    - The next middleware.
/// 
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


