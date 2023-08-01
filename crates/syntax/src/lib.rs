pub mod ast;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    Whitespace = 0,
    LineComment,
    BlockComment,

    Module,
    ModuleHeader,
    ModuleKw,
    WhereKw,

    ExportList,
    ImportList,

    ImportDeclaration,
    ImportKw,
    AsKw,

    ModuleName,
    QualifiedName,
    QualifiedPrefix,
    Name,
    NameRef,
    Upper,
    Lower,
    Hole,
    Operator,

    At,
    Equal,
    Period,
    Period2,
    Colon,
    Colon2,
    LeftArrow,
    RightArrow,
    LeftThickArrow,
    RightThickArrow,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftSquare,
    RightSquare,
    Tick,
    Comma,
    Pipe,
    Minus,

    AdoExpression,
    AdoKw,
    QualifiedAdo,
    DoExpression,
    DoKw,
    QualifiedDo,
    ApplicationExpression,
    TermArgument,
    TypeArgument,
    ConstructorExpression,
    LiteralExpression,
    IfThenElseExpression,
    IfKw,
    ThenKw,
    ElseKw,
    InfixChain,
    NegateExpression,
    OperatorChain,
    OperatorNameExpression,
    ParenthesizedExpression,
    TernaryExpression,
    TypedExpression,
    VariableExpression,

    LiteralChar,
    LiteralString,
    LiteralRawString,
    LiteralInteger,
    LiteralNumber,
    LiteralTrue,
    LiteralFalse,

    ConstructorType,
    ForallType,
    TypeVariableBinding,
    KindedType,
    VariableType,

    Pattern,

    ValueDeclaration,
    AnnotationDeclaration,

    DataDeclaration,
    DataKw,

    NewtypeDeclaration,
    NewtypeKw,
    ForallKw,

    TypeDeclaration,
    TypeKw,

    ClassDeclaration,
    ClassKw,

    InstanceDeclaration,
    InstanceKw,

    DeriveInstanceDeclaration,
    DeriveKw,

    ForeignDataDeclaration,
    ForeignValueDeclaration,
    ForeignKw,

    FixityDeclaration,
    InfixlKw,
    InfixrKw,
    InfixKw,

    /// Convenience node for patterns such as `l: e` or `e :: T`.
    Labeled,
    /// Convenience node for patterns such as `@variable` or `?hole`.
    Prefixed,
    /// Convenience node for patterns such as `( element )`.
    Wrapped,
    /// Convenience node for a non-empty array of elements.
    OneOrMore,
    /// Convenience node for a tuple of elements.
    Pair,

    Sentinel,
    Error,
    EndOfFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PureScript {}

impl rowan::Language for PureScript {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::EndOfFile as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(value: SyntaxKind) -> Self {
        Self(value as u16)
    }
}

pub type SyntaxNode = rowan::SyntaxNode<PureScript>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<PureScript>;
pub type SyntaxToken = rowan::SyntaxToken<PureScript>;
pub type SyntaxElement = rowan::SyntaxElement<PureScript>;

impl SyntaxKind {
    pub fn is_whitespace_or_comment(&self) -> bool {
        matches!(self, Self::Whitespace | Self::LineComment | Self::BlockComment)
    }
}

#[cfg(test)]
mod tests {
    use rowan::{ast::AstNode, NodeOrToken};

    use crate::{ast, SyntaxElement, SyntaxKind, SyntaxNode};

    fn print(indent: usize, element: SyntaxElement) {
        let kind: SyntaxKind = element.kind();
        print!("{:indent$}", "", indent = indent);
        match element {
            NodeOrToken::Node(node) => {
                println!("- {:?}", kind);
                for child in node.children_with_tokens() {
                    print(indent + 2, child);
                }
            }
            NodeOrToken::Token(token) => println!("- {:?} {:?}", token.text(), kind),
        }
    }

    #[test]
    fn syntax_playground() {
        let mut builder = rowan::GreenNodeBuilder::default();

        builder.start_node(SyntaxKind::Module.into());
        builder.start_node(SyntaxKind::ModuleHeader.into());
        builder.token(SyntaxKind::ModuleKw.into(), "module");
        builder.token(SyntaxKind::Whitespace.into(), " ");
        builder.start_node(SyntaxKind::ModuleName.into());
        builder.token(SyntaxKind::Upper.into(), "PureScript");
        builder.token(SyntaxKind::Period.into(), ".");
        builder.token(SyntaxKind::Upper.into(), "Main");
        builder.finish_node();
        builder.token(SyntaxKind::Whitespace.into(), " ");
        builder.start_node(SyntaxKind::ExportList.into());
        builder.token(SyntaxKind::LeftParenthesis.into(), "(");
        builder.token(SyntaxKind::RightParenthesis.into(), ")");
        builder.finish_node();
        builder.token(SyntaxKind::Whitespace.into(), " ");
        builder.token(SyntaxKind::WhereKw.into(), "where");
        builder.finish_node();
        builder.finish_node();

        let purescript_module = SyntaxNode::new_root(builder.finish());

        println!("{}", purescript_module);
        print(2, purescript_module.clone().into());

        let module_name = purescript_module
            .children()
            .next()
            .unwrap()
            .children()
            .next()
            .and_then(ast::ModuleName::cast)
            .unwrap();

        let rust_module = SyntaxNode::new_root(
            module_name
                .segments()
                .next()
                .unwrap()
                .replace_with(rowan::GreenToken::new(SyntaxKind::Upper.into(), "Rust")),
        );

        println!("{}", rust_module);
        print(2, rust_module.into());
    }
}
