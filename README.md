# XDR-Codegen

Xdr-js-serialize is a library for facilitating (de)serialization between the
[XDR](https://en.wikipedia.org/wiki/External_Data_Representation) format and
Javascript Dictionaries.

This repository is best used in tandom with [xdr-codegen](https://github.com/kochavalabs/xdr-codegen)
for anything beyond basic xdr manipulation.

## Usage

We currently support code generation for 3 languages: javascript, rust and go.
The generated code has the following dependencies:

- go: [go-xdr](https://github.com/stellar/go-xdr)
- rust: [xdr-rs-serialize](https://github.com/kochavalabs/xdr-rs-serialize)
- javascript: [xdr-js-serialize](https://github.com/kochavalabs/xdr-js-serialize)

```bash
# Javascript generation
cargo run test.x --language js # | eslint --stdin
# Rust generation
cargo run test.x --language rust # | rustfmt
# Go generation
cargo run test.x --language go # | gofmt
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
