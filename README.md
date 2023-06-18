# Rustmark

[Rust]:     https://www.rust-lang.org/
[Rustmark]: https://crates.io/crates/rustmark

Rustmark is a simple Markdown server written in [Rust][]. It is intended to be
easy to use, easy to fork and customise, and easy to deploy.

Rustmark currently exists as a crate on [crates.io][Rustmark] to establish
presence and gain visibility and awareness while its library and binary features
mature. (See the [Usage](#usage) section for more information.) It may also be
useful to able to run it and see it working before then using it as a foundation
for a new project.

The main sections in this README are:

  - [Content](#content)
  - [Features](#features)
  - [Why Rustmark?](#why-rustmark)
  - [Usage](#usage)
  - [Setup](#setup)
  - [Deployment](#deployment)
  - [Legal](#legal)


## Content

The entry point for reading the Markdown files is the [index page](content/index.md).
If you are here to read the content (for instance if you are using a Git system
to preview the Markdown, such as GitHub or Gitea), start there.

An example showing available [Markdown features](content/rustmark/features.md)
is also provided, along with some [Guidelines](content/rustmark/guidelines.md)
for use.


## Features

[Axum]:            https://crates.io/crates/axum
[Bulma]:           https://bulma.io/
[CommonMark]:      https://commonmark.org/
[CommonMark 0.30]: https://spec.commonmark.org/0.30/
[Comrak]:          https://crates.io/crates/comrak
[Figment]:         https://crates.io/crates/figment
[Font Awesome]:    https://fontawesome.com/
[GFM]:             https://github.github.com/gfm/
[Hyper]:           https://crates.io/crates/hyper
[Nerd Fonts]:      https://www.nerdfonts.com/
[Syntect]:         https://crates.io/crates/syntect
[Tera]:            https://crates.io/crates/tera
[Terracotta]:      https://crates.io/crates/terracotta
[Tracing]:         https://crates.io/crates/tracing
[Twemoji]:         https://twemoji.twitter.com/

The main high-level points of note are:

  - **Markdown**
      - Rendering of Markdown files using [Comrak][]
      - Syntax highlighting using [Syntect][]
      - Full compliance with [CommonMark][] [0.30][CommonMark 0.30]
      - Support for [GitHub-flavoured Markdown (GFM)][GFM]
      - Tables, task lists, strikethrough, and autolinks (from [GFM][])
      - Superscript, footnotes, description lists, and emoji shortcodes
      - Callouts and details blocks, based on extended blockquotes
  - **Display**
      - CSS foundation using the [Bulma][] CSS framework
      - Icons using [Font Awesome][]
      - Twitter emojis using [Twemoji][]
      - Code displayed using [Nerd Fonts][]
      - Collapsible callouts and details blocks
      - Collapsible document sections based on headings (automatic)
      - Linkable headings
      - Automatic per-page table of contents in navigation menu
  - **Build**
      - HTML generated from Markdown asynchronously at build time and included
        in binary
      - Efficient rebuild process regenerating only changed files
  - **Customisation**
      - Custom JS and CSS files for customisation overrides
      - Templates implemented using the [Tera][]
        template engine
      - Configuration from config file and env vars using [Figment][]
      - Simple codebase layout
      - Easy to extend and build upon
  - **Security**
      - Simple authentication using sessions and config-based user list
      - Login page, public and protected routes, logout ability
      - Protected static content files for use alongside Markdown content
  - **Performance**
      - High-performance asynchronous HTTP server using [Tokio Hyper][Hyper]
      - Based on the robust and ergonomic web framework [Axum][]
  - **Other**
      - Compatibility with browsing the content on a Git server such as GitHub
        or Gitea
      - Static file handling
      - Single-file deployment — all assets baked in
      - Logging of HTTP requests and events using [Tokio Tracing][Tracing]
      - Graceful handling of 404 and 500 HTTP errors
      - Graceful handling of runtime application errors

More details are available about the [supported Markdown features](content/rustmark/features.md),
along with examples.

### Authentication

Rustmark features [Terracotta][]'s custom-rolled authentication system,
providing a basic session-based setup. However, it is highly recommended to
store the user credentials securely in a database. That is currently outside the
scope of this project, for a number of reasons, primarily the ambition to
provide a simple system that can be extended to use any database required. You
will probably also want to store the sessions in a database instead of in
memory.

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

[Coda]:         https://coda.io/
[Confluence]:   https://www.atlassian.com/software/confluence
[Gatsby]:       https://www.gatsbyjs.com/
[GitHub Pages]: https://pages.github.com/
[Hugo]:         https://gohugo.io/
[Jekyll]:       https://jekyllrb.com/
[Jira]:         https://www.atlassian.com/software/jira
[Notion]:       https://notion.so/
[mdBook]:       https://rust-lang.github.io/mdBook

The intention is to provide a simple system that is easy to maintain, and
focused on developer documentation — or at least, documentation written by
people who are comfortable working in Markdown with text editors, and committing
their changes to a Git repository.

There are many tools available to provide wiki-style functionality in very
friendly ways, the leading product (arguably) being [Notion][]. [Coda][] also
deserves an honourable mention, but [Confluence][], although widely used due to
the popularity of [Jira][], lags a long way behind.

### So why not just use Notion?

Notion is a great product, with some excellent and powerful features, and it is
also very easy to use. However, it does not feature any kind of approval
process, meaning that it is all too easy for incorrect information to creep in —
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

Additionally, some people/companies do not like the idea of their information
being stored in a third-party system, and would prefer to keep it in-house. This
is another reason why Rustmark is a good choice, as it allows full control over
the content, distribution, and access.

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

[GitHub Pages][] is a great way to host static content,
and it is very easy to use. However, not everyone uses or wants to use GitHub,
and there are constraints on the free accounts that may not make it the ideal
choice for some people. There are also limitations on the amount of
customisation that can be performed, and it is not possible to do anything
dynamic, as ultimately it is based on [Jekyll][].

### Why not Jekyll, or Hugo, or one of the other static site generators?

There are many, many static site generators available, and each has their pros
and cons. [Jekyll][], being written in Ruby, is not very performant. [Hugo][] is
written in Go, and is very fast, but it is not the easiest to customise.
[Gatsby][] is written in JavaScript, and is very customisable, but it is also
very complex, heavily dependent upon React, and requires a lot of dependencies.
They are just some of the most popular and widely-known systems. [mdBook][] is
perhaps a close contender, and Rust-based, but it still has critical
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


## Usage

[Coding standards]:          https://github.com/danwilliams/terracotta#coding-standards
[Commits to forks]:          https://github.com/orgs/community/discussions/45474
[Create repo from template]: https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template
[Terracotta structure]:      https://github.com/danwilliams/terracotta#structure

The Rustmark repository is designed so that it can be forked, and content added.
As such, it is best to keep in line with the existing structure and intended
usage, to make updates from the upstream repository easier to merge and apply.

Rustmark presents its core Markdown features as a library, for use in other
projects without using the whole application, and there are plans for it to also
become useful in a standalone capacity as a binary without having to clone the
full repository.

### Getting started

The Rustmark repository is intended to be forked, although you may not want to
do so in an explicit manner (i.e. by clicking the "Fork" button on GitHub).
Instead, the recommended approach is to clone the repository, and then push it
to a new location. This will give you a clone with all the commit history, but
without the link to the upstream repository, so it will not be counted as a fork
by GitHub. This is ideal if you want to add content and customise the
application for your own use, and also want to be able to merge in Rustmark
updates, but do not want to contribute back to the upstream repository.

Alternatively, you can use the repository as a template, and create a new
repository based on it. The Rustmark repository is set up as a template
repository on GitHub, so that you can easily [click the "Use this template"
button to create a new repository based on it][Create repo from template]. You
can then clone your new repository and start working on it. This will give you a
starting point where you have all the project files, but none of the commit
history. This is beneficial if you want to make extensive changes to the
project, and are not bothered about being able to merge in Rustmark updates.

Regarding forking and cloning, you should be aware of the following points:

  - You will have the full commit history when you fork or clone, which will
    likely be useful, but it is specifically relevant to Rustmark, and so
    mention of e.g. release versions will be in that context. This is fine if
    your project will just add content and customisations, and keep up with
    Rustmark versions, but for more extensive changes you should rename your
    clone and implement your own, independent versioning scheme, in which case
    neither forking nor cloning is recommended, and the repository template
    route will be best.
  - You will also have the various release version tags created on the Rustmark
    repository, which will be fine if you are following the Rustmark release
    cycle, but will otherwise likely conflict with your own tags if you are not.
  - There is a significant advantage in maintaining a Git tree association with
    Rustmark as an upstream repository, and as adding content and performing
    typical customisation will not lead to conflicts, you are best off being
    able to pull in updates for it.
  - Forks on GitHub are treated as subsidiaries of the original repository, and
    not first-class repositories in their own right. For this reason, [commits
    made to forks don't count as contributions in user profiles][Commits to forks],
    which is not a desirable situation if you are starting a new project and not
    intending to contribute changes back to the upstream repository. This is the
    main reason why cloning is recommended over forking.

For these reasons, forking in the GitHub-recognised sense is not recommended,
and cloning and pushing to a new repository is the preferred route for standard
use cases.

### Structure

Rustmark is based on [Terracotta][], which is a web application template. This
document focuses on Rustmark, but if you want to know more about the underlying
application structure, you should refer to the [Terracotta structure
documentation][Terracotta structure].

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

This document focuses on how to use and customise Rustmark, but if you are
wanting to make more extensive changes, you should refer to the [Terracotta
documentation][Terracotta structure]. You can also find information there about
the [coding standards][] used.


## Setup

The steps to set up this project are simple and standard. You need a
reasonably-recent Rust environment, on a Linux machine. There are currently no
special requirements beyond what is needed to build a standard Rust project.

Note that these instructions are for building the application yourself, which
will usually be in context of having created a [new Rustmark project](#getting-started)
by cloning, forking, or possibly using the Rustmark repository as a template. In
this case these steps will apply for your new project. You can also download the
crate using `cargo install rustmark`, which will install the latest version of
Rustmark from crates.io, but this currently is not particularly useful beyond
letting you poke at the default, running application without having to clone the
repository and build it yourself, to see if you like it. See the [Getting
started](#getting-started) section for more information on creating a project
using based on Rustmark.

### Environment

[Rustup]: https://rustup.rs/

There are some key points to note about the environment you choose:

  - Debian and Ubuntu are the Linux distros of choice, although other distros
    should also work just fine, as there are no special requirements.
  - Running natively on Windows is not targeted or tested, and there are no
    plans to support it, so although it may work, it also may not. Running on
    WSL does work fine, and is the recommended way to run on Windows.
  - Running natively on MacOS is untested, although there is no known technical
    reason why it would not work.

Typically, you will set up Rust using [`rustup`][Rustup], which is the
recommended way to install Rust. The `stable` toolchain is targeted, as the
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

#### General options

The following options should be specified without any heading:

  - `host`   - The host to listen on. Defaults to `127.0.0.1`.
  - `port`   - The port to listen on. Defaults to `8000`.
  - `logdir` - The directory to store log files in. Defaults to `log`.
  - `title`  - The title of the application. Defaults to `Rustmark`.

As shown here:

```toml
host   = "127.0.0.1"
port   = 8000
logdir = "log"
title  = "Rustmark"
```

#### Local loading options

By default, all resources are baked into the binary, and served from there. This
is the most efficient way to run the application, but it is also possible to
load resources from the local filesystem, which can be useful for development
and testing, and when there are large content files.

It is possible to supplement or override Markdown content files and static
assets. Static assets are subdivided into protected and public.

It is advisable to bake Markdown files into the binary for performance reasons,
as they will not be cached if loaded locally, so will be parsed on every
request unless baked in. This is more important for production environments than
development ones, where it might be desirable to re-parse each time.

The following options should be specified under a `[local_loading]` heading:

  - `markdown`         - The loading behaviour for Markdown content.
  - `protected_assets` - The loading behaviour for protected static assets.
  - `public_assets`    - The loading behaviour for public static assets.

Each of these options can be one of the following values:

  - `Deny`       - Deny loading from the local filesystem. This is the default
                   for all the options.
  - `Supplement` - Load from the local filesystem if the baked-in resources are
                   not present.
  - `Override`   - Load from the local filesystem if present, and otherwise load
                   from the baked-in resources.

As shown here:

```toml
[local_loading]
markdown         = "Supplement" # default is "Deny"
protected_assets = "Override"   # default is "Deny"
public_assets    = "Override"   # default is "Deny"
```

For those options that allow loading from the local filesystem, the following
options can be specified under a `[local_paths]` heading:

  - `markdown`         - The path to the Markdown content. Defaults to
                         `content`.
  - `protected_assets` - The path to the protected static assets. Defaults to
                         `content`.
  - `public_assets`    - The path to the public static assets. Defaults to
                         `static`.

As shown here:

```toml
[local_paths]
markdown         = "content"
protected_assets = "content"
public_assets    = "static"
```

#### User list

A list of user credentials can be specified under a `[users]` heading:

  - `username: password` - The username as the key, and the password as the
                           value.

As shown here:

```toml
[users]
joe = "1a2b3c"
```

This is a simple list of username/password pairs, where the username is the key
and the password is the value. The password is stored in plain text, so be aware
of the security implications of this (ideally you would implement an integration
with your preferred database instead). The username and password are both
case-sensitive.

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


## Deployment

[UPX]: https://upx.github.io/

You can build the project in release mode by using `cargo build --release`.
Everything required for deployment will be contained in the single binary file
produced. It is recommended to run [`upx`][UPX] on the executable before
deployment, to reduce the file size.

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

[Bulma license]:        https://github.com/jgthms/bulma/blob/master/LICENSE
[CC-BY license]:        https://creativecommons.org/licenses/by/4.0/
[Font Awesome license]: https://fontawesome.com/license/free
[MIT license]:          http://opensource.org/licenses/MIT
[Public Domain]:        https://creativecommons.org/publicdomain/zero/1.0/
[Rust logo use]:        https://github.com/rust-lang/rust/issues/11562#issuecomment-50833809
[Rust logo]:            https://github.com/rust-lang/rust/issues/11562#issuecomment-32700278
[Rustacean]:            https://rustacean.net/
[SIL OFL license]:      https://scripts.sil.org/OFL
[Twemoji license]:      https://github.com/twitter/twemoji#attribution-requirements

### Disclaimer

The name "Rustmark" is a combination of the words "Rust" and "Markdown". There
is no affiliation with the Rust project or the Rust Foundation, nor any intent
to imply any.

### Attributions

This project uses the [Rust logo][] as a default, due to being written in Rust.
The logo is [freely usable][Rust logo use] under the [CC-BY (Creative Commons
Attribution) license][CC-BY license].

An image of Ferris the crab (the Rust mascot) is used to illustrate the Markdown
content examples. This image is sourced from [rustacean.net][Rustacean] and is
in the [Public Domain][], so can be freely used.

This project uses the [Bulma CSS framework][Bulma], which is [published][Bulma license]
under the [MIT license][] and free to use without restriction.

The [Font Awesome][] icons are [published][Font Awesome license] under the
[CC-BY (Creative Commons Attribution) license][CC-BY license], and the webfonts
under the [SIL OFL (Open Font License)][SIL OFL license]. They are freely
usable, along with the CSS code used to display them, which is released under
the [MIT license][].

The [Twemoji][] graphics used to stylise Unicode emojis are [published by
Twitter][Twemoji license] under the [CC-BY (Creative Commons Attribution)
license][CC-BY license], and are freely usable, along with the Twitter
JavaScript code used to transform them, which is released under the [MIT license][].


