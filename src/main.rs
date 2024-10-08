use std::collections::HashMap;
use std::vec;

use air::arch::Arch;
use air::instructions::builder::{self, InstructionBuilder};
use air::instructions::{BasicBlock, Blob, BlockParamData, CmpTy, Reg, RegData};
use types::{I64, VOID};

enum AArch64Instruction {
    Add { dst: Reg, src: Reg, imm: i64 },  // ADD Xd, Xn, #imm
    Sub { dst: Reg, src: Reg, imm: i64 },  // SUB Xd, Xn, #imm
    Mov { dst: Reg, imm: i64 },            // MOV Xd, #imm
    B { label: String },                   // B label
    Cbz { src: Reg, label: String },       // CBZ Xn, label
    Label { name: String },                // label:
}

struct State {
    blocks: HashMap<String, BasicBlock>
}

// Implement the translation function that converts an AArch64 instruction to the SSA-based IR.
fn translate_aarch64_to_ir(state: &mut State, builder: &mut InstructionBuilder, inst: AArch64Instruction) {
    match inst {
        AArch64Instruction::Add { dst, src, imm } => {
            // TODO: Translate ADD instruction to IR
            let imm0 = &builder.iconst(imm);
            let v0 = &builder.add(src, *imm0, I64);
            builder.write_reg(*v0, dst, I64);
        }
        AArch64Instruction::Sub { dst, src, imm } => {
            // TODO: Translate SUB instruction to IR
            let imm0 = &builder.iconst(-imm);
            let v0 = &builder.add(src, *imm0, I64);
            builder.write_reg(*v0, dst, I64);
        }
        AArch64Instruction::Mov { dst, imm } => {
            // TODO: Translate MOV instruction to IR
            let imm0 = &builder.iconst(imm);
            builder.write_reg(*imm0, dst, I64);
        }
        AArch64Instruction::B { label } => {
            // TODO: Translate B instruction to IR
            let block0 = get_and_add_block(builder, state, &label);
            builder.jump(block0, vec![]);
        }
        AArch64Instruction::Cbz { src, label } => {
            // TODO: Translate CBZ instruction to IR
            let block0 = get_and_add_block(builder, state, &label);
            let block1 = builder.current_block();
            let val0 = builder.read_reg(src, I64);
            let val1 = builder.iconst(0);
            let cond0 = builder.icmp(CmpTy::Eq, val0, val1, I64);
            builder.jumpif(cond0, block0, vec![], block1, vec![]);
        }
        AArch64Instruction::Label { name } => {
            // TODO: Translate label to IR
            let block0 = builder.create_block::<Vec<BlockParamData>, BlockParamData>(&name, vec![]);
            state.blocks.insert(name, block0);
            builder.jump(block0, vec![]);
        }
    }
}

// Gets a block by name from the state and creates block if block doesnt exist yet
fn get_and_add_block(builder: &mut InstructionBuilder, state: &mut State, label: &String) -> BasicBlock {
    if !state.blocks.contains_key(label) {
        let block0 = builder.create_block::<Vec<BlockParamData>, BlockParamData>(label, vec![]);
        state.blocks.insert(label.clone(), block0);
        block0
    }
    else {
        state.blocks[label]
    }
}

fn main() {
    let mut blob = Blob::new(Arch::new(I64, vec![
        RegData::new("x0", I64),
        RegData::new("x1", I64),
        RegData::new("x2", I64),
        RegData::new("x3", I64),
    ]));

    let x0 = Reg(0);
    let x1 = Reg(1);
    let x2 = Reg(2);
    let x3 = Reg(3);

    let mut builder = InstructionBuilder::new(&mut blob);
    let mut state = State {
        blocks: HashMap::new(),
    };
    

    // the following should be a loop decrementing a counter and branching to the start of the loop
    let instructions = vec![
        AArch64Instruction::Mov { dst: x0, imm: 10 },
        AArch64Instruction::Mov { dst: x1, imm: 1 },
        AArch64Instruction::Mov { dst: x2, imm: 0 },
        AArch64Instruction::Label { name: "start".to_string() },
        AArch64Instruction::Add { dst: x3, src: x2, imm: 1 },
        AArch64Instruction::Sub { dst: x0, src: x0, imm: 1 },
        AArch64Instruction::Cbz { src: x0, label: "end".to_string() },
        AArch64Instruction::B { label: "start".to_string() },
        AArch64Instruction::Label { name: "end".to_string() },
    ];

    for inst in instructions {
        translate_aarch64_to_ir(&mut state, &mut builder, inst);
    }
}
