use crate::Block;
use crate::Opcode;
use cranelift::prelude::{Block as JitBlock, *};

impl Block {
    pub fn render<'a>(&self, builder: &mut FunctionBuilder<'a>) -> JitBlock {
        let block = builder.create_block();
        builder.switch_to_block(block);
        builder.seal_block(block);
        for inst in &self.instructions {
            inst.render(builder)
        }
        builder.ins().trap(TrapCode::User(0));
        block
    }
}
