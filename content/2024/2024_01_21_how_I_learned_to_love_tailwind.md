---
title: "How I Learned to Not Worry and Love Tailwind"
date: "2024-01-24"
slug: "how-i-learned-to-not-worry-and-love-tailwind"
category: "web development"
tag: "css frameworks"
summary: "A post about how I put my disdain for Tailwind aside."
draft: false
edited: false
---

Crow... I eat many black feathers today. I've recently been pushing a lot of negativity regarding Tailwind. I'm not too big to admit that my attitude comes from a place of ignorance, as well as outsized influence by opinions out there in internet land. I need to learn to make up my own mind based on my own experiences.

I embraced Tailwind today and added it into the toolbox. Truth is, I need it. I'm comically bad at visual design and can't create a decent looking web site to save my soul. Literally anything I create looks like it came from 1996. I need all the help I can get.

I believe you have to have a good, innate visual art style sense in order to be a web developer. I see now that Tailwind provides a framework in which to think. You can look at its output and educate yourself as to what it's doing, hopefully picking up some patterns over time so that usage makes more sense. The biggest issue for me with CSS is the lack of a mental model. No matter what I do, I just can't seem to build one.

Given that, I have two choices, either continue to be stubborn, and likely never build anything, or use a tool that can help me learn and grow. I wrote Tailwind off as a crutch a few days ago. Turns out I need the crutch.

The trick is to use that crutch in the simplest way possible.

I'm a Rust guy (duh, shocking) and am really trying to give myself no other choice but to use Rust as my main language. Tailwind always in my mind implied pooping JavaScript into my beautiful, clean Rust world. Somehow, I found out that there is a command line interface (CLI) for Tailwind which takes the JavaScript poop right off the table.

Even better yet, to my complete shock this morning, I discovered that the Trunk build tool has baked the Tailwind CLI right in! This means I don't even have to download and save the CLI as part of my project. Instructions to get up with Tailwind are right in the docs for Trunk.

Here's what your index.html would look like:

```html
<!DOCTYPE html>

<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link data-trunk rel="icon" type="image/icon" href="assets/favicon.ico" />
    <link
      data-trunk
      rel="tailwind-css"
      type="text/css"
      href="assets/screen.css"
      media="screen"
    />
    <title>Shuttle Leptos Template</title>
  </head>
  <body></body>
</html>
```

This example shows the index.html for a Leptos framework template I wrote earlier today. The key is the line with `data-trunk rel="tailwind-css"`. This is the clue to Trunk that it should invoke the Tailwind CLI as part of it's build process.

From here, you can add a tailwind.config.js file as you normally would:

```javascript
/** @type {import('tailwindcss').Config} */

module.exports = {
  content: ["index.html", "./src/**/*.rs"],

  theme: {
    extend: {},
  },

  plugins: [],
};
```

The final thing is to have the basic Tailwind directives in the screen.css file:

```css
@tailwind base;

@tailwind components;

@tailwind utilities;
```

This will trigger the Tailwind CLI to look through all your Rust code in the src folder and grab class definitions. Here's the main.rs as an example:

```rust
// src/main.rs


// dependencies

use leptos::*;

// the <App /> component
#[component]

fn App() -> impl IntoView {

    view! {

        <div class="bg-slate-200 h-screen flex flex-col justify-center items-center">
            <header>
                <h1 class="text-2xl">"Welcome to Leptos!"</h1>
            </header>
            <main>
                <section>
                    <article>
                        <p>"This template is written with the Leptos framework."</p>
                    </article>
                </section>
            </main>
        </div>
    }
}

// main function, renders the <App /> component into the page

fn main() {

    mount_to_body(|| view! { <App /> })

}
```

This template is extremely simple, using just a single `<App />` component. Tailwind classes are defined in the HTML that's written as part of the Leptos `view!` macro, which does the rendering work.

That's it. Any Rust frontend framework like Dioxis, Leptos, Sycamore, or Yew can use this method. They all use Trunk as a build tool, so you're good to go.

I enjoy web development in Rust quite a bit and, despite my Tailwind bashing, am glad there's a straightforward way to have access to this tool in my projects.
