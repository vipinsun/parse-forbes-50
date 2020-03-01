# parse-forbes-50
The rust code here parses the text file downloaded from the Forbes article and creates a csv file with different columns, which can be loaded into any spreadsheet. It is generalized enough to This allows all the data manipulation functions that a spreadsheet allows.

Parses the input text file, using nom cargo. Writes using csv and serde-derive.

It is mostly for exploring nom as a parser writer; used to use lex and yacc for this years ago. But nom seems excellent since it uses the power of Rust. Will be exploring more of this.

