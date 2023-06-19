# Changelog

[Font Awesome]:        https://fontawesome.com/
[Keep a Changelog]:    https://keepachangelog.com/en/1.0.0/
[Nerd Font]:           https://www.nerdfonts.com/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[Terracotta]:          https://crates.io/crates/terracotta
[Twemoji]:             https://twemoji.twitter.com/

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][], and this project adheres to
[Semantic Versioning][].


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


