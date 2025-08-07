# astgen
AST generator, built with python for parse table construction and Rust for lexer &amp; compiler.

**astgen** accepts a JSON-style dialect of BNF context-free grammar and generates rust source code * `rustc` compiled binary that constructs an abstract syntax tree from a target source.

## Feature Notes
- accepts python dict, not JSON, in tablegen.py
- boilerplate & hard-coded parse tables in rust source

## To-do
- parse logic
- AST construction while parsing
- rust lexer
