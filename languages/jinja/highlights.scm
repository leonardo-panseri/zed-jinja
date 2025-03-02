; adapted from https://github.com/cathaysia/tree-sitter-jinja/blob/41b17a33f335130ce9861fd21bffeb88fd768ef4/tree-sitter-jinja/queries/highlights.scm

[
  "{{"
  "{{-"
  "{{+"
  "+}}"
  "-}}"
  "}}"
  "{%"
  "{%-"
  "{%+"
  "+%}"
  "-%}"
  "%}"
] @keyword

[
  "include"
  "import"
  "from"
  "extends"
  "as"
] @keyword

[
  "if"
  "else"
  "endif"
  "elif"
] @keyword

[
  "for"
  "in"
  "continue"
  "break"
  "endfor"
] @keyword

[
  "block"
  "with"
  "filter"
  "macro"
  "set"
  "trans"
  "pluralize"
  "autoescape"
  "call"
] @keyword

[
  "endblock"
  "endwith"
  "endfilter"
  "endmacro"
  "endset"
  "endtrans"
  "endtrans"
  "endautoescape"
  "endcall"
] @keyword

[
  (attribute_ignore)
  (attribute_context)
  "recursive"
] @keyword

(do_statement
  "do" @keyword)

[
  ","
  "."
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "<"
  ">"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.list_marker

(string_literal) @string

(number_literal) @number

(float_literal) @number

(boolean_literal) @boolean

(null_literal) @constant

(comment) @comment

[
  (unary_operator)
  (binary_operator)
] @operator

((binary_operator) @keyword
    (#any-of? @keyword
    	"|"
        "is"))

(builtin_test
  [
    "boolean"
    "even"
    "in"
    "mapping"
    "sequence"
    "callable"
    "integer"
    "ne"
    "string"
    "defined"
    "filter"
    "iterable"
    "none"
    "test"
    "divisibleby"
    "float"
    "le"
    "number"
    "eq"
    "ge"
    "lower"
    "odd"
    "undefined"
    "escaped"
    "gt"
    "lt"
    "sameas"
    "upper"
  ] @operator)

(inline_trans
    "_" @function)

"debug" @function

(function_call
    (identifier) @function)

(function_call
  (arg
    (identifier) @attribute))

(import_statement
    (identifier) @variable)

(import_as
    (identifier) @variable)

(primary_expression
    (identifier) @variable)

(raw_block
    (raw_start) @keyword
    (raw_body) @embedded
    (raw_end) @keyword)
