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
use nipper::Document;
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
	//		Parse Markdown														
	println!("Parsing file: {}", input_path.display());
	let adaptor     = SyntectAdapter::new("base16-ocean.dark");
	let mut plugins = ComrakPlugins::default();
	plugins.render.codefence_syntax_highlighter = Some(&adaptor);
	let html        = markdown_to_html_with_plugins(
		&fs::read_to_string(input_path).unwrap(),
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
	//		Find page title														
	//	We'll use the first h1 element as the title of the page, but only if the
	//	first H1 is the first element in the document. If any other content
	//	comes before it, we won't count it as being the title.
	let document  = Document::from(&html);
	let mut title = document.select("h1:first-child").text().to_string();
	if title.is_empty() {
		title     = "Untitled".to_owned();
	}
	//	Remove the title from the index page, as it will have one added showing
	//	the application title.
	if input_path == Path::new("content/index.md") {
		document.select("h1:first-child").remove();
	}
	//		Process callouts													
	//	Find all blockquotes that match callout syntax.
	let mut toggle_count  = 0;
	for mut blockquote in document.select("blockquote").iter() {
		let mut strong    = blockquote.select("strong:first-child").first();
		let strong_text   = strong.text().to_string();
		if strong_text.is_empty() || strong_text.contains(' ') {
			continue;
		}
		let class         = strong_text.replace(|c: char| !c.is_alphanumeric(), "").to_lowercase();
		blockquote.add_class(&class);
		if !vec!["image", "images", "screenshot", "screenshots"].contains(&&*class) {
			continue;
		}
		toggle_count     += 1;
		let strong_html   = format!(
			"{}{}{}{}{}{}{}{}{}",
			r#"<input class="toggle" id="toggle-"#,  toggle_count, r#"" type="checkbox" />"#,
			r#"<label class="toggle" for="toggle-"#, toggle_count, r#"">"#,
			r#"<i class="toggle"></i>"#,
			r#"</label>"#,
			strong.html(),
		);
		strong.remove();
		blockquote.set_html(format!(
			r#"<p>{}</p><div class="collapsible">{}</div>"#,
			strong_html,
			blockquote.children().iter().map(|c| c.html().to_string()).collect::<Vec<String>>().join("\n"),
		));
	}
	//		Make all headings collapsible										
	//	We'll assume two things: first, that all headings are top-level elements
	//	in the HTML generated from the Markdown; and second, that there is only
	//	one H1 element in the document, which serves as the page title. The
	//	first assumption seems reasonable, as there doesn't seem to be a valid
	//	way to end up with a heading nested inside another element. The second
	//	assumption is not guaranteed, but it's the way we're advising people to
	//	structure their Markdown files.
	let mut headings         = vec!["h2", "h3", "h4", "h5", "h6"];
	loop {
		let heading_tag      = headings.last().unwrap().to_owned();
		let mut heading_html = String::new();
		let mut buffer_html  = String::new();
		let mut active       = false;
		let mut elements     = document.select("body > *").iter().enumerate().peekable();
		while let Some((_, mut element)) = elements.next() {
			let next_element = elements.peek().cloned();
			let next_tag     = match &next_element {
				None               => "".to_owned(),
				Some((_, element)) => match element.get(0) {
					None           => "".to_owned(),
					Some(node)     => {
						if node.node_name().is_none() {
							"".to_owned()
						} else {
							node.node_name().unwrap().to_string().to_lowercase()
						}
					},
				},
			};
			if let Some(node) = element.get(0) {
				if node.node_name().is_some() {
					let tag   = node.node_name().unwrap().to_string().to_lowercase();
					if !active && tag == heading_tag {
						active        = true;
						toggle_count += 1;
						element.append_html(format!(
							"{}{}{}{}{}{}{}{}",
							r#"<input class="toggle" id="toggle-"#,  toggle_count, r#"" type="checkbox" />"#,
							r#"<label class="toggle" for="toggle-"#, toggle_count, r#"">"#,
							r#"<i class="toggle"></i>"#,
							r#"</label>"#,
						));
						heading_html.push_str(&element.html());
						element.remove();
						continue;
					}
				}
			};
			if !active {
				continue;
			}
			buffer_html.push_str(&element.html());
			if
				(!next_tag.is_empty() && headings.contains(&&*next_tag))
			||	next_tag == "section"
			||	next_element.is_none()
			{
				element.replace_with_html(format!(
					r#"{}<div class="collapsible">{}</div>"#,
					heading_html,
					buffer_html,
				));
				active       = false;
				heading_html = String::new();
				buffer_html  = String::new();
				continue;
			}
			element.remove();
		}
		headings.pop();
		if headings.is_empty() {
			break;
		}
	}
	//		Write output														
	//	We use a custom format - the first line of the file is the title we
	//	extracted, and the rest is the HTML.
	let mut output_file = File::create(output_path).unwrap();
	output_file.write_all(format!("{}\n", &title).as_bytes()).unwrap();
	output_file.write_all(document.html().as_bytes()).unwrap();
}


