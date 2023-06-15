# Rustmark

Rustmark is a simple Markdown server written in [Rust](https://www.rust-lang.org/).
It is intended to be easy to use, easy to fork and customise, and easy to
deploy.

The main sections in this README are:

  - [Content](#content)
  - [Features](#features)
  - [Why Rustmark?](#why-rustmark)
  - [Setup](#setup)
  - [Usage](#usage)
  - [Deployment](#deployment)
  - [Legal](#legal)


## Content

The entry point for reading the Markdown files is the [index page](content/index.md).
If you are here to read the content (for instance if you are using a Git system
to preview the Markdown, such as GitHub or Gitea), start there.

An example showing available [Markdown features](content/rustmark/features.md)
is also
provided, along with some [Guidelines](content/rustmark/guidelines.md) for use.


## Features

The main high-level points of note are:

  - **Markdown**
      - Rendering of Markdown files using [Comrak](https://crates.io/crates/comrak)
      - Syntax highlighting using [Syntect](https://crates.io/crates/syntect)
      - Full compliance with [CommonMark](https://commonmark.org/) [0.30](https://spec.commonmark.org/0.30/)
      - Support for [GitHub-flavoured Markdown](https://github.github.com/gfm/)
      - Tables, task lists, strikethrough, and autolinks (from [GFM](https://github.github.com/gfm/))
      - Superscript, footnotes, description lists, and emoji shortcodes
      - Callouts, based on extended blockquotes
  - **Display**
      - CSS foundation using the [Bulma](https://bulma.io/) CSS framework
      - Icons using [Font Awesome](https://fontawesome.com/)
      - Twitter emojis using [Twemoji](https://twemoji.twitter.com/)
      - Code displayed using [Nerd Fonts](https://www.nerdfonts.com/)
      - Collapsible callouts for screenshots and other large images
      - Collapsible document sections based on headings (automatic)
      - Linkable headings
      - Automatic per-page table of contents in navigation menu
  - **Build**
      - HTML generated from Markdown asynchronously at build time and included
        in binary
      - Efficient rebuild process regenerating only changed files
  - **Customisation**
      - Custom JS and CSS files for customisation overrides
      - Templates implemented using the [Tera](https://crates.io/crates/tera)
        template engine
      - Configuration from config file and env vars using [Figment](https://crates.io/crates/figment)
      - Simple codebase layout
      - Easy to extend and build upon
  - **Security**
      - Simple authentication using sessions and config-based user list
      - Login page, public and protected routes, logout ability
      - Protected static content files for use alongside Markdown content
  - **Performance**
      - High-performance asynchronous HTTP server using [Tokio Hyper](https://crates.io/crates/hyper)
      - Based on the robust and ergonomic web framework [Axum](https://crates.io/crates/axum)
  - **Other**
      - Compatibility with browsing the content on a Git server such as GitHub
        or Gitea
      - Static file handling
      - Single-file deployment — all assets baked in
      - Logging of HTTP requests and events using [Tokio Tracing](https://crates.io/crates/tracing)
      - Graceful handling of 404 and 500 HTTP errors
      - Graceful handling of runtime application errors

More details are available about the [supported Markdown features](content/rustmark/features.md),
along with examples.

### Authentication

Rustmark features [Terracotta](https://crates.io/crates/terracotta)'s
custom-rolled authentication system, providing a basic session-based setup.
However, it is highly recommended to store the user credentials securely in a
database. That is currently outside the scope of this project, for a number of
reasons, primarily the ambition to provide a simple system that can be extended
to use any database required. You will probably also want to store the sessions
in a database instead of in memory.

The authentication system is set up to make it easy to configure routes as
either public or protected, and is fully-implemented including a login page,
logout action, and handling of every part of the authentication journey and the
possible situations.

### Databases

Rustmark very purposefully does not include any kind of database integration.
There are so many, and such a plethora of crates to choose from, that this is
best left open. Database interaction is very straightforward and so this is a
simple addition to make.


## Why Rustmark?

The intention is to provide a simple system that is easy to maintain, and
focused on developer documentation - or at least, documentation written by
people who are comfortable working in Markdown with text editors, and committing
their changes to a Git repository.

There are many tools available to provide wiki-style functionality in very
friendly ways, the leading product (arguably) being [Notion](https://notion.so).
[Coda](https://coda.io) also deserves an honourable mention, but
[Confluence](https://www.atlassian.com/software/confluence), although widely
used due to the popularity of Jira, lags a long way behind.

### So why not just use Notion?

Notion is a great product, with some excellent and powerful features, and it is
also very easy to use. However, it does not feature any kind of approval
process, meaning that it is all too easy for incorrect information to creep in -
and the more pages there are to keep an eye on, the worse this problem becomes.
Although Notion does have page edit history, and change alerts, it is not
possible to require that changes are approved before they are published. Locking
edit access is not really a solution, as it would prevent the very people who
are most likely to need to make changes from doing so, and focus the work on a
small number of specific people.

Therefore, the primary purpose of Rustmark is to provide a simple, easy-to-use
system for publishing developer-focused documentation, with a simple approval
process managed through standard Git pull requests. It is not intended to be a
replacement for Notion, or any other wiki-style system, and it is quite likely
best to use both in tandem, with Notion for general documentation and pages that
non-techies need to edit.

### If it works with Git server Markdown previewing, why not just use that?

Most Git server systems provide a Markdown preview feature, which is great for
those people that have access to Git. But what if the documentation needs to be
accessible to people who do not have access to the Git server? Although Rustmark
is aimed at developers, that particular focus is in the context of editing. It
also needs to be accessible to non-developers - plus, browsing pages on a Git
server is not always the most user-friendly experience.

Additionally, this approach allows for a lot of flexibility in terms of styling
and presentation, customisation, and hosting.

### What about GitHub Pages?

[GitHub Pages](https://pages.github.com) is a great way to host static content,
and it is very easy to use. However, not everyone uses or wants to use GitHub,
and there are constraints on the free accounts that may not make it the ideal
choice for some people. There are also limitations on the amount of
customisation that can be performed, and it is not possible to do anything
dynamic, as ultimately it is based on [Jekyll](https://jekyllrb.com).

### Why not Jekyll, or Hugo, or one of the other static site generators?

There are many, many static site generators available, and each has their pros
and cons. [Jekyll](https://jekyllrb.com), being written in Ruby, is not very
performant. [Hugo](https://gohugo.io) is written in Go, and is very fast, but it
is not the easiest to customise. [Gatsby](https://www.gatsbyjs.com) is written
in JavaScript, and is very customisable, but it is also very complex, heavily
dependent upon React, and requires a lot of dependencies. They are just some of
the most popular and widely-known systems. [mdBook](https://rust-lang.github.io/mdBook)
is perhaps a close contender, and Rust-based, but it still has critical
differences.

Each system has had key decisions made about it which differentiate it from the
others. One key decision that Rustmark makes is to use Markdown files as the
source of the content, but also to mantain compatibility with general Git server
previewing. Most regular static site generators use some flavour of templating
language, and those that use Markdown do not provide quite the same focus or
features as Rustmark.

With Rustmark, it's possible to easily share things like README files between
different repositories, without worrying about conversion or compatibility,
which is a benefit for dev teams. Rustmark also has no JavaScript dependencies,
and indeed hardly any JavaScript at all.

Additionally, there was a desire to write something in Rust!

### Why run it as a server? Why not just generate static content?

It is totally possible to generate static content from the Markdown files, and
then host that content on a static web server. If that is a requirement then the
build output can be used directly, and there is then technically no need to run
the server. Indeed, being able to do this in a more formal manner may end up
being a feature of Rustmark.

However, there are three compelling reasons for running it as a server:

  1. It allows self-managed authentication, which can be extended as required in
     a way that is not possible with a static site alone (and HTTP auth is not
     exactly ideal).
  2. Everything is packaged up as one single binary, which is easy to deploy and
     run.
  3. It allows for dynamic content and features, such as search (not currently
     implemented), which is not possible with a static site. Rustmark provides a
     decent springboard for building a more complex system, if required.

Additionally, because Rustmark has a web sever built in, there is zero secondary
setup required to get started. Just run the binary, and it works. Of course,
some people will want to run it behind a reverse proxy, and that is also
possible.

### With everything being in one binary, isn't that a limiting factor?

Yes. Although, it has been tested with a repository containing over 10,000
Markdown files, which is 550MB of Markdown, and it works just fine. It is
unlikely that many people will have a repository that large, and if they do,
they probably have bigger problems to worry about!

Still, it is a valid concern, and it is something that may be addressed in the
future, probably with a configuration setting to control whether everything is
built into the binary or left externally to be deployed alongside it.


## Setup

The steps to set up this project are simple and standard. You need a
reasonably-recent Rust environment, on a Linux machine. There are currently no
special requirements beyond what is needed to build a standard Rust project.

### Environment

There are some key points to note about the environment you choose:

  - Debian and Ubuntu are the Linux distros of choice, although other distros
    should also work just fine, as there are no special requirements.
  - Running natively on Windows is not targeted or tested, and there are no
    plans to support it, so although it may work, it also may not. Running on
    WSL does work fine, and is the recommended way to run on Windows.
  - Running natively on MacOS is untested, although there is no known technical
    reason why it would not work.

Typically, you will set up Rust using [`rustup`](https://rustup.rs/), which is
the recommended way to install Rust. The `stable` toolchain is targeted, as the
focus is on stability and correctness, rather than bleeding-edge features.

Once you have Rust installed, you can build the project using `cargo build`.
This will download and compile all dependencies, and build the project. You can
then run the project using `cargo run`.

### Configuration

Rustmark is configured using a TOML file. The default configuration file is
`Config.toml`, which should be placed in the same directory as the binary. The
configuration settings (and file) are optional, and if not provided, Rustmark
will use default values for all configuration options.

It is also possible to pass configuration parameters from the command line, as
environment variables. The environment variables take precedence over the
configuration file options.

### Running

Rustmark can be run using the `cargo run` command, or by running the compiled
binary directly. The server will listen on port 8000 by default, and will serve
content from the `markdown` and `static` directories. The `markdown` directory
contains the Markdown files to be rendered, and the `static` directory contains
the static files to be served.

### Testing

You can run the test suite using `cargo test`. This will run all unit and
integration tests.

**Note that, at present, there are no tests written specifically for this
project, as it is mostly a combination of other crates from the Rust ecosystem.
Tests might be added when the project is more mature and sensible things to test
have been clearly identified.**

### Documentation

You can build the developer documentation using `cargo doc`. This will generate
HTML files and place them into `target/doc`. You can then open the documentation
in your browser by opening `target/doc/rustmark/index.html`.

Building the documentation for local development use will also provide you with
links to the source code.


## Usage

The repository is designed so that it can be forked, and content added. As such,
it is best to keep in line with the existing structure and intended usage, to
make updates from the upstream repository easier to merge and apply.

### Structure

Markdown files should be placed in the `content` directory, along with any
images and other files that need to be protected by the same authentication as
the rendered Markdown pages. Public images should be placed in the `static/img`
path, and will be served from the `/img` URL path.

All of the content and static material is included in the compiled binary,
making it very straightforward to deploy.

### Customisation

If any customisations are required, they should be placed in the `js/custom.js`
and `css/custom.css` files. These files are included after the default CSS and
JavaScript, and so can be used to override the default behaviour. These files
will not be modified when updating from the upstream repository.

Note that the `custom.js` file is only included in the rendered Markdown pages,
once logged in, and not in the general system pages such as the login page. The
`custom.css` file is included in all pages.


## Deployment

You can build the project in release mode by using `cargo build --release`.
Everything required for deployment will be contained in the single binary file
produced. It is recommended to run [`upx`](https://upx.github.io/) on the
executable before deployment, to reduce the file size.

The resulting binary file can then be copied to the deployment environment, and
run directly. This will often be in a Docker or Kubernetes container, but that
is outside the scope of this document.

A typical build script might look like this:

```sh
cargo build --release
upx --best target/release/rustmark
scp target/release/rustmark you@yourserver:/path/to/deployment/directory
```


## Legal

### Disclaimer

The name "Rustmark" is a combination of the words "Rust" and "Markdown". There
is no affiliation with the Rust project or the Rust Foundation, nor any intent
to imply any.


### Attributions

This project uses the [Rust logo](https://github.com/rust-lang/rust/issues/11562#issuecomment-32700278)
as a default, due to being written in Rust. The logo is
[freely usable](https://github.com/rust-lang/rust/issues/11562#issuecomment-50833809)
under the [CC-BY (Creative Commons Attribution) license](https://creativecommons.org/licenses/by/4.0/).

An image of Ferris the crab (the Rust mascot) is used to illustrate the Markdown
content examples. This image is sourced from [rustacean.net](https://rustacean.net/)
and is in the [Public Domain](https://creativecommons.org/publicdomain/zero/1.0/),
so can be freely used.

This project uses the [Bulma CSS framework](https://bulma.io/), which is
[published](https://github.com/jgthms/bulma/blob/master/LICENSE) under the
[MIT license](http://opensource.org/licenses/MIT) and free to use without
restriction.

The [Font Awesome](https://fontawesome.com/) icons are [published](https://fontawesome.com/license/free)
under the [CC-BY (Creative Commons Attribution) license](https://creativecommons.org/licenses/by/4.0/),
and the webfonts under the [SIL OFL (Open Font License)](https://scripts.sil.org/OFL).
They are freely usable, along with the CSS code used to display them, which is
released under the [MIT license](http://opensource.org/licenses/MIT).

The [Twemoji](https://twemoji.twitter.com/) graphics used to stylise Unicode
emojis are [published by Twitter](https://github.com/twitter/twemoji#attribution-requirements)
under the [CC-BY (Creative Commons Attribution) license](https://creativecommons.org/licenses/by/4.0/),
and are freely usable, along with the Twitter JavaScript code used to transform
them, which is released under the [MIT license](http://opensource.org/licenses/MIT).


