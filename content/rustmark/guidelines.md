# Guidelines

This document provides some guidelines for best practices to follow when writing
Markdown files for use with Rustmark, and where to place supporting files such
as images and files for download.


## Top-level headings

Following common convention, you should structure your Markdown files so that
there is a single top-level heading (`h1`) on the first line, and that all other
headings are sub-headings of this. The rationale is that a top-level heading on
the first line is the title of the document, and there should only be one title
per document.


## Indentation and alignment

Markdown files should always use spaces, and not tabs, for indentation and
elsewhere. This is because the various Markdown parsers and renderers may have
different ideas about how many spaces a tab should be, and it is not possible to
reliably render tabs in a browser. Spaces are therefore the best and safest
option for compatibility.

You should also ensure that your Markdown files hard-wrap at 80 columns. This is
comfortable to read when editing the source text. The exception is links: as a
general rule, the text of a link should fit within the 80-column limit, but the
URL may extend beyond it. If the URL extends beyond 120 characters, the link
should be moved to the next line.


## Editing

Generally speaking, it is good practice to avoid reflowing text when editing
Markdown files. This is because it makes it harder to see what has changed in
Git diffs. This may well lead to situations where there are some short lines,
but this is preferable to reflowing text as it is easy to see what has changed.
As the final reading medium is the rendered output, the readability of the
source text is a secondary concern.

From time to time it may be desirable to reflow paragraphs, in which case such
actions are usually carried out in a separate commit to the actual content
changes.


## Structure

Markdown files and other content, such as images and files for download, should
be placed in the `content` directory. The Markdown files will be rendered as
HTML, and other files will be served directly. All files placed in the `content`
directory will be served from the root URL path, and will be protected by the
application's authentication layer, just like the rendered Markdown pages. This
allows for relative URLs to be used from the Markdown files, making it easier to
author them and increasing compatibility with other systems such as previews in
GitHub, Gitea, etc. This benefit outweighs the slight inconvenience of having
other files mixed in with the Markdown files.

The exact structure of the `content` directory is not important, but it is
recommended to create subdirectories for images and other files, to maintain
clarity through separation of concerns, and to do so relative to the Markdown
files that use them. This can be seen in this example, where the images are
placed in the `content/rustmark/images` directory, which is then accessible from
this Markdown file both when previewing using a Git server and also when
properly rendered and served using Rustmark in the same way, using relative URLs
such as `images/image.png`.

Any images placed in the `static/img` directory will be publicly available
without needing authentication. This is useful for logos and other files that
may be used to customise the application. It is possible to encounter name
clashes between files in `content` and `static` if the same root folder names
are used, which is why images in `static` are placed in a directory called `img`
rather than `images`. Although it could be desirable to allow files in one
location to override the other, the Axum router does not allow multiple handlers
to process the same route, so this is not possible without moving away from that
router. This may be done in future, as the application routing is very simple,
but for now it is not a requirement.


