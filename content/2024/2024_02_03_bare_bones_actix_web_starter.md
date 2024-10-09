---
title: "Bare Bones Actix Web Starter"
date: "2024-02-03"
slug: "bare-bones-actix-web-starter"
category: "backend web frameworks"
tag: "actix web"
summary: "An article showing you how to get started with Actix Web"
draft: false
edited: false
---

## Introduction

I drift between what I consider the “big three” Rust backend web frameworks, Actix Web, Axum, and Rocket. Today I’d like to focus a little on Actix Web and take you through how to build a basic starter. This starter can serve as a template for any future project using the framework.

Actix Web has been around many years and the most mature of the “big three”. Many production services are built with it. It’s relatively easy to get off the ground, is good to work with, and has a great, vibrant ecosystem.

## What are we Building?

Before we go any further, what are we going to build?

The project will be an API which will listen at localhost:8000 for incoming requests. There will be a single `/health_check` endpoint, which will return a `200 OK` response and empty body. We’ll incorporate a fallback handler which will return `404 Not Found` for any endpoints that haven’t been defined yet.

Let’s get our starter spun up!

## Setup

First, ensure that you’ve installed Rust. I’m not going to go through it here. See my article [Let the Journey Begin](/article/2022-04-05/let-the-journey-begin) for a link that will help you install Rust if needed.

Open a command prompt in your favoured environment, go to wherever you save your code and type:

```bash
cargo new --bin --vcs none starter-actix-web
```

This will:

- create a new Rust binary project
- disable version control (for now, we’ll do it manually later)

Then, change into the fresh starter-actix-web directory and a barebones project should be ready to work on. The project directory should look like:

![project directory](/assets/images/articles/images/starter-actix-web-proj-dir.webp)

## Dependencies

We don’t need very much to get started. In fact, for now, we’re not going to add anything other than the actix-web crate itself. From your command prompt, type:

```bash
cargo add actix-web
```

This will download and add the actix-web crate to the dependencies for the project, automatically editing the `Cargo.toml` file as needed. `Cargo.toml` is the overall configuration file for the project. It contains essential metadata as well as any runtime and development dependencies. At this point, yours should look like:

![initial Cargo.toml](/assets/images/articles/images/initial-cargo-toml.webp)

## Structure

Alright, we’re rolling. Before we go any further, let’s organize our project to make it easier to work with and more straightforward to add things in the future. Why you ask? Well, yes, you could do everything in your `main.rs` file and call it a day, BUT, that will quickly get messy. I like to leverage Rust’s module system to split my projects into separate files and folders, keeping everything logical and tidy. For background, head over to this article which provides background on the high level organizational concepts.

We’re going to broadly divide the project into a binary crate and a library crate. Let’s make the following changes to our Cargo.toml:

```toml
[[bin]]
name = "actix-web-template"
path = "src/bin/main.rs

[lib]
name = "actix_web_template_lib"
path = "src/lib/lib.rs"
```

This will help the compiler understand that our project consists of a binary and a library, with path information clearly defined. In addition to the changes to `Cargo.toml`, make two new directories, `bin` and `lib`, under the `src` directory. Move `main.rs` into `bin` and create a new file called `lib.rs` under the lib directory. Once you’re done that, the overall structure is complete.

## Binary Crate

There will be some pre-made content within `main.rs`. Delete that, and replace it with:

```rust
// src/bin/main.rs

// dependencies
use actix_web_template_libe::startup;

// main function
#[actix_web::main]
async fn main() {
  startup()
    .await
    .expect("Unable to start the server on port 8000")
}
```

When you’re done, you’ll get a warning related to the fact we haven’t written `startup.rs` yet. We pull it into scope, with a use statement, to get it done. Just leave the warning for now, we’ll get there in the next section. Keen eyes will notice that the name of the library crate, as defined in Cargo.toml is used to refer to what we need and bring it in.

What we’ve done is create a simple, thin main function which does nothing except call startup.rs, which lives in our library crate. It does the heavy lifting of configuring our API, including registering routes and whatever ancillary functionality we include. The function startup() returns a future, so we need to await on it. If the future resolves without errors, the API will start and begin listening for requests. If something goes wrong, our application will panic and crash, with the error message that it was unable to start the server on the desired port. Error handling is not sophisticated here, it’s ok to just panic and crash, as there’s literally no point in continuing if we can listen for requests.

Let’s move on to the library crate.

## Library

The bulk of the action is in the library crate. Now that we’ve defined where things are located we can fill in the details.

Let’s return back to lib.rs first. We need to put a few things in it.

```rust
// src/lib/lib.rs

pub mod routes;

pub mode startup;

pub use startup::*;
```

This short amount of code declares to the Rust compiler that we have a `routes` module and a `startup` module. The `routes` module will contain the code for our endpoint handlers. The `startup` module contains all the code needed to configure and spin up the server. We put a `use startup::*;` here as well, which enables us to easily refer to the inside components of the module from anywhere. Also, note that everything here has the pub keyword, enabling visibility from other parts of the code base.

Here’s what you’re going to put into startup.rs:

```rust
// src/lib/startup.rs

// dependencies

use crate::routes::health_check;

use actix_web::{web, App, HttpResponse, HttpServer};

// startup function, configures the app, returns a server with all the configured routes

pub async fn startup() -> std::io::Result<()> {

    HttpServer::new(|| App::new().service(health_check).default_service(web::to(HttpResponse::NotFound)
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
```

First, we pull in the first of our routes. You’ll draw ire from the compiler, because they’re not written yet, so ignore that for now. We need a couple of things from actix-web itself, so we bring them into scope. The meat of the matter is the `startup()` function, which doesn’t need an parameters, and returns a `std::io::Result<()>` as available in the Rust standard library. The reason we need that is that the `.bind action`, where we grab an address and bind a port to it, is fallible, so we need to handle the error. We use the Rust `?` question mark operator to “bubble up” any error to the function caller, which in this case is main() from our binary. Any error will get converted into the message we defined back in the binary, using `.expect()`.

Back to the rest of what’s happening here. We define the details of the app using `App::new()` and attach our `health_check` route as well as an in-built default which will return a `NOT FOUND` for any non-existent routes. Then `HttpServer::new()` is used to spin up the server. I recommend reading the [“Getting Started”](https://actix.rs/docs/) section in the Actix Web docs for a bit more in depth of what’s happening.

Inside the `routes` folder, we need to define another `mod.rs`, it will look like this:

```rust
// src/lib/routes/mod.rs

pub mod health_check;

pub use health_check::*;
```

It’s pretty simple, but as more routes are added, the list of modules declared here will grow. We define a module for our `health_check` endpoint, and give it a use statement so that it’s available anywhere. The second last puzzle piece is our actual `health_check` endpoint:

```rust
// src/lib/routes/health_check.rs

// dependencies
use actix_web::{get, HttpResponse, Responder};

// health_check endpoint
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
```

This endpoint will accept requests made to the `/health_check` route and will respond with a `200 OK` response and an empty body. This endpoint is useful as a way of checking that our API is alive and ready to accept requests. It can be used in combination with a logging or monitoring service to keep tabs on the availability of the API.

That’s it!

Let’s move on to add the skeleton of some integration tests.

## Testing

Testing is important. I’m still learning to do it properly. The thing I love about Rust is there’s really no excuse to not do it, as all the tools are readily available.

Let’s set up a directory outside the `src` folder and call it `tests`. Within, we’ll create a sub-folder called `api`. Lastly, we’ll create a `main.rs` and `health_check.rs` files within the `tests/api` folder. The Rust compiler knows to look for integration tests within this special `test` folder we’ve created. Each test contained within will be compiled into it’s own crate. This allows us to keep things tidy and organized, just like the split into `bin` and `lib` crates in our application.`

You should have something like this after the above folders are created:

![tests folder](/assets/images/articles/images/starter-actix-web-tests.webp)

Now, within `main.rs`, add the following:

```rust
// tests/api/main.rs

mod health_check;
```

Last thing we need is to create the actual test within `health_check.rs`:

```rust
// tests/api/health_check.rs

// dependencies
use actix_web::{test, App};
use actix_web_template_lib::routes::health_check;

#[actix_web::test]
async fn test_health_check() {
  let app = test::init_service(App::new().service(health_check)).await;
  let req = test::TestRequest::get().uri("/health_check").to_request();
  let resp = test::call_service(&app, req).await;
  assert!(resp.status().is_success());
}
```

Actix Web has built in machinery for testing, you can read more about it [here in their docs](https://actix.rs/docs/testing). To run the test, type `cargo test` at the command prompt. If all goes well, you should see that our `test_health_check()` function is a success.

![successful test](/assets/images/articles/images/starter-actix-web-successful%20test.webp)

## Moment of Truth

Now that we’ve completed our starter, and verified that the `/health_check` endpoint is available thanks to our integration test, there’s only one thing left to do. Run it!

```bash
cargo run
```

After the app compiles, you should see:

![first run](/assets/images/articles/images/starter-actix-web-first-run.webp)

In a separate command prompt window, we can check for life using cURL:

```bash
curl -v http://localhost:8000/health_check
```

![health check workds](/assets/images/articles/images/starter-actix-web-health-check-works.webp)

```bash
curl -v http://localhost:8000/does_not_exist
```

![does not exist works](/assets/images/articles/images/starter-actix-web-does-not-exist-works.webp)

Congratulations! You have yourself a jumping off point for adventures in Actix Web. From here, you can add further routes, application logic, integrate a database…whatever.

## Version Control

I tend to leave version control setup until I’m nearly done with all the initial setup, folder creation, etc. Let’s get this starter of ours covered by git. First, make a `.gitignore` with the following content:

```bash
# .gitignore for the starter-actix-web repo

# exclude build artifacts
target/
```

```bash

git init

git add .

git commit -m "Initial commit"
```

If you wish, you can push the whole thing up to a GitHub repo for later use. I’ve pushed mine up here: [Actix Web Starter](https://github.com/sentinel1909/starter-actix-web). It’s extremely wise to use a version control system when building any piece of software, no matter how small. Git can save you from self-inflicted wounds.

## Conclusion

I hope you’ve enjoyed this brief overview of how to get started with Actix Web. Best of luck in your further adventures! In the future, I’ll expand on this starter and show you how to embellish it and build on it.

## Resources

[Actix Web Documentation](https://actix.rs/)
