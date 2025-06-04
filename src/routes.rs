//! Routes for the application.



//		Packages																										

use crate::{
	auth::{Credentials, User},
	handlers::{get_index, get_page},
	state::AppState,
};
use axum::routing::{MethodRouter, get, post};
use std::sync::Arc;
use terracotta::{
	assets::handlers::get_public_static_asset,
	auth::handlers::{get_logout, post_login},
	health::handlers::{get_ping, get_version},
	stats::handlers::{get_stats, get_stats_feed, get_stats_history},
};



//		Functions																										

//		protected																
/// Returns a list of protected routes.
pub fn protected() -> Vec<(&'static str, MethodRouter<Arc<AppState>>)> {
	vec![
		("/",      get(get_index)),
		("/*path", get(get_page))    //  Also handles get_protected_static_asset(uri)
	]
}

//		public																	
/// Returns a list of public routes.
pub fn public() -> Vec<(&'static str, MethodRouter<Arc<AppState>>)> {
	vec![
		("/api/ping",          get(get_ping)),
		("/api/version",       get(get_version)),
		("/api/stats",         get(get_stats)),
		("/api/stats/history", get(get_stats_history)),
		("/api/stats/feed",    get(get_stats_feed)),
		("/login",             post(post_login::<_, Credentials, User, User>)),
		("/logout",            get(get_logout::<User>)),
		("/css/*path",         get(get_public_static_asset)),
		("/img/*path",         get(get_public_static_asset)),
		("/js/*path",          get(get_public_static_asset)),
		("/webfonts/*path",    get(get_public_static_asset)),
	]
}


