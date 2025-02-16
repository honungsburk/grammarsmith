# Grammarsmith

Tools to build your own lexer and parsers by hand.

## Motivation

Creating lexers and parsers by hand turns out to not be that difficult, however I have found that there is some code I keep copying around.
This crate is a collection of those utilities so that I don't have to keep rewriting them.

## Installation

```toml
[dependencies]
grammarsmith = "0.1.0"
```

or if you want to use the `serde` features:

```toml
[dependencies]
grammarsmith = { version = "0.1.0", features = ["serde"] }
```

