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
use nipper::{Document, Selection};
use tendril::StrTendril;



//		Functions

//		parse																	
/// Parse Markdown into HTML, extract metadata, and return the result.
/// 
/// # Parameters
/// 
/// * `markdown`     - The Markdown to parse.
/// * `remove_title` - Whether to remove the page title from the HTML.
/// 
/// # Returns
/// 
/// * `html`  - The HTML generated from parsing the Markdown.
/// * `toc`   - A table of contents based on headings found.
/// * `title` - The page title.
/// 
pub fn parse(markdown: &str, remove_title: bool) -> (String, Vec<(u8, String, String)>, StrTendril) {
	//		Parse Markdown														
	let adaptor     = SyntectAdapter::new("base16-ocean.dark");
	let mut plugins = ComrakPlugins::default();
	plugins.render.codefence_syntax_highlighter = Some(&adaptor);
	let html        = markdown_to_html_with_plugins(
		markdown,
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
	let document = Document::from(&html);
	let title    = find_title(&document, remove_title);
	let toc	     = find_headings(&document);
	process_details(&document.select("blockquote"));
	process_callouts(&document.select("blockquote"));
	process_headings(&document);
	(title, toc, document.html())
}

//		find_title																
/// Find the title of the page, and remove it from the document if requested.
/// 
/// The page title is the first `h1` element, but only if the first H1 is the
/// first element in the document. If any other content comes before it, it is
/// not counted as being the title.
/// 
/// If there is no matching H1, the title is "Untitled".
/// 
/// # Parameters
/// 
/// * `document`     - The HTML document tree to search for the title.
/// * `remove_title` - Whether to remove the page title from the document.
/// 
pub fn find_title(document: &Document, remove_title: bool) -> String {
	let mut title = document.select("h1:first-child").text().to_string();
	if title.is_empty() {
		title     = "Untitled".to_owned();
	}
	if remove_title {
		document.select("h1:first-child").remove();
	}
	title
}

//		find_headings															
/// Find all the headings in the document.
/// 
/// To make a table of contents, we need to find all the headings in order, and
/// then construct a hierarchy from them. That hierarchy can then be used to
/// represent the links in a table of contents, even though the list in the
/// document is most likely flat.
/// 
/// The hierarchy of headings found is represented by this function as a vector
/// of tuples, where each tuple contains the level of the heading, the ID of the
/// heading, and the text of the heading, rather than using a nested tree
/// structure.
/// 
/// # Parameters
/// 
/// * `document` - The HTML document tree to search for headings.
/// 
pub fn find_headings(document: &Document) -> Vec<(u8, String, String)> {
	let mut toc: Vec<(u8, String, String)> = vec![];
	for element in document.select("h1, h2, h3, h4, h5, h6").iter() {
		let node  = element.get(0).unwrap();
		let id    = element.select("a").attr("id").unwrap().to_string();
		let tag   = node.node_name().unwrap().to_string().to_lowercase();
		let text  = node.text().to_string();
		let level = tag.strip_prefix('h').unwrap().parse::<u8>().unwrap();
		toc.push((level, id, text));
	}
	toc
}

//		process_details															
/// Process all the details blocks in a selection of blockquotes.
/// 
/// The details blocks are used to create collapsible sections in the document.
/// They are created by using a blockquote with a paragraph that starts with
/// `->`. The paragraph is converted to a summary, and the blockquote is
/// converted to a details block.
/// 
/// # Parameters
/// 
/// * `blockquotes` - The selection of HTML blockquote elements to search for
///                   details blocks.
/// 
pub fn process_details(blockquotes: &Selection) {
	//	Find all blockquotes that match details syntax.
	for mut blockquote in blockquotes.iter() {
		let mut paragraph = blockquote.select("p:first-child").first();
		let para_text     = paragraph.text().to_string();
		if para_text.starts_with("->") {
			//	We need to specially process the contents of the blockquote, because the
			//	HTML will be rewritten, and so any references to nested blockquotes
			//	inside it that were found in the original selection will be orphaned and
			//	and will no longer be valid.
			process_details(&blockquote.select("blockquote"));
			let mut summary: Vec<String> =  vec![];
			let mut para_html            =  paragraph.html()
				.strip_prefix("<p>").unwrap()
				.strip_suffix("</p>").unwrap()
				.trim()
				.to_owned()
			;
			while para_html.starts_with("-&gt;") {
				match para_html.split_once('\n') { 
					Some((line, rest))   => {
						summary.push(line.strip_prefix("-&gt;").unwrap().trim().to_owned());
						para_html        =  rest.trim().to_owned();
					},
					None                 => {
						summary.push(para_html.strip_prefix("-&gt;").unwrap().trim().to_owned());
						para_html        =  "".to_owned();
					},
				}
			}
			//	This is somewhat yucky, but Nipper doesn't provide any way to access the
			//	inner HTML of an element, and the element children only provide access
			//	to the HTML nodes, not the text nodes.
			paragraph.replace_with_html(format!(
				r#"<summary>{}</summary><p>{}</p>"#,
				summary.join("\n"),
				para_html,
			));
			blockquote.replace_with_html(format!(
				r#"<details>{}</details>"#,
				blockquote.html()
					.strip_prefix("<blockquote>").unwrap()
					.strip_suffix("</blockquote>").unwrap()
				,
			));
		}
	}
}

//		process_callouts														
/// Process all the callouts in a selection of blockquotes.
/// 
/// The callouts are used to create attention-grabbing sections. They are
/// created by using a blockquote where the first paragraph contains a single
/// word as a `**strong**` element, which is then used as the class name for the
/// blockquote.
/// 
/// # Parameters
/// 
/// * `blockquotes` - The selection of HTML blockquote elements to search for
///                   callouts.
/// 
pub fn process_callouts(blockquotes: &Selection) {
	//	Find all blockquotes that match callout syntax.
	let mut toggle_count  = 0;
	for mut blockquote in blockquotes.iter() {
		let mut paragraph = blockquote.select("p:first-child").first();
		let mut strong    = paragraph.select("strong:first-child").first();
		let para_text     = paragraph.text().to_string();
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
			r#"<input class="toggle" id="toggle-c"#,  toggle_count, r#"" type="checkbox" />"#,
			r#"<label class="toggle" for="toggle-c"#, toggle_count, r#"">"#,
			r#"<i class="toggle"></i>"#,
			r#"</label>"#,
			strong.html(),
		);
		let para_html: String;
		if para_text.strip_prefix(&strong_text).unwrap().starts_with(':') {
			//	There doesn't seem to be a better way of removing just the text from the
			//	paragraph, and the paragraph may contain other elements and not just
			//	text, so those need to be preserved.
			strong.replace_with_html(strong_html);
			para_html     = paragraph.html().to_string();
			paragraph.remove();
		} else {
			strong.remove();
			para_html     = format!(r#"<p>{}</p>"#, strong_html);
		}
		blockquote.set_html(format!(
			r#"{}<div class="collapsible">{}</div>"#,
			para_html,
			blockquote.children().iter()
				.map(|c| c.html().to_string())
				.collect::<Vec<String>>()
				.join("\n")
			,
		));
		//	We need to specially process the contents of the blockquote, because the
		//	HTML has been rewritten, and so any references to nested blockquotes
		//	inside it that were found in the original selection have been orphaned
		//	and are no longer valid.
		process_callouts(&blockquote.select("blockquote"));
	}
}

//		process_headings														
/// Process all the headings in the document and make them collapsible.
/// 
/// Two things are assumed: first, that all headings are top-level elements in
/// the HTML generated from the Markdown; and second, that there is only one H1
/// element in the document, which serves as the page title.
/// 
/// The first assumption seems reasonable, as there doesn't seem to be a valid
/// way to end up with a heading nested inside another element. The second
/// assumption is not guaranteed, but it's the way we're advising people to
/// structure their Markdown files.
/// 
/// All headings found in the document are converted to collapsible sections,
/// with the heading text as the summary and the heading content as the
/// collapsible content.
/// 
/// # Parameters
/// 
/// * `document` - The HTML document tree to search for headings.
/// 
pub fn process_headings(document: &Document) {
	let mut headings         = vec!["h2", "h3", "h4", "h5", "h6"];
	let mut toggle_count     = 0;
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
							r#"<input class="toggle" id="toggle-h"#,  toggle_count, r#"" type="checkbox" />"#,
							r#"<label class="toggle" for="toggle-h"#, toggle_count, r#"">"#,
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
}


