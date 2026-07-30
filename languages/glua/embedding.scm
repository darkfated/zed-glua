(function_declaration
  "function" @name
  name: (_) @name
  body: (block) @collapse) @item

(variable_declaration
  (assignment_statement
    (variable_list
      (identifier) @name)
    (expression_list
      (function_definition
        body: (block) @collapse)))) @item
