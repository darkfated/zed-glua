; Comment prefix
(comment_prefix) @comment

; -- Visibility modifiers --
[
  "public"
  "private"
  "protected"
  "package"
] @keyword.modifier

; -- Type keywords --
[
  "fun"
  "async"
  "table"
  "keyof"
  "typeof"
  "extends"
  "in"
  "and"
  "or"
] @keyword.type

; -- Identifiers --
(identifier) @variable

; -- Built-in types --
((identifier) @type.builtin
  (#match? @type.builtin
    "^(string|number|integer|boolean|table|function|thread|userdata|nil|any|unknown|self)$"))

; -- Class definitions --
(class_annotation
  name: (identifier) @type.definition)

(class_annotation
  parent: (type_list
    (type
      (primary_type
        (basic_type
          (identifier) @type)))))

(class_annotation
  [
    "exact"
    "partial"
    "constructor"
  ] @keyword.modifier)

; -- Fields and parameters --
(field_annotation
  name: (field_name) @variable.member)

(field_annotation
  visibility: [
    "public"
    "private"
    "protected"
    "package"
  ] @keyword.modifier)

(param_annotation
  name: (param_name) @variable.parameter)

(param_annotation
  type: (type_annotation_value) @type)

(param_annotation
  description: (description) @comment)

(param_def
  name: (identifier) @variable.parameter)

(param_def
  type: (type) @type)

; -- Return annotations --
(return_annotation
  type: (return_type_annotation) @type)

(return_annotation
  description: (description) @comment)

; -- Generics --
(generic_annotation
  name: (identifier) @type.parameter)

(generic_annotation
  constraint: (type_annotation_value) @type)

(generic_type
  base: (identifier) @type)

(generic_type
  params: (generic_params_types
    (type) @type))

(generic_params
  params: (identifier) @type.parameter)

; -- Aliases and enums --
(alias_annotation
  name: (identifier) @type.definition)

(alias_annotation
  type: (type_annotation_value) @type)

(enum_annotation
  name: (identifier) @type.definition)

(enum_annotation
  "key" @keyword.modifier)

; -- Operators --
[
  "call"
  "add"
  "sub"
  "mul"
  "div"
  "mod"
  "pow"
  "concat"
  "len"
  "eq"
  "lt"
  "le"
  "unm"
  "bnot"
  "band"
  "bor"
  "bxor"
  "shl"
  "shr"
  "index"
] @operator

; -- Literals --
(string) @string
(number) @number
(boolean) @boolean
"nil" @constant.builtin

; -- Template types --
(template_chars) @string
(template_substitution
  "${" @punctuation.bracket
  type: (type) @type
  "}" @punctuation.bracket)

; -- Description text --
(text_line) @comment
(description) @comment
(continuation_description) @comment

(other_annotation
  description: (description) @string.documentation)

; -- Punctuation --
[
  ":"
  "|"
  ","
  "?"
  "&"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "<"
  ">"
  "{"
  "}"
] @punctuation.bracket

; -- Function type --
(function_type
  "async" @keyword)
(function_type
  "fun" @keyword.function)
(function_type
  params: (param_list
    (param_def
      name: (identifier) @variable.parameter
      type: (type) @type)))
(function_type
  return: (type_list
    (type) @type))

; -- Table type --
(table_type
  "table" @type.builtin)
(table_type
  key: (type) @type)
(table_type
  value: (type) @type)

; -- Table fields --
(table_field
  name: (identifier) @property)
(table_field
  type: (type_list
    (type) @type))

; -- Array type --
(array_type
  element: (primary_type) @type)

; -- Diagnostic --
(diagnostic_annotation
  action: [
    "disable"
    "enable"
    "disable-next-line"
    "disable-line"
  ] @keyword.directive)

(diagnostic_list
  (identifier) @constant)

; -- Module --
(module_annotation
  name: (string) @module)

; -- Version --
(version_annotation
  version: [
    (identifier) @constant
    (string) @string
    (version_range) @constant
  ])

; -- See references --
(see_annotation
  reference: (identifier) @variable)

; -- Namespace --
(namespace_annotation
  name: (identifier) @namespace)

; -- Using --
(using_annotation
  path: [
    (identifier) @namespace
    (string) @string
  ])

; -- Language --
(language_annotation
  language: (identifier) @constant)

; -- Cast --
(cast_annotation
  name: (identifier) @variable)
(cast_annotation
  type: (type_annotation_value) @type)

; -- Source --
(source_annotation
  source: (string) @string)

; -- As --
(as_annotation
  type: (type_annotation_value) @type)

; -- Other annotation tag --
(other_annotation
  tag: (tag_name) @keyword)

; -- Binary types --
(binary_type
  left: (primary_type) @type)
(binary_type
  op: ["&" "extends" "in"] @keyword.type)
(binary_type
  right: (primary_type) @type)

; -- Unary types --
(unary_type
  op: ["keyof" "typeof"] @keyword.type)
(unary_type
  argument: (primary_type) @type)

; -- Conditional type --
(conditional_type
  condition: (type) @type)
(conditional_type
  true_type: (type) @type)
(conditional_type
  false_type: (type) @type)

; -- Parenthesized type --
(parenthesized_type
  "(" @punctuation.bracket
  (type_list
    (type) @type)
  ")" @punctuation.bracket)

; -- Tuple type --
(tuple_type
  "[" @punctuation.bracket
  (tuple_elements
    (type_list
      (type) @type))
  "]" @punctuation.bracket)
