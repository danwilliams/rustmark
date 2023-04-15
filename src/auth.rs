#![allow(non_snake_case)]

//		Packages

use crate::{
	utility::*,
};
use axum::{
	Extension,
	Form,
	async_trait,
	extract::{FromRequestParts, State},
	http::{Request, StatusCode, Uri, request::Parts},
	middleware::Next,
	response::{Html, IntoResponse, Redirect, Response},
};
use axum_sessions::SessionHandle;
use base64::engine::{Engine as _, general_purpose::STANDARD as BASE64};
use ring::hmac;
use secrecy::{ExposeSecret, SecretVec};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tera::Context;
use tracing::info;



//		Constants

const SESSION_AUTH_ID_KEY: &str = "_auth_id";
const SESSION_USER_ID_KEY: &str = "_user_id";



//		Structs

//		PostLogin																
#[derive(Debug, Deserialize)]
pub struct PostLogin {
	username: String,
	password: String,
	uri:      String,
}

//		User																	
#[derive(Clone, Debug, Serialize)]
pub struct User {
	username: String,
	password: String,
}

impl User {
	//		find																
	pub async fn find(state: Arc<AppState>, username: &String, password: &String) -> Option<Self> {
		if state.Config.users.contains_key(username) {
			let pass = state.Config.users.get(username).unwrap();
			if pass == password {
				return Some(Self {
					username: username.clone(),
					password: pass.clone(),
				});
			}
		}
		None
	}
	
	//		find_by_id															
	pub async fn find_by_id(state: Arc<AppState>, id: &String) -> Option<Self> {
		if state.Config.users.contains_key(id) {
			let password = state.Config.users.get(id).unwrap();
			return Some(Self {
				username: id.clone(),
				password: password.clone(),
			});
		}
		None
	}
	
	//		get_password_hash													
	pub fn get_password_hash(&self) -> SecretVec<u8> {
		SecretVec::new(self.password.clone().into())
	}
}

//		AuthContext																
#[derive(Clone)]
pub struct AuthContext {
	pub current_user: Option<User>,
	session_handle:   SessionHandle,
	key:              hmac::Key,
}

impl AuthContext {
	//		new																	
	pub fn new(session_handle: SessionHandle, key: hmac::Key) -> Self {
		Self {
			current_user: None,
			session_handle,
			key,
		}
	}
	
	//		get_session_auth_id													
	fn get_session_auth_id(&self, password_hash: &[u8]) -> String {
		let tag = hmac::sign(&self.key, password_hash);
		BASE64.encode(tag.as_ref())
	}
	
	//		get_user															
	pub async fn get_user(&mut self, appstate: Arc<AppState>) -> Option<User> {
		let session                 = self.session_handle.read().await;
		if let Some(user_id)        = session.get::<String>(SESSION_USER_ID_KEY) {
			if let Some(user)       = User::find_by_id(Arc::clone(&appstate), &user_id).await {
				let session_auth_id = session
					.get::<String>(SESSION_AUTH_ID_KEY)
					.and_then(|auth_id| BASE64.decode(auth_id).ok())
					.unwrap_or_default()
				;
				drop(session);
				let password_hash   = user.get_password_hash();
				let data            = password_hash.expose_secret();
				if hmac::verify(&self.key, data, &session_auth_id).is_ok() {
					return Some(user);
				} else {
					self.logout().await;
				}
			}
		}
		None
	}
	
	//		login																
	pub async fn login(&mut self, user: &User) {
		let auth_id       = self.get_session_auth_id(user.get_password_hash().expose_secret());
		let user_id       = &user.username;
		let mut session   = self.session_handle.write().await;
		session.insert(SESSION_AUTH_ID_KEY, auth_id).unwrap();
		session.insert(SESSION_USER_ID_KEY, user_id).unwrap();
		self.current_user = Some(user.clone());
	}
	
	//		logout																
	pub async fn logout(&mut self) {
		let mut session = self.session_handle.write().await;
		session.destroy();
	}
}

#[async_trait]
impl<State> FromRequestParts<State> for AuthContext
where State: Send + Sync {
	type Rejection = std::convert::Infallible;
	
	//		from_request_parts													
	async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
		let Extension(auth_cx): Extension<AuthContext> =
			Extension::from_request_parts(parts, state)
				.await
				.expect("Auth extension/layer missing")
		;
		Ok(auth_cx)
	}
}



//		Functions

//		auth_layer																
pub async fn auth_layer<B>(
	State(appstate):           State<Arc<AppState>>,
	Extension(session_handle): Extension<SessionHandle>,
	mut request:               Request<B>,
	next:                      Next<B>,
) -> Response {
	let mut auth_cx      = AuthContext::new(session_handle, appstate.Key.clone());
	let user             = auth_cx.get_user(Arc::clone(&appstate)).await;
	let mut username     = "none".to_owned();
	if user.is_some() {
		username         = user.as_ref().unwrap().username.clone();
	}
	info!("Current user: {}", username);
	auth_cx.current_user = user;
	request.extensions_mut().insert(auth_cx.clone());
	request.extensions_mut().insert(auth_cx.current_user);
	next.run(request).await
}

//		protect																	
pub async fn protect<B>(
	State(appstate): State<Arc<AppState>>,
	Extension(user): Extension<Option<User>>,
	uri:             Uri,
	request:         Request<B>,
	next:            Next<B>,
) -> Response {
	match user {
		Some(_) => {
			//	let user = user.clone();
			//	request.extensions_mut().insert(user);
			next.run(request).await
		},
		_       => {
			(
				StatusCode::UNAUTHORIZED,
				get_login(State(appstate), uri).await,
			).into_response()
		},
	}
}

//		get_login																
pub async fn get_login(
	State(state): State<Arc<AppState>>,
	mut uri:      Uri,
) -> Html<String> {
	let mut params  = extract_uri_query_parts(uri.clone());
	let mut failed  = false;
	if params.contains_key("failed") {
		failed      = true;
		params.remove("failed");
	}
	uri             = build_uri(uri.path().to_string(), params);
	let mut context = Context::new();
	context.insert("PageURL", &uri.path_and_query().unwrap().to_string());
	context.insert("Failed",  &failed);
	Html(state.Template.render("login", &context).unwrap())
}

//		post_login																
pub async fn post_login(
	State(state): State<Arc<AppState>>,
	mut auth:     AuthContext,
	Form(login):  Form<PostLogin>,
) -> Redirect {
	let uri        = login.uri.parse::<Uri>().unwrap();
	let mut params = extract_uri_query_parts(uri.clone());
	let user       = User::find(Arc::clone(&state), &login.username, &login.password).await;
	if user.is_some() {
		info!("Logging in user: {}", user.as_ref().unwrap().username);
		auth.login(user.as_ref().unwrap()).await;
	} else {
		params.insert("failed".to_owned(), "".to_owned());
		info!("Failed login attempt for user: {}", &login.username);
	}
	Redirect::to(build_uri(uri.path().to_string(), params).path_and_query().unwrap().to_string().as_str())
}

//		get_logout																
pub async fn get_logout(
	mut auth: AuthContext,
) -> Redirect {
	if auth.current_user.is_some() {
		info!("Logging out user: {}", auth.current_user.as_ref().unwrap().username);
	}
	auth.logout().await;
	Redirect::to("/")
}


