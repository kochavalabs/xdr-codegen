# XDR Codegen

[![CircleCI](https://circleci.com/gh/kochavalabs/xdr-codegen.svg?style=svg)](https://circleci.com/gh/kochavalabs/xdr-codegen)

Xdr-codegen is a binary that is used to take the [XDR Language Specification](https://tools.ietf.org/html/rfc4506#section-6)
and generate source code in various languages. The goal of this is to facilitate
the communication of XDR objects across binaries that are written in different
languages. Another way to say this is we take .x files and convert them to the
appropriate js, go or rust source (protoc for XDR).

**Warning:** This project was put together to aid us in our development in a
short amount of time. There is still more work to be done before xdr-codegen
is completely compatible with the [XDR Language Specification](https://tools.ietf.org/html/rfc4506#section-6).

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
# Commonjs generation
cargo run test.x --language commonjs # | eslint --stdin
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
