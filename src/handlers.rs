//		Packages

use crate::{
	utility::*,
};
use axum::{
	extract::State,
	response::Html,
};
use std::sync::Arc;
use tera::Context;



//		Functions

//		get_index																
pub async fn get_index(State(state): State<Arc<AppState>>) -> Html<String> {
	let mut context = Context::new();
	context.insert("Title",   "Terracotta");
	context.insert("Content", "Index");
	Html(state.Template.render("index", &context).unwrap())
}



