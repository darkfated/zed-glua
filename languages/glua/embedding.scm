; Function declarations with leading comments (covers both `function` and `local function`)
((comment) @context
  .
  (function_declaration
    "function" @name
    name: (_) @name
    (comment) @collapse
    body: (block) @collapse) @item)

; Variable-assigned functions with leading comments
((comment) @context
  .
  (variable_declaration
    (assignment_statement
      (variable_list
        (identifier) @name)
      (expression_list
        (function_definition
          (comment) @collapse
          body: (block) @collapse)))) @item)
