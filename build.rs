//		Modules

#[path = "src/lib.rs"]
mod rustmark;



//		Packages

use serde_json::{self};
use std::{
	env,
	fs::{File, self},
	io::prelude::*,
	path::Path,
	time,
};
use tokio::{self};
use walkdir::WalkDir;



//		Functions

//		main																	
#[tokio::main]
async fn main() {
	println!("cargo:rerun-if-changed=content");
	//	We use unwrap throughout because this is a build script, and if there
	//	are any errors, we want the build to fail and for us to see the error.
	let env_out_dir = env::var("OUT_DIR").unwrap();
	let input_root  = Path::new("content");
	let output_root = Path::new(&env_out_dir);
	let mut tasks   = vec![];
	
	//		Traverse output directory											
	//	We do this first so that we can delete any files that are no longer
	//	present in the input directory.
	for output_path in WalkDir::new(output_root).follow_links(true) {
		let output_path = output_path.unwrap().path().to_path_buf();
		let input_path  = input_root.join(output_path.strip_prefix(output_root).unwrap());
		if output_path == output_root || !output_path.exists() {
			continue;
		}
		//		Delete things that no longer exist in the input directory		
		if
			!input_path.exists()
		||	(input_path.is_dir()  && !output_path.is_dir())
		||	(input_path.is_file() && !output_path.is_file())
		{
			if output_path.is_dir() {
				println!("Deleting directory: {}", output_path.display());
				fs::remove_dir_all(&output_path).unwrap();
			}
			if output_path.is_file() {
				println!("Deleting file: {}", output_path.display());
				fs::remove_file(&output_path).unwrap();
			}
			continue;
		}
	}
	
	//		Traverse input directory											
	for input_path in WalkDir::new(input_root).follow_links(true) {
		let input_path  = input_path.unwrap().path().to_path_buf();
		let output_path = output_root.join(input_path.strip_prefix(input_root).unwrap());
		println!("Found: {}", input_path.display());
		if input_path == input_root {
			continue;
		}
		//		Create directories												
		//	We don't create the directories as an async process for two reasons:
		//	first, there's no real advantage in doing so; and secondly, we want to
		//	avoid any collisions that might occur if we try to create the same
		//	directory from two different async tasks at the same time.
		if input_path.is_dir() {
			if !output_path.exists() {
				println!("Creating directory: {}", output_path.display());
				fs::create_dir_all(output_path).unwrap();
			}
			continue;
		}
		//		Compare timestamps												
		if output_path.exists() {
			let input_mtime  = fs::metadata(&input_path).unwrap()
				.modified().unwrap()
				.duration_since(time::UNIX_EPOCH).unwrap()
				.as_secs()
			;
			let output_mtime = fs::metadata(&output_path).unwrap()
				.modified().unwrap()
				.duration_since(time::UNIX_EPOCH).unwrap()
				.as_secs()
			;
			if input_mtime < output_mtime {
				println!("Skipping file: {}", input_path.display());
				continue;
			}
		}
		//		Handle files													
		//	We spawn a new task for each file, so that we can process them in
		//	parallel to whatever degree is allowed by the runtime.
		let task = tokio::spawn(async move {
			if input_path.extension().is_some() && input_path.extension().unwrap() == "md" {
				parse(&input_path, &output_path).await;
			} else {
				copy(&input_path, &output_path).await;
			}
		});
		tasks.push(task);
	}
	//	Wait for all tasks to finish
	for task in tasks {
		task.await.unwrap();
	}
}

//		copy																	
async fn copy(input_path: &Path, output_path: &Path) {
	//	We ideally want to use hardlinks here in order to save space, but when
	//	they were used, problems were found whereby the input files were getting
	//	truncated.
	println!("Copying file: {} -> {}", input_path.display(), output_path.display());
	fs::copy(input_path, output_path).unwrap();
}

//		parse																	
async fn parse(input_path: &Path, output_path: &Path) {
	println!("Parsing file: {}", input_path.display());
	let (title, toc, html) = rustmark::parse(
		&fs::read_to_string(input_path).unwrap(),
		input_path,
	);
	//	We use a custom format - the first line of the file is the title we
	//	extracted, the second line is a JSON array with the table of contents,
	//	and the rest is the HTML.
	let mut output_file = File::create(output_path).unwrap();
	output_file.write_all(format!("{}\n", &title).as_bytes()).unwrap();
	output_file.write_all(format!("{}\n", serde_json::to_string(&toc).unwrap()).as_bytes()).unwrap();
	output_file.write_all(html.as_bytes()).unwrap();
}


