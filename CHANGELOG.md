# Changelog

[Docker]:              https://www.docker.com/
[Font Awesome]:        https://fontawesome.com/
[Keep a Changelog]:    https://keepachangelog.com/en/1.0.0/
[Nerd Font]:           https://www.nerdfonts.com/
[OpenAPI]:             https://www.openapis.org/
[RapiDoc]:             https://mrin9.github.io/RapiDoc/
[Redoc]:               https://redoc.ly/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[Swagger]:             https://swagger.io/
[Terracotta]:          https://crates.io/crates/terracotta
[Twemoji]:             https://twemoji.twitter.com/

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][], and this project adheres to
[Semantic Versioning][].


## 0.6.0 (23 September 2024)

### Added

  - Added `/api/version` endpoint to `health` module
  - Added MSRV (Minimum Supported Rust Version) in `Cargo.toml`, set to 1.81.0
  - Added standard linting configuration

### Changed

  - Upgraded to [Terracotta][] 0.4.0, now a library
  - Updated [Font Awesome][] from 6.4.0 -> 6.6.0
  - Updated [Bulma][] from 0.9.4 -> 1.0.2
  - Adjusted logo/nav CSS for dark mode
  - Improved error-handling
  - Updated crate dependencies
  - Updated documentation

### Removed

  - Removed non-custom [Terracotta][] functionality which is now in the library


## 0.5.0 (28 October 2023)

### Added

  - Added `health` module
      - Added `/api/ping` endpoint
  - Added `stats` module
      - Added `/api/stats` endpoint with request count, response count, response
        times, open connections, memory usage, summary data per period, and
        breakdown per endpoint
      - Added `/api/stats/history` endpoint with type selector and from/limit
        constraints
      - Added `/api/stats/feed` websocket endpoint with type selector
      - Implemented using a central statistics queue and circular buffers for
        historical data
      - Per-second tick clock to keep statistics up-to-date
      - Configurable buffer sizes and summary periods
  - Added [OpenAPI][] functionality, including UIs for [Swagger][], [Rapidoc][],
    and [Redoc][]
  - Added developer documentation
  - Added API integration documentation

### Changed

  - Changed memory allocator to `jemalloc`
  - Improved error logging


## 0.4.2 (24 September 2023)

### Added

  - Added `Dockerfile` for building and running the application in a [Docker][]
    container

### Changed

  - Updated crate dependencies


## 0.4.1 (19 June 2023)

### Fixed

  - Corrected logic so that baked-in static files are always served whole, and
    not streamed
  - Fixed issue where some baked-in static files were not being served


## 0.4.0 (19 June 2023)

### Added

  - Added loading of Markdown files, HTML templates, and static assets from the
    local filesystem at runtime (configurable)
  - Implemented streaming of large static assets (configurable)
  - Added host option to config
  - Added Rustdoc source code documentation

### Changed

  - Improved README documentation


## 0.3.0 (18 June 2023)

### Added

  - Extended Markdown with details blocks
  - Extracted extended Markdown functionality into library
  - Added micro-animations to collapsible toggle indicators
  - Made all callout types collapsible
  - Added page icons to Rustmark content

### Fixed

  - Collapsible elements now work in Firefox
  - Nested callouts now render correctly

### Changed

  - Changed collapsible elements to use details blocks approach
  - Various styling tweaks and improvements
  - Improved features page documentation and examples
  - Improved README documentation
  - Improved CSS structure


## 0.2.1 (15 June 2023)

### Changed

  - Updated [Font Awesome][]
  - Updated crate dependencies
  - Improved README documentation


## 0.2.0 (15 June 2023)

### Added

  - Implemented Twitter emojis ([Twemoji][])
  - Added collapsible callouts for screenshots and images
  - Made headings collapsible
  - Added heading link anchors on hover
  - Added automatic table of contents menu
  - Added titles for image and screenshot callouts

### Changed

  - Moved extra Markdown processing from client-side JavaScript to Rust build
    script
  - Various styling tweaks

### Removed

  - Removed use of hardlinks from build script


## 0.1.0 (12 June 2023)

### Added

  - Forked [Terracotta][] repository
  - Added rendering of Markdown files
  - Added [Nerd Font][] for displaying code
  - Extended Markdown with callouts
  - Added custom JS and CSS files for customisation overrides
  - Amended routing to allow protected static content files
  - Added a build script to render Markdown pre-build
  - Added example Markdown to illustrate features
  - Added guidelines for adding content


