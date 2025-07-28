# astgen
AST generator, built with python for parse table construction and Rust for lexer &amp; compiler.
Still under active development, will be completed soon.

## file format
**astgen** is constructed in two parts; the python code that analyzes a syntax and constructs a compiler-friendly parse table, and the C program that actually parses a source code.
```
[syntax.syn] -> tablegen.py -> [syntax.pstb syntax.map]
[syntax.pstb syntax.map syntax.tok] -> parser (.c)
```

### explanation
- syntax.syn: this file illustrates production rules of a syntax (variant of BNF)
- syntax.pstb: the parse table (action + goto table) => tokens are represented as IDs
- syntax.map: maps token names (strings) to their respective ID
- syntax.tok: illustrates lexical rules to tokenize source file

### example formats of each file

#### syntax.syn
```
plus mul id lparen rparen
E	E plus T
	T
T	T mul F
	F
F	id
	lparen E rparen
```

#### syntax.tok
```
id	"[_a-zA-Z][_a-zA-Z0-9]*"
plus	"\+"
mul	"\*"
lparen "\("
rparen "\)"
```

#### syntax.map
```
E	10
T	11
F	12
plus	13
mul	14
id	15
lparen	16
rparen	17
```
