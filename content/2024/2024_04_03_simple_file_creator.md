---
title: "nf - A Simple File Creation CLI Tool"
date: "2024-04-03"
slug: "nf-file-creation-tool"
category: "programming projects"
tag: "rust"
summary: "An article describing a simple file creation command line interface tool I made using Rust."
draft: false
edited: false
---

# nf - A Simple File Creation CLI Tool

A common way of learning the ropes with Rust is to write a CLI tool. What is a CLI tool? It's a type of program that you interact with through the command line, by providing arguments that contain information you want the program to act on. I wanted something like the `touch` tool in Linux, but available under Windows. The Linux `touch` CLI tool enables creation of an empty file. Yes, the "ni" command exists in Powershell, but the point here was mostly to create something that was useful in a basic way that I could build on.

I christened my effort as `nf` short for "new file". Here's the code:

```rust
// nf main.rs

// dependencies
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io::{self, Write};

// struct to represent command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

// main function, program entry point
fn main() -> Result<()> {
    // initialize color_eyre
    color_eyre::install()?;

    // get the command line arguments
    let args = Args::parse();

    // create a handle for writing output
    let mut stdout = io::stdout();

    // create the new file
    let new_file = File::create(args.file);

    // write a success message or an error if the file couldn't be created
    match new_file {
        Ok(file) => write!(stdout, "{:#?} successfully created", file)?,
        Err(err) => write!(stdout, "{:#?}", err)?,
    }

    Ok(())
}
```

There are three main parts:

- dependencies

  - we pull in the [clap](https://crates.io/crates/clap) crate, specifically the Parser trait, more on how that's used in a moment...
  - the color_eyre crate is great to provide more colourful, nicely formatted error messages
  - the `File` struct from the `fs` module in the Rust standard library, this gives us an object which allows access to an open file on the filesystem
  - the `Write` trait from the `io` module in the Rust standard library, this gives us a way to write the file

- Args struct
  - Rust is all about the type system, we need a way to model command line arguments, we do this by defining a struct with one argument, to represent the file name
  - our struct has the `Parser` trait derived on it (from the clap crate) as well as the `Debug` trait from the Rust standard library
  - the command macro enables pulling in of information from Cargo.toml, which can display information about the program
- main function
  - the main function returns the Result<()> type from color_eyre, which, in the success case will be nothing (hence the use of the unit () type) or an error from `std::io::Error`
  - after initializing `color_eyre` we declare a variable called `args` and bind the incoming command line arguments to it, which in this case is the filename that needs to be created
  - we initialize a mutable variable called `stdout` which enables writing messages (either success or failure) to the console
  - we create the file by invoking the `create()` method available on the `File` struct and pass it our received command line argument, an bind it to a variable called `new_file`
  - the `create()` method could fail, so we need to match on it to determine the path forward
    - the `Ok()` match arm is the success case, we pass the file to the `write!` macro along with `stdout`, a message, and our file
    - the `Err()` match arm is the error case, we write an error message to `stdout`
  - the program terminates by returning an `Ok(())` wih the empty unit type if everything completes successfully

After doing `cargo build --release` to build a Windows executable, I put the file in a Utilities folder I have set up in the root of my C: drive, which allows me to run the tool from anywhere (possible because I added an environment variable which points to the previously mentioned Utilities folder). To use it type:

```bash
nf --file <filename>
```

In the future I plan on adding a `--directory` flag such that you can add directories as well.

It's kind of a dumb little thing, but it's mine and I can take some satisfaction from the fact I made it and adding too it is a further way to learn. Frequently, I have an issue with "creating things that are already made" and I need to just do things like this to get over it.

If you're interested to beg, borrow, or steal my code, it's here: [New File CLI Tool](https://github.com/sentinel1909/nf)
