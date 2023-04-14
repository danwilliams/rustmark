//		Packages

use axum::response::Html;



//		Functions

//		get_index																
pub async fn get_index() -> Html<String> {
	Html("Hello world!".to_owned())
}


