# Cargo simple todo

## Why

Because
[probablyclem/cargo-todo](https://github.com/ProbablyClem/cargo-todo) is
a nice tool, but it does way too much for my taste.

## How to use it

Install it with `cargo install cargo-simple-todo`.

Every todo must be inside a `//` comment and must start with either
`@todo` or `@todo:` (case insensitve)

Example todo comments:

``` rust
// @todo: Do this thing

// @TodO do this weird thing

// Something @todo Do this
//@TODO : something else
```

Then execute `cargo todo`.

Try with `cargo todo --help` for further usage info.

## Exit code

-   If any todos are found, exit code `1` is returned to allow use in
    CI.

-   For general errors, exit code `255` is returned.

Thanks to
[orenbenkiki/cargo-todox](https://github.com/orenbenkiki/cargo-todox)
for the inspiration on this one.

## Warning

[probablyclem/cargo-todo](https://github.com/ProbablyClem/cargo-todo)
and `cargo-simple-todo` conflict, so you can't keep them both installed
(they both use the `cargo-todo` bin name).
