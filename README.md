# XDRGen

A tool for generating xdr friendly objects in various languages.

Implementation is in very early stages. Can parse simple .x files into
an AST.

To run and see debug output for AST:

```bash
cat idl/block.x  | cargo run
```