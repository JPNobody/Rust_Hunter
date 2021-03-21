# Overview

This program is to simulate a hunter hunting deer. This program only handles the hunter looking for prey and waiting, along with if the hunter sees the prey, and if the hunter can hit the prey. The prey part of the simulation is handled by another Eli Jukes and you can check out his project [here](https://github.com/JukeOfHazzard/Rust-development)

The purpose of writing this program was to familiarize myself with the Rust programming language. By creating this project I was able to understand how to program in rust, and how it differs from other programming languages.


[Software Demo Video](https://youtu.be/_TJvKik-G80)

# Development Environment

For this project I used Visual Studio Code and rustc/Cargo to compile and run my rust programs. 

As previously stated, I used the Rust programming language, and I also used the [fastrand](https://crates.io/crates/fastrand) crate and the [text_io](https://crates.io/crates/text_io) crate.

The fastrand crate is used to get a random number, which I do to simulate rolling a twenty sided die.

The text_io crate is used to simplify getting input from the commandline.

# Useful Websites

* [Help getting Rust set up and running files](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
* [Rust tutorial site](https://www.tutorialspoint.com/rust/index.htm)
* [Official Rust site](https://www.rust-lang.org/)
* [Site for finding libraries(crates) for Rust](https://crates.io/)

# Future Work

* Make the software work with watching multiple prey.
* Add networking so that the program that runs the prey can interact with the program.
* Remove commandline prompts once the networking is done.