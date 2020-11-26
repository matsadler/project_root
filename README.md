# project_root

project_root finds the root directory of a project from the current working
directory, based upon a marker file.

I wanted to write a script that needed a name for the project the current
working directory is within, this could be taken from the name of the project
root directory. This script was to be run every time the working directory
changed (achievable with `function my_func--on-variable PWD` in `fish`), but my
first attempt as a shell script added too much lag when changing directories.

## Usage

**Rust**

    ~> pwd
    /Users/mat/src/some_rust_project/src/some_module
    ~> project_root Cargo.toml
    /Users/mat/src/some_rust_project

**Name Only**

    ~> pwd
    /Users/mat/src/some_rust_project/src/some_module
    ~> project_root --basename Cargo.toml
    some_rust_project

**Ruby**

    ~> pwd
    /Users/mat/src/some_rails_project/test/models
    ~> project_root Gemfile
    /Users/mat/src/some_rails_project

**JavaScript**

    ~> pwd
    /Users/mat/src/some_js_project/lib/some_module
    ~> project_root package.json
    /Users/mat/src/some_js_project

etc.

### Options

    USAGE:
        project_root [FLAGS] <FILE>

    FLAGS:
        -b, --basename    Return only the basename of the root directory
        -h, --help        Prints help information
        -V, --version     Prints version information
        -v, --verbose     Verbose mode, multiples increase the verbosity

    ARGS:
        <FILE>    Marker file that appears in the root directory

## Build

project_root is written in [Rust] 2018 Edition, using Rust 1.48. You can
install Rust using [rustup]. [Cargo] is used to build project_root and manage
dependencies. If you're new to Rust, [The Rust Programming Language][book] - an
introductory book about Rust - is available free online.

[Rust]: https://www.rust-lang.org/
[rustup]: https://www.rust-lang.org/en-US/install.html
[Cargo]: https://doc.rust-lang.org/stable/cargo/
[book]: https://doc.rust-lang.org/book/2018-edition/index.html

project_root can then be built for development with:

    cargo build

The binary will be written to `target/debug/project_root`.

Building for release can be done with:

    cargo build --release

The binary will be written to `target/release/project_root`.

## Install

Copy the binary from `target/release/project_root` to somewhere in your `$PATH`
for example:

    cp target/release/project_root /usr/local/bin/

## Docs

To see user documentation run:

    project_root --help

Developer documentation can be built with:

    cargo doc

The docs will be written to `target/doc/project_root`.

To build the docs and open them in your web browser run:

    cargo doc --open

## Troublshooting

Run project_root at maximum output verbosity with the `-vvvv` flag.
