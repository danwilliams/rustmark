//		Packages

use crate::{
	ASSETS_DIR,
	MARKDOWN_DIR,
	utility::*,
};
use axum::{
	body,
	extract::State,
	http::{HeaderValue, StatusCode, Uri, header},
	response::{Html, IntoResponse, Response},
};
use comrak::{
	ComrakOptions,
	ComrakExtensionOptions,
	ComrakParseOptions,
	ComrakRenderOptions,
	ComrakPlugins,
	ListStyleType,
	markdown_to_html_with_plugins,
	plugins::syntect::SyntectAdapter,
};
use mime_guess::{self};
use std::sync::Arc;
use tera::Context;



//		Functions

//		get_index																
pub async fn get_index(State(state): State<Arc<AppState>>) -> Html<String> {
	let mut context = Context::new();
	context.insert("Title",   &state.Config.title);
	context.insert("Content", "Index");
	Html(state.Template.render("index", &context).unwrap())
}

//		get_page																
pub async fn get_page(
	State(state): State<Arc<AppState>>,
	uri: Uri,
) -> impl IntoResponse {
	let path       =  uri.path().trim_start_matches('/');
	match MARKDOWN_DIR.get_file(path) {
		None       => (StatusCode::NOT_FOUND).into_response(),
		Some(file) => {
			let adaptor     = SyntectAdapter::new("base16-ocean.dark");
			let mut plugins = ComrakPlugins::default();
			plugins.render.codefence_syntax_highlighter = Some(&adaptor);
			let mut context = Context::new();
			context.insert("Title",   &path);
			context.insert("Content", &markdown_to_html_with_plugins(
				file.contents_utf8().unwrap(),
				&ComrakOptions {
					extension:                     ComrakExtensionOptions {
						strikethrough:             true,
						tagfilter:                 true,
						table:                     true,
						autolink:                  true,
						tasklist:                  true,
						superscript:               true,
						header_ids:                Some("".to_owned()),
						footnotes:                 true,
						description_lists:         true,
						front_matter_delimiter:    Some("---".to_owned()),
						shortcodes:                true,
					},
					parse:                         ComrakParseOptions {
						smart:                     true,
						default_info_string:       Some("".to_owned()),
						relaxed_tasklist_matching: true,
					},
					render:                        ComrakRenderOptions {
						hardbreaks:                false,
						github_pre_lang:           false,
						full_info_string:          true,
						width:                     80,
						unsafe_:                   true,
						escape:                    false,
						list_style:                ListStyleType::Dash,
						sourcepos:                 false,
					},
				},
				&plugins,
			));
			(
				StatusCode::OK,
				Html(state.Template.render("page", &context).unwrap()),
			).into_response()
		},
	}
}

//		get_static_asset														
pub async fn get_static_asset(uri: Uri) -> impl IntoResponse {
	let path       =  uri.path().trim_start_matches('/');
	let mime_type  =  mime_guess::from_path(path).first_or_text_plain();
	match ASSETS_DIR.get_file(path) {
		None       => Response::builder()
			.status(StatusCode::NOT_FOUND)
			.body(body::boxed(body::Empty::new()))
			.unwrap()
		,
		Some(file) => Response::builder()
			.status(StatusCode::OK)
			.header(
				header::CONTENT_TYPE,
				HeaderValue::from_str(mime_type.as_ref()).unwrap(),
			)
			.body(body::boxed(body::Full::from(file.contents())))
			.unwrap()
		,
	}
}


