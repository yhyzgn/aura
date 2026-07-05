; Liora Rust highlight query using Zed-compatible capture names.
(block_comment) @comment
(doc_comment) @comment
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string
(integer_literal) @number
(float_literal) @number
(boolean_literal) @boolean
(function_item name: (identifier) @function)
(call_expression function: (identifier) @function.call)
(macro_invocation macro: (identifier) @function.macro)
(type_identifier) @type
(primitive_type) @type
(field_identifier) @property
(lifetime) @attribute
