; Function declarations (covers both `function foo()` and `local function foo()`)
(function_declaration
  "function" @context
  name: (_) @name) @item

; Variable declarations (local x = function ... end)
(variable_declaration
  (assignment_statement
    (variable_list
      (identifier) @name)
    (expression_list
      (function_definition) @_fn))) @item
