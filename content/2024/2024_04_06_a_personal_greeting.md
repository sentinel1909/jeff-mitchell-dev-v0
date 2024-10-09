---
title: "A Personal Greeting"
date: "2024-04-06"
slug: "personal-greeting"
category: "programming projects"
tag: "rust"
summary: "An article which explores the basics of input and output fron stdin and stdout, via a simple command line greeting program."
draft: false
edited: false
---

## Introduction

I struggle a lot with the problem solving aspect of programming. I have an unfortunate tendency to sit and expect the answer to birth from my forehead like Athena from Zeus.

It doesn't work that way.

It's an essential skill to be able to break the bigger thing down into small pieces, implement each piece one at a time, then putting it altogether.

Let's practice by looking at simple input and output using Rust.

The goal of this article is to write a simple program that takes the user's name and outputs a greeting message customized with their name. Very basic, yes, but it's an opportunity to explore some key concepts, namely:

- how to get input from the console
- how to sanitize the input
- how to do string concatenation
- how to write back to the console
- error handling

## Background

Before we start, let's take a short side trip. We're going to be working with `stdin` and `stdout`, directly, in our code. We're not going to use the `println` macro, as one typically would, to send our output to the console. The `println!` macro is a convenience and abstracts away from us most of plumbing. However, I want to understand the plumbing, so we'll be going lower.

_What is stdin_?

You can think of `stdin` as a standardized way of receiving a stream of information from the console that our program is running in. In this case, we're going to use `stdin` to get the users name after it's typed in. `stdin` is a programming convenience, you could define your own if you wanted to. We shall not go that low today though.

_What is stdout_?

`stdout` is pretty much the opposite of `stdin`, it gives us a standardized way of writing information to the console that our program is running in. We're not going to use the `println!` macro, as noted earlier. Part of the reason for this is that, in any large command line interface (CLI) program using `println!` incurs a performance hit and it's more efficient to write directly to `stdout`.

Alright, that's it. Let's code.

## Pseudocode

Our program won't have a lot to it, but here's some minimal pseudocode:

```rust
// program to greet a person by their first name

{

  output to the console, "What's your name?"

  receive the input,

  trim the input to remove any carriage returns or other similar characters

  concatenate the entered name with the message, "Hello "

  output the greeting message to the console.

}
```

## Real Code

Translating our pseudocode into real Rust, adding some necessary emblishments, we get:

```rust
// dependencies
use std::io::{self, Write};

// main function
fn main() -> io::Result<()> {

    // output to the console, "What's your name?"
    let mut stdout = io::stdout();
    writeln!(stdout, "What's your name?")?;

    // receive the input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // trim the input to remove any carriage returns or other similar characters
    let name = input.trim();
    let msg = format!("Hello, {}!", name);

    // output the greeting message to the console
    write!(stdout, "{}", msg)?;

    // end the program by returning the unit type () wrapped with an Ok()
    Ok(())
}
```

## What's Happening?

Let's break our code down, starting with our one dependency line:

- we bring the `std::io` module into scope, the `self` means we can refer to items without having to directly prefix them with `std::io`
- we bring the `Write` trait into scope, which brings us the `writeln!` and `write!` macros, enabling output without the potential performance hit from the `println!` macro
- we start our main function, which will return a std::io::Result<()> to allow us to gracefully handle any errors by using the Result<T, E> type
- our main function does the following:
- we open a handle to `stdout()` and bind it to a mutable variable named, `stdout`, it must be mutable so that it can be changed later by appending our output
- we pass our `stdout` handle, and the question "What's your name?" to the `writeln!` macro, which outputs the question
  - the write operation could fail here, so we use the `?` operator to gracefully handle either the error that might come out
- we declare a new mutable string variable and bind it to the name `input`, to receive our raw input
- we use the `stdin()` method, which accepts a reference to our freshly initialized `input` string variable
  - the read operation could fail here, so we use the `?` operator to gracefully handle either the error that might come out
- we trim the carriage return off the input, using the `trim()` method, and bind the result to a variable called `name`
- we use the `format!` macro to put together a greeting message with the name that was just received
- we write the output to the console, by calling the `write!` macro and giving it as arguments our handle to `stdout()` and the message we just created
  - the write operation could fail, so we again use the `?` operator to handle any errors
- the program ends by return a unit type (), wrapped in an `Ok()`
  - this is needed because our main function must return either an Ok value or an Error

## Moment of Truth

If all goes well, when you run the program, your output should look like:

![greetng program output](/assets/images/articles/images/a-personal-greeting-final-output.png)

I used [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=72d18345baf7164c97cd252b90689c71) while writing this article. Feel free to follow the link and play about on your own.

# Conclusion

That's it! A very simple program, but lot's of good concepts come to the surface here. Thanks for reading!
