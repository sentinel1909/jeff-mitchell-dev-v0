---
title: "Serving the Web in a Rocket"
date: "2023-12-26"
slug: "serving-the-web-with-rust"
category: "web frameworks"
tag: "rocket"
summary: "An article providing an overview of how to server web content with the Rocket web framework."
draft: false
edited: false
---

The utility of the Rust language goes far beyond "systems" programming. I find web development with Rust a very clean, hassle-free experience...granted, you have to learn it, and the curve is steep. However, it doesn't take as long as you might think to be productive.

What if I told you that, in a few lines of Rust code, you could serve up some web content with the Rocket web framework? I originally titled this article, "Serving the Web...with 6 Lines of Rust", but the internet yelled at me for lying. I was using a framework they said, just be honest about that, they said...

Alright, I caved and changed the title.

Anyhow...

Show, don't tell...here we go...

## The Rocket Web Framework

The Rocket web framework enables simple Rust web development. Rocket is production-ready with its November 0.5 release. Get started by building a new Rust binary and changing into the freshly created folder:

```bash
cargo new --bin rocket-web && cd rocket-web
```

Next, add Rocket:

```bash
cargo add rocket
```

This will add the Rocket web framework as a dependency and allow us to build with it. Fire up your favourite code editor, which in my case is Visual Studio Code:

```bash
code .
```

Let's get the engine of this thing done first, type (or copy pasta) the following code:

```rust
// src/main.rs

// global import of all things Rocket

#[macro_use] extern crate rocket;

// dependencies

use rocket::fs::{FileServer, relative};

// main function

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("static")))
}
```

The comments make this more than 6 lines, I know.

What's going on?

- we bring the tools available to us in Rocket into scope with a global, `#[macro_use]`statement
- we bring a couple of rocket dependencies related to serving files into scope
- we build a main function which builds an index route and attaches files, held in a static folder, to it

This is all there is, Rust wise. Next, make a folder named static in the root of the project folder:

```bash
mkdir static
```

In this folder, all the static web content will be held, grabbed by the Rocket server, and served up at the "/" index route.

I'm not going to go into enormous detail creating a full website. You can do that as an exercise on your own. It is useful to show some CSS and JavaScript appropriately linked into our HTML, just to make it a little interesting. The JavaScript will run on page load and display the current year below a "Hello, World!" message.

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
     
    <link rel="stylesheet" type="text/css" href="screen.css" />
    <title>Hello World!</title>
  </head>
  <body>
     
    <h1>Hello, World!</h1>
     
    <h2 id="date"></h2>

    <script src="scripts.js"></script>
  </body>
</html>
```

If you're following along on your own, make your content and add some styling and JavaScript.

Alright, let's blast off!

At a terminal prompt, do a cargo check to make sure there are no errors in the code above. If you do receive any error messages, check for typos and see what the Rust compiler tells you might be the issue. When all cleaned up, do:

```bash
cargo run
```

The application will compile in debug mode. Rocket will display nicely formatted information about what it's doing. The application should load and serve up the site at localhost:8000.

![rocket launched](/assets/images/articles/images/rocket_launched.png)

Visit that with your web browser, and voila! a website!

![simple hello world website, served with Rocket](/assets/images/articles/images/rocket_website_output.png)

See? Rust doesn't have to be difficult. From here, it's pretty much straight-up vanilla web development. If serving static files is all you're interested in, then you'll not need to touch the Rust code again.

That's it! Thanks for reading! I've created a [GitHub repo](https://github.com/sentinel1909/rocket-web) for the code behind this article, feel free to use it as a basis for your own creations.

In the future, I'll show you how you can deploy your Rocket-based web creations to the world.
