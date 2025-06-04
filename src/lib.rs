//! Rustmark
//!
//! Extensible web application for serving Markdown-based content.
//!



//		Global configuration

//	Customisations of the standard linting configuration
#![allow(clippy::doc_markdown,            reason = "Too many false positives")]
#![allow(clippy::multiple_crate_versions, reason = "Cannot resolve all these")]



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
use rubedo::sugar::s;
use serde::{Deserialize, Serialize};
use tendril::StrTendril;



//		Structs

//		Heading																	
/// A heading extracted from Markdown.
#[derive(Debug, Deserialize, Serialize)]
pub struct Heading {
	/// The level of the heading. This can be 1-6.
	level: u8,
	
	/// The HTML id attribute of the heading.
	id:    String,
	
	/// The text of the heading.
	text:  String,
}



//		Functions

//		parse																	
/// Parses Markdown into HTML, extract metadata, and return the result.
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
#[must_use]
pub fn parse(markdown: &str, remove_title: bool) -> (String, Vec<Heading>, StrTendril) {
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
				header_ids:                Some(s!("")),
				footnotes:                 true,
				description_lists:         true,
				front_matter_delimiter:    Some(s!("---")),
				shortcodes:                true,
			},
			parse:                         ComrakParseOptions {
				smart:                     true,
				default_info_string:       Some(s!("")),
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
/// Finds the title of the page, and remove it from the document if requested.
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
		"Untitled".clone_into(&mut title);
	}
	if remove_title {
		document.select("h1:first-child").remove();
	}
	title
}

//		find_headings															
/// Finds all the headings in the document.
/// 
/// To make a table of contents, we need to find all the headings in order, and
/// then construct a hierarchy from them. That hierarchy can then be used to
/// represent the links in a table of contents, even though the list in the
/// document is most likely flat.
/// 
/// The hierarchy of headings found is represented by this function as a vector
/// of Heading structs, where each struct contains the level of the heading, the
/// ID of the heading, and the text of the heading, rather than using a nested
/// tree structure.
/// 
/// # Parameters
/// 
/// * `document` - The HTML document tree to search for headings.
/// 
pub fn find_headings(document: &Document) -> Vec<Heading> {
	let mut toc: Vec<Heading> = vec![];
	for element in document.select("h1, h2, h3, h4, h5, h6").iter() {
		let Some(node) = element.get(0) else {
			continue;
		};
		let Some(id) = element.select("a").attr("id").map(|s| s.to_string()) else {
			continue;
		};
		let Some(tag) = node.node_name().map(|s| s.to_string().to_lowercase()) else {
			continue;
		};
		let Some(level) = tag.strip_prefix('h').map(|s| s.parse::<u8>().unwrap_or(6)) else {
			continue;
		};
		let text = node.text().to_string();
		toc.push(Heading { level, id, text });
	}
	toc
}

//		process_details															
/// Processes all the details blocks in a selection of blockquotes.
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
#[allow(clippy::allow_attributes,   reason = "using expect below doesn't work")]
#[allow(clippy::missing_panics_doc, reason = "Infallible")]
pub fn process_details(blockquotes: &Selection<'_>) {
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
			let mut summary         = vec![];
			let Some(mut para_html) = paragraph.html()
				.strip_prefix("<p>")
				.and_then(|s| s.strip_suffix("</p>"))
				.map(|s| s.trim().to_owned())
			else {
				continue;
			};
			#[expect(clippy::unwrap_used, reason = "The prefix is checked in the loop and so must be present")]
			while para_html.starts_with("-&gt;") {
				if let Some((line, rest)) = para_html.split_once('\n') {
					summary.push(line.strip_prefix("-&gt;").unwrap().trim().to_owned());
					para_html = rest.trim().to_owned();
				} else {
					summary.push(para_html.strip_prefix("-&gt;").unwrap().trim().to_owned());
					para_html = String::new();
				}
			}
			//	This is somewhat yucky, but Nipper doesn't provide any way to access the
			//	inner HTML of an element, and the element children only provide access
			//	to the HTML nodes, not the text nodes.
			paragraph.replace_with_html(format!(
				r"<summary>{}</summary><p>{para_html}</p>",
				summary.join("\n"),
			));
			if let Some(content) = blockquote.html()
				.strip_prefix("<blockquote>")
				.and_then(|s| s.strip_suffix("</blockquote>"))
			{
				blockquote.replace_with_html(format!(r"<details>{content}</details>"));
			}
		}
	}
}

//		process_callouts														
/// Processes all the callouts in a selection of blockquotes.
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
pub fn process_callouts(blockquotes: &Selection<'_>) {
	//	Find all blockquotes that match callout syntax.
	for mut blockquote in blockquotes.iter() {
		let mut paragraph = blockquote.select("p:first-child").first();
		let mut strong    = paragraph.select("strong:first-child").first();
		let para_text     = paragraph.text().to_string();
		let strong_text   = strong.text().to_string();
		if strong_text.is_empty() || strong_text.contains(' ') {
			continue;
		}
		let class         = strong_text.replace(|c: char| !c.is_alphanumeric(), "").to_lowercase();
		blockquote.add_class("callout");
		blockquote.add_class(&class);
		let para_html: String;
		if para_text
			.strip_prefix(&strong_text)
			.is_some_and(|stripped| stripped.starts_with(':'))
		{
			//	There doesn't seem to be a better way of removing just the text from the
			//	paragraph, and the paragraph may contain other elements and not just
			//	text, so those need to be preserved.
			para_html = paragraph.html()
				.strip_prefix("<p>")
				.and_then(|s| s.strip_suffix("</p>"))
				.map_or_else(|| paragraph.html().to_string(), ToOwned::to_owned)
			;
			paragraph.remove();
		} else {
			para_html     = strong.html().to_string();
			strong.remove();
		}
		let open          = !["image", "images", "screenshot", "screenshots"].contains(&&*class);
		let mut chld_html = blockquote.children().iter()
			.map(|c| c.html().to_string())
			.collect::<Vec<String>>()
			.join("\n")
		;
		chld_html
			.replace("<p></p>", "")
			.trim()
			.clone_into(&mut chld_html)
		;
		blockquote.set_html(
			if chld_html.is_empty() {
				format!(r"<p>{para_html}</p>")
			} else {
				format!(
					r#"<details {} class="callout-collapse"><summary>{para_html}</summary>{chld_html}</details>"#,
					if open { "open" } else { "" },
				)
			}
		);
		//	We need to specially process the contents of the blockquote, because the
		//	HTML has been rewritten, and so any references to nested blockquotes
		//	inside it that were found in the original selection have been orphaned
		//	and are no longer valid.
		process_callouts(&blockquote.select("blockquote"));
	}
}

//		process_headings														
/// Processes all the headings in the document and make them collapsible.
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
	loop {
		let Some(heading_tag) = headings.last().map(ToOwned::to_owned) else {
			continue;
		};
		let mut heading_html = String::new();
		let mut buffer_html  = String::new();
		let mut active       = false;
		let mut elements     = document.select("body > *").iter().enumerate().peekable();
		while let Some((_, mut element)) = elements.next() {
			let next_element = elements.peek().cloned();
			let next_tag     = next_element.clone().map_or_else(|| s!(""), |(_, el)|
				el.get(0).map_or_else(|| s!(""), |node|
					node.node_name().map_or_else(|| s!(""), |name| name.to_string().to_lowercase())
				),
			);
			if let Some(node) = element.get(0) {
				if node.node_name().is_some() {
					let Some(tag) = node.node_name().map(|s| s.to_string().to_lowercase()) else {
						continue;
					};
					if !active && tag == heading_tag {
						active        = true;
						heading_html  = element.html().to_string();
						element.remove();
						continue;
					}
				}
			}
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
					r#"<details open class="heading-collapse {heading_tag}"><summary>{heading_html}</summary>{buffer_html}</details>"#,
				));
				active       = false;
				heading_html = String::new();
				buffer_html  = String::new();
				continue;
			}
			element.remove();
		}
		_ = headings.pop();
		if headings.is_empty() {
			break;
		}
	}
}


