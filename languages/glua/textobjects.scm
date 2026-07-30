; Function bodies (covers both `function` and `local function`)
(function_declaration
  body: (_) @function.inside) @function.around

; Anonymous function expressions
(function_definition
  body: (_) @function.inside) @function.around

; Class definitions (tables assigned to variables)
(table_constructor) @class.inside

; Parameters
(parameters
  (identifier) @parameter.inside)

; Arguments
(arguments
  (_) @argument.inside) @argument.around

; Comments
(comment)+ @comment.around

; Return statements
(return_statement) @entry.inside

; If blocks
(if_statement) @entry.inside

; Loops
(while_statement) @entry.inside
(for_statement) @entry.inside
(repeat_statement) @entry.inside

; Do blocks
(do_statement) @entry.inside
