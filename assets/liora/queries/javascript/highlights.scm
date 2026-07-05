; Liora JavaScript highlight query using Zed-compatible capture names.
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @boolean
(false) @boolean
["async" "await" "break" "case" "catch" "class" "const" "continue" "debugger" "default" "delete" "do" "else" "export" "extends" "finally" "for" "from" "function" "get" "if" "import" "in" "instanceof" "let" "new" "of" "return" "set" "static" "switch" "this" "throw" "try" "typeof" "var" "void" "while" "with" "yield"] @keyword
(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function.call)
(call_expression function: (member_expression property: (property_identifier) @function.method.call))
(property_identifier) @property
