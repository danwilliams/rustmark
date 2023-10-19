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
available.

  - **`GET /api/ping`**
    Returns a `200 OK` response with an empty body.

The health check endpoints are not authenticated, and not versioned.

### Statistics

Statistics are available at `/api/stats`.

  - **`GET /api/stats`**
    Returns a `200 OK` response with a JSON body containing various statistics
    about the API service.

  - **`GET /api/stats/history`**
    Returns a `200 OK` response with a JSON body containing historical interval
    statistics about the API service.

  - **`GET /api/stats/feed`**
    Returns a `200 OK` response with a status code of `101 Switching Protocols`
    and the `Connection` header set to `Upgrade`. This will upgrade the HTTP
    connection to a WebSocket connection, It will then stream statistics every
    second in JSON format.

The statistics endpoints are not authenticated, and not versioned.

#### Types of measurements

There are three main areas for which measurements are tracked and interval-based
statistics are calculated. For each of the areas below, cumulative statistics
are calculated for each endpoint, and for each HTTP status code returned.
Average, maximum, and minimum response times plus sample count are tracked for
each interval, and summarised for each period of time configured.

  - **Response times**
    These are straightforward, being the amount of time taken to respond to a
    request.

  - **Active connections**
    The connection average is not a measure of connections over time. For that,
    the number of requests per period can be used, to work out, for instance,
    the number of requests per second. That's a trivial calculation. Rather, the
    connection average is the average number of connections in existence when
    the system is being used. Periods of zero activity will not affect this
    average - although obviously they will cause the requests per time period to
    fall. When each response is served, the number of active connections is
    sampled, and this is used for the average. This is more important than
    requests per second for connection monitoring, as it shows the average load
    profile of the system: in other words, "when the system is actively used,
    what is the typical number of connections". This is what the average, max,
    and min are measuring - and so the min will never fall below 1. The
    importance of these statistics is that they help ensure the system is
    providing what is needed when it is asked.

  - **Memory usage**
    The memory usage is the amount of memory used by the application at the end
    of processing each request and preparing its response. This is not a measure
    of the peak memory usage whilst processing the request, although if there
    are multiple simultaneous requests then the point of sampling will naturally
    coincide with some of the other requests still being processed. Rather, this
    is a measure at a consistent point in time, so that memory leaks can be
    detected.


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


