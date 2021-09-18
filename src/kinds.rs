use crate::SyntaxKind;

// Nodes
pub const FN: SyntaxKind = SyntaxKind(1);
pub const NAME: SyntaxKind = SyntaxKind(2);
pub const PARAM_LIST: SyntaxKind = SyntaxKind(3);
pub const BIN_EXPR: SyntaxKind = SyntaxKind(4);
pub const FIELD: SyntaxKind = SyntaxKind(5);
pub const TYPE: SyntaxKind = SyntaxKind(6);
pub const STRUCT: SyntaxKind = SyntaxKind(7);

// Tokens
pub const WHITESPACE: SyntaxKind = SyntaxKind(100);
pub const IDENT: SyntaxKind = SyntaxKind(101);
pub const FN_KW: SyntaxKind = SyntaxKind(102);
pub const INT: SyntaxKind = SyntaxKind(103);
pub const PLUS: SyntaxKind = SyntaxKind(104);
pub const STAR: SyntaxKind = SyntaxKind(105);
pub const COMA: SyntaxKind = SyntaxKind(106);
pub const COLON: SyntaxKind = SyntaxKind(107);
pub const STRUCT_KW: SyntaxKind = SyntaxKind(108);
pub const L_CURLY: SyntaxKind = SyntaxKind(109);
pub const R_CURLY: SyntaxKind = SyntaxKind(110);
