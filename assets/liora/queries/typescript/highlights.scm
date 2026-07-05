; Liora TypeScript highlight query using Zed-compatible capture names.
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @boolean
(false) @boolean
["abstract" "as" "async" "await" "break" "case" "catch" "class" "const" "continue" "debugger" "declare" "default" "delete" "do" "else" "enum" "export" "extends" "finally" "for" "from" "function" "get" "if" "implements" "import" "in" "infer" "instanceof" "interface" "keyof" "let" "module" "namespace" "new" "of" "private" "protected" "public" "readonly" "return" "satisfies" "set" "static" "switch" "this" "throw" "try" "type" "typeof" "var" "void" "while" "with" "yield"] @keyword
(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function.call)
(call_expression function: (member_expression property: (property_identifier) @function.method.call))
(type_identifier) @type
(predefined_type) @type
(property_identifier) @property
