use air::arch::Arch;
use air::instructions::builder::InstructionBuilder;
use air::instructions::{Blob, Reg, RegData};
use types::I64;

enum AArch64Instruction {
    Add { dst: Reg, src: Reg, imm: i64 },  // ADD Xd, Xn, #imm
    Sub { dst: Reg, src: Reg, imm: i64 },  // SUB Xd, Xn, #imm
    Mov { dst: Reg, imm: i64 },            // MOV Xd, #imm
    B { label: String },                   // B label
    Cbz { src: Reg, label: String },       // CBZ Xn, label
    Label { name: String },                // label:
}

// Implement the translation function that converts an AArch64 instruction to the SSA-based IR.
fn translate_aarch64_to_ir(builder: &mut InstructionBuilder, inst: AArch64Instruction) {
    match inst {
        AArch64Instruction::Add { dst, src, imm } => {
            // TODO: Translate ADD instruction to IR
        }
        AArch64Instruction::Sub { dst, src, imm } => {
            // TODO: Translate SUB instruction to IR
        }
        AArch64Instruction::Mov { dst, imm } => {
            // TODO: Translate MOV instruction to IR
        }
        AArch64Instruction::B { label } => {
            // TODO: Translate B instruction to IR
        }
        AArch64Instruction::Cbz { src, label } => {
            // TODO: Translate CBZ instruction to IR
        }
        AArch64Instruction::Label { name } => {
            // TODO: Translate label to IR
        }
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

    // the following should be a loop decrementing a counter and branching to the start of the loop
    let instructions = vec![
        AArch64Instruction::Mov { dst: x0, imm: 10 },
        AArch64Instruction::Mov { dst: x1, imm: 1 },
        AArch64Instruction::Mov { dst: x2, imm: 0 },
        AArch64Instruction::B { label: "start".to_string() },
        AArch64Instruction::Add { dst: x3, src: x2, imm: 1 },
        AArch64Instruction::Sub { dst: x0, src: x0, imm: 1 },
        AArch64Instruction::Cbz { src: x0, label: "end".to_string() },
        AArch64Instruction::B { label: "start".to_string() },
        AArch64Instruction::Label { name: "end".to_string() },
    ];

    for inst in instructions {
        translate_aarch64_to_ir(&mut builder, inst);
    }
}
