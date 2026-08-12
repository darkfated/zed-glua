; Function bodies
(function_declaration
  body: (_) @function.inside) @function.around

; Anonymous function expressions
(function_definition
  body: (_) @function.inside) @function.around

; Class definitions
(table_constructor) @class.inside @class.around

; Comments
(comment)+ @comment.around
