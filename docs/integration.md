# Integration documentation

This file contains information of use to developers wanting to integrate
Rustmark's API into other applications.

The main sections in this document are:

  - [Machine-consumable endpoints](#machine-consumable-endpoints)
  - [User-facing endpoints](#user-facing-endpoints)
  - [Asset-serving endpoints](#asset-serving-endpoints)
  - [Authentication](#authentication)
  - [Authorisation](#authorisation)


## Machine-consumable endpoints

At present there are no machine-consumable endpoints.


## User-facing endpoints

As Rustmark's primary purpose is to serve Markdown files as HTML to a browser,
it comes with a number of pre-configured endpoints that are intended for
consumption by humans using a browser. These are:

  - **Protected**
      - `/`: Index page
      - `/*path`: Any Markdown files that exist in the `content` folder will be
        served as HTML, providing the path does not match any registered
        endpoint

  - **Public**
      - `/login`: Login page
      - `/logout`: Logout endpoint


## Asset-serving endpoints

There are a number of endpoints that serve static assets. These are:

  - **Protected**
      - `/*path`: Any non-Markdown files that exist in the `content` folder and
        where the path does not match any registered endpoint

  - **Public**
      - `/css/*path`: CSS files
      - `/img/*path`: Image files
      - `/js/*path`: JavaScript files
      - `/webfonts/*path`: Webfont files


## Authentication

At present there is no API-specific authentication mechanism, and the only
available authentication is for users logging in via a browser.


## Authorisation

Rustmark does not come with any authorisation functionality.


