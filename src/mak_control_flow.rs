use crate::mak_basic_block::*;

pub struct ControlFlowNode {
    pub block_: BasicBlock,
    pub branches_: Vec<ControlFlowNode>
}