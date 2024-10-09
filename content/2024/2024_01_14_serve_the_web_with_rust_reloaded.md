---
title: "Serve the Web with Rust...Reloaded"
date: "2024-01-14"
slug: "serve-the-web-with-rust-relaoded"
category: "backend web frameworks"
tag: "axum"
summary: "An article providing an overview of how to serve up web content with the Axum web framework."
draft: false
edited: false
---

In an earlier post, I showed how to serve web content with the Rocket web framework. Let’s do it again, but this time with Axum.

Axum is an unopinionated web application framework, written and maintained by the [Tokio](https://tokio.rs/) team. It’s very light and is built on top of the [hyper](https://hyper.rs/) HTTP library. Hyper reached a major milestone in November 2023, with it’s version 1.0 release. Axum was bumped to version 0.7 shortly after. The Tokio team wrote a [great blog post](https://tokio.rs/blog/2023-11-27-announcing-axum-0-7-0) explaining the major changes.

Alright, let’s get into some code…

```rust
// src/main.rs

// dependencies
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
>use tokio::net::TcpListener;

// root endpoint handler, sends back a "Hello from Axum!" message in HTML
async fn hello_from_axum() -&gt; impl IntoResponse {
    Html(include_str!("assets/index.html"))
}

// main function, creates a Router type, attaches our handler to the "/" route
#[tokio::main]
async fn main() {
    // create an app, has the Router type, has one index route
    let app = Router::new().route("/", get(hello_from_axum));

    // create an address to listen on
    let addr = SocketAddr::new([0, 0, 0, 0].into(), 3000);

    // get a listener, use the address we just created; the TcpListener::bind method could fail, so we use a match statement to handle that
   // we panic if we can't get a listener
    let listener = match TcpListener::bind(&amp;addr).await {
        Ok(tcp_listener) => tcp_listener,
        Err(err) => panic!("Could not get a TcpListener, {}", err),
    };

   // start up the server with our listener and the app; panics with an error message if the axum::serve function fails
    axum::serve(listener, app)
        .await
        .expect("Could not start up the axum server");
}

```

### Dependencies

First, we bring into scope several things from the axum crate itself:

- the `Html` type and `IntoResponse` trait, both from the response module
- the `get` method from the routing module, used to handle get requests
- the `Router` type, used to compose handlers and services
- the `SocketAddr` type from the standard library
- the `TcpListener` type from the Tokio crate

Axum relies on the Tokio crate for asyncronous operations, so be sure Tokio is added as a dependency in the Cargo.toml configuration file.

Let’s look at the main function first, then we’ll talk about the single handler function as part of that.

### Main Function

The general algorithm of our main function is:

- create an instance of our app; has one route at “/”, which accepts a get request and calls our `hello_from_axum` handler function
  - the `hello_from_axum` function takes no parameters, it simply returns some HTML created by reading in an `index.html` file using the `include_str!` macro
  - the `impl IntoResponse` return type indicates that we leverage the fact that Axum knows how to return an Html message without us writing any extra code, the IntoResponse trait is already implemented for the `Html` type
- create an address to listen on
- create a listener, using the address we just created, to listen for requests; there is some basic error handling here with a match statement, we panic if the attempt to get an address fails
- start up the server, using the `axum::serve` method, passing in the listener and app variables

Error handling is purposely left fairly basic. It’s generally not a good practice to panic and crash the program, but in this instance it makes sense. The application can’t run without an address to listen on or a listener to receive requests, so if these things fail there isn’t a lot of point in doing anything else.

That’s it!

If you typed everything in correctly, you can start up the basic server by typing:

```bash
cargo run
```

into your console. The server should start up and be listening on port 3000. If something happens error wise, you’ll see error messages returned at the console.

Head over to your web browser and type:

```bash
http://localhost:3000
```

You should be greeted with:

![](../../assets/images/articles/images/serve_the_web_with_rust_reloaded_output.webp)

Pretty cool hey?

I know, not really. It’s pretty basic and not very interesting. It’s yet another example of how uninteresting and boring Rust code is, which is great. Axum is terrific in its simplicity, but at the same time has much power and a great ecosystem. It really is garnering the mindshare of the Rust community and many projects appear to be standardizing around it.

There are a few warts on this simple server, chief of which is it doesn’t shut down very gracefully. I’ll show you how to do that in a future post.
