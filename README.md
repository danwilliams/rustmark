# Rustmark

Rustmark is a simple Markdown server written in [Rust](https://www.rust-lang.org/).
It is intended to be easy to use, and easy to deploy.


## Setup

The steps to set up this project are simple and standard. You need a
reasonably-recent Rust environment, on a Linux machine. There are currently no
special requirements beyond what is needed to build a standard Rust project.

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


## Configuration

Rustmark is configured using a TOML file. The default configuration file is
`Config.toml`, which should be placed in the same directory as the binary. The
configuration settings (and file) are optional, and if not provided, Rustmark
will use default values for all configuration options.

It is also possible to pass configuration parameters from the command line, as
environment variables. The environment variables take precedence over the
configuration file options.


## Running

Rustmark can be run using the `cargo run` command, or by running the compiled
binary directly. The server will listen on port 8000 by default, and will serve
content from the `markdown` and `static` directories. The `markdown` directory
contains the Markdown files to be rendered, and the `static` directory contains
the static files to be served.


## Testing

You can run the test suite using `cargo test`. This will run all unit and
integration tests.

**Note that, at present, there are no tests written specifically for this
project, as it is mostly a combination of other crates from the Rust ecosystem.
Tests might be added when the project is more mature and sensible things to test
have been clearly identified.**


## Documentation

You can build the developer documentation using `cargo doc`. This will generate
HTML files and place them into `target/doc`. You can then open the documentation
in your browser by opening `target/doc/rustmark/index.html`.

Building the documentation for local development use will also provide you with
links to the source code.


## Usage

The repository is designed so that it can be forked, and content added. As such,
it is best to keep in line with the existing structure and intended usage, to
make updates from the upstream repository easier to merge and apply.

Markdown files should be placed in the `content` directory, along with any
images and other files that need to be protected by the same authentication as
the rendered Markdown pages. Public images should be placed in the `static/img`
path, and will be served from the `/img` URL path.

All of the content and static material is included in the compiled binary,
making it very straightforward to deploy.


## Customisation

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


## Disclaimer

The name "Rustmark" is a combination of the words "Rust" and "Markdown". There
is no affiliation with the Rust project or the Rust Foundation, nor any intent
to imply any.


## Attributions

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


