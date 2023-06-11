//		Packages

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
use scraper::{Html, Selector};
use std::{
	env,
	fs::{File, self},
	io::prelude::*,
	path::Path,
};
use walkdir::WalkDir;



//		Functions

//		main																	
fn main() {
	let env_out_dir = env::var("OUT_DIR").unwrap();
	let input_root  = Path::new("content");
	let output_root = Path::new(&env_out_dir);
	
	for input_path in WalkDir::new(input_root).follow_links(true) {
		//	This uses unwrap because this is a build script, and if there are any
		//	errors, we want the build to fail and for us to see the error.
		let input_path  = input_path.unwrap().path().to_path_buf();
		let output_path = output_root.join(input_path.strip_prefix(input_root).unwrap());
		println!("Found: {}", input_path.display());
		if input_path == input_root {
			continue;
		}
		//		Create directories												
		if input_path.is_dir() {
			if !output_path.exists() {
				println!("Creating directory: {}", output_path.display());
				fs::create_dir_all(output_path).unwrap();
			}
			continue;
		}
		//		Copy non-Markdown files											
		if input_path.extension().is_none() || input_path.extension().unwrap() != "md" {
			//	Normally we might want to use hardlinks in order to save space, but the
			//	way that cargo build works is to create a new directory every time. It
			//	might therefore be confusing to use hardlinks given that changes would
			//	then pollute the build directories, negating Cargo's intent of being
			//	able to look at any build directory to see exactly what was built for
			//	that particular build.
			println!("Copying file: {} -> {}", input_path.display(), output_path.display());
			fs::copy(input_path, output_path).unwrap();
			continue;
		}
		//		Parse Markdown files											
		//	Under different circumstances, i.e. in other build systems, we might
		//	want to keep track of file modification times, and only re-parse files
		//	that have been modified since the last build. However, cargo build
		//	always creates a new build directory, so we have to re-parse everything.
		//	Although this does take more time, it fits with Cargo's intent of being
		//	able to look at any build directory and see exactly what was built for
		//	that particular build.
		println!("Parsing file: {}", input_path.display());
		let adaptor     = SyntectAdapter::new("base16-ocean.dark");
		let mut plugins = ComrakPlugins::default();
		plugins.render.codefence_syntax_highlighter = Some(&adaptor);
		let html        = markdown_to_html_with_plugins(
			&fs::read_to_string(&input_path).unwrap(),
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
		);
		//		Interrogate HTML												
		//	We'll use the first h1 element as the title of the page, but only if the
		//	first H1 is the first element in the document. If any other content
		//	comes before it, we won't count it as being the title.
		let mut document =  Html::parse_document(&html);
		let (title, id)  =  match document.select(&Selector::parse("h1:first-child").unwrap()).next() {
			Some(h1)     => (h1.text().collect::<String>(), Some(h1.id())),
			None         => ("Untitled".to_owned(), None),
		};
		//	Remove the title from the index page, as it will have one added showing
		//	the application title.
		if input_path == Path::new("content/index.md") {
			if let Some(id) = id { document.tree.get_mut(id).unwrap().detach() }
		}
		//		Write output													
		//	We use a custom format - the first line of the file is the title we
		//	extracted, and the rest is the HTML.
		let mut output_file = File::create(&output_path).unwrap();
		output_file.write_all(format!("{}\n", &title).as_bytes()).unwrap();
		output_file.write_all(document.html().as_bytes()).unwrap();
	}
}


