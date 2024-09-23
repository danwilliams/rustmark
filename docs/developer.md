# Developer documentation

This file contains information of use to developers wanting to extend Rustmark
for their own projects. The rules and information contained here should serve as
a good baseline for any project, but can be changed as required.

The main sections in this document are:

  - [Getting started](#getting-started)
  - [Codebase structure](#codebase-structure)
  - [API endpoint structure](#api-endpoint-structure)
  - [API endpoint commit checklist](#api-endpoint-commit-checklist)
  - [Coding standards](#coding-standards)
  - [Coverage](#coverage)


## Getting started

[Commits to forks]:          https://github.com/orgs/community/discussions/45474
[Create repo from template]: https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template

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


## Codebase structure

[Terracotta]:           https://crates.io/crates/terracotta
[Terracotta structure]: https://github.com/danwilliams/terracotta/docs/developer.md#codebase-structure

Rustmark is based on [Terracotta][], which is a web application framework. This
document focuses on Rustmark, but if you want to know more about the underlying
application structure, you should refer to the [Terracotta structure
documentation][Terracotta structure].


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


## Coding standards

[Terracotta coding standards]: https://github.com/danwilliams/terracotta/docs/developer.md#coding-standards

Rustmark inherits the [Terracotta coding standards][].


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


