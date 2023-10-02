# Developer documentation

This file contains information of use to developers wanting to extend Rustmark
for their own projects. The rules and information contained here should serve as
a good baseline for any project, but can be changed as required.

The main sections in this document are:

  - [API endpoint structure](#api-endpoint-structure)
  - [API endpoint commit checklist](#api-endpoint-commit-checklist)
  - [Coverage](#coverage)


## API endpoint structure

Machine-consumable endpoints should be placed under a path of `/api`. Those that
have application functionality should be versioned, and placed under a path of
`/api/vX`, where `X` is the version number. For example, the first version of
the API is placed under `/api/v1`.


## API endpoint commit checklist

This section contains a checklist of things that are mandatory to do when adding
a new API endpoint.

**For each endpoint added**, the following need to also be added:

  - Rustdocs
  - Unit tests
  - OpenAPI documentation
  - Written documentation

**For each commit made**, the following need to pass without errors or warnings:

  - `cargo build`
  - `cargo clippy`
  - `cargo doc`
  - `cargo test`

**Before a new endpoint can be declared complete**:

  - A coverage report needs to be run and checked. See the [Coverage](#coverage)
    section for more details.


## Coverage

[kcov]:          https://github.com/SimonKagstrom/kcov
[rust-coverage]: https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html#source-based-code-coverage
[Tarpaulin]:     https://crates.io/crates/cargo-tarpaulin

[Since Rust 1.60.0, coverage is supported natively][rust-coverage]. This means
that there is no need to use external tools such as [Tarpaulin][] or [kcov][] to
generate coverage reports, which is a huge improvement.

### Preparation

On a Debian or Ubuntu system, you will need to install the `grcov` package. You
will also need to install the `llvm-tools-preview` component for Rust, and
create a directory to store the coverage reports in.

```bash
sudo apt install grcov
rustup component add llvm-tools-preview
mkdir coverage
```

### Running

The following commands will run the tests and generate coverage reports. The
profile files are then deleted, as they are not needed. The commands will
generate reports in HTML and LCOV formats, the latter of which can be loaded
into various tools.

Note that the `--binary-path` is important, and needs to point to your build
directory. By default this will be under `./target`, but if you have changed
this, e.g. to store builds in a central location, then you will need to adjust
the path accordingly.

```bash
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/html
grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/tests.lcov
find . -name "*.profraw" -delete
```

### Viewing

The HTML report can be viewed by opening `coverage/html/index.html` in your
browser.


