; Statement blocks
(if_statement
  "end" @end) @indent

(do_statement
  "end" @end) @indent

(while_statement
  "end" @end) @indent

(for_statement
  "end" @end) @indent

(repeat_statement
  "until" @end) @indent

(function_declaration
  "end" @end) @indent

; Table constructors
(table_constructor
  [
    "{"
    "}"
  ] @end) @indent

; Bracket pairs
(_
  "["
  "]" @end) @indent

(_
  "("
  ")" @end) @indent
