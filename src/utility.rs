#![allow(non_snake_case)]

//		Packages

use serde::Deserialize;



//		Structs

//		Config																	
#[derive(Deserialize)]
pub struct Config {
	pub port: u16,
}

//		AppState																
#[allow(dead_code)]
pub struct AppState {
	pub Config: Config,
}


