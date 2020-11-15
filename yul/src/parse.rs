use crate::SyntaxNode;
use rowan::GreenNode;

#[derive(Clone)]
pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn new(green_node: GreenNode) -> Self {
        Self { green_node }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    pub fn debug_tree(&self) -> String {
        let mut formatted = format!("{:#?}", self);
        formatted.pop(); // Cut trailing newline
        formatted
    }
}

impl core::fmt::Debug for Parse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.syntax_node().fmt(f)
    }
}

impl serde::Serialize for Parse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.syntax_node().serialize(serializer)
    }
}
