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

### Health check

There is a simple availability check endpoint at `/api/ping`. This endpoint is
intended to be used by monitoring systems to check that the application is
available. Statistics are available at `/api/stats`.

  - **`GET /api/ping`**
    Returns a `200 OK` response with an empty body.

  - **`GET /api/stats`**
    Returns a `200 OK` response with a JSON body containing various statistics
    about the API service.

  - **`GET /api/stats/raw`**
    Returns a `200 OK` response with a JSON body containing raw interval
    statistics about the API service.

The health check endpoints are not authenticated, and not versioned.


## User-facing endpoints

[OpenAPI]: https://www.openapis.org/
[Swagger]: https://swagger.io/
[Redoc]:   https://redoc.ly/
[RapiDoc]: https://mrin9.github.io/RapiDoc/

### Web application

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

### Documentation

[OpenAPI][] documentation is available at the following endpoints:

  - `/api-docs/swagger` - [Swagger][] UI
  - `/api-docs/redoc`   - [Redoc][] UI
  - `/api-docs/rapidoc` - [RapiDoc][] UI

Although Swagger is the typical choice, all three of these UIs are made
available to allow for personal preference. They all allow browsing of the API
functionality according to the OpenAPI schema generated from the code.


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


