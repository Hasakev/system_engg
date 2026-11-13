//! e04: enums and `match`, by way of a tiny stack machine.
//!
//! A Rust enum is a tagged union: a small tag saying which variant is live,
//! followed by enough bytes for the largest variant.
//!
//! ```text
//!   Op::Push(7)   | tag = Push | 7 |
//!   Op::Add       | tag = Add  | - |
//! ```
//!
//! `match` reads the tag and forces you to handle every variant. That is the
//! decode step of every interpreter and emulator, including the CHIP-8 one
//! later in this repo.
//!
//! Machine code for this toy CPU, one byte per opcode:
//!
//! ```text
//!   0x00        Halt
//!   0x01 nn     Push nn      (the next byte is the value)
//!   0x02        Add          pop b, pop a, push a + b
//!   0x03        Mul          pop b, pop a, push a * b
//!   0x04        Dup          push a copy of the top
//!   0x05        Swap         exchange the top two
//!   0x06        Pop          discard the top
//! ```
//!
//! Tracing `01 02 01 03 02 04 03 00`, the stack grows to the right:
//!
//! ```text
//!   Push 2   [2]
//!   Push 3   [2, 3]
//!   Add      [5]
//!   Dup      [5, 5]
//!   Mul      [25]
//!   Halt     -> returns [25]
//! ```
//!
//! Before you code, write answers in ANSWERS.md:
//! 1. Guess `size_of::<Op>()`, then check it with a `println!`. Explain the number.
//! 2. What does the compiler do if you forget one variant in a `match`?

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Halt,
    Push(u8),
    Add,
    Mul,
    Dup,
    Swap,
    Pop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmError {
    /// `byte` at `offset` is not an opcode.
    BadOpcode { offset: usize, byte: u8 },
    /// The `Push` opcode at `offset` is the last byte, so it has no operand.
    Truncated { offset: usize },
    /// The instruction at index `pc` needed more values than the stack held.
    StackUnderflow { pc: usize },
    /// Execution ran past the last instruction without reaching `Halt`.
    NoHalt,
}

/// Turns machine code into instructions.
pub fn decode(bytes: &[u8]) -> Result<Vec<Op>, VmError> {
    todo!()
}

/// Executes `program` from index 0 and returns the stack (bottom first) when
/// it reaches `Halt`. Arithmetic wraps on overflow.
pub fn run(program: &[Op]) -> Result<Vec<i64>, VmError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_every_opcode() {
        let bytes = [0x01, 0x2A, 0x02, 0x03, 0x04, 0x05, 0x06, 0x00];
        assert_eq!(
            decode(&bytes),
            Ok(vec![
                Op::Push(42),
                Op::Add,
                Op::Mul,
                Op::Dup,
                Op::Swap,
                Op::Pop,
                Op::Halt
            ])
        );
        assert_eq!(decode(&[]), Ok(vec![]));
    }

    #[test]
    fn decode_operand_is_not_an_opcode() {
        // 0x09 would be a bad opcode, but here it is Push's operand.
        assert_eq!(decode(&[0x01, 0x09, 0x00]), Ok(vec![Op::Push(9), Op::Halt]));
    }

    #[test]
    fn decode_reports_bad_opcode_with_offset() {
        assert_eq!(
            decode(&[0x01, 0x01, 0x7F]),
            Err(VmError::BadOpcode {
                offset: 2,
                byte: 0x7F
            })
        );
    }

    #[test]
    fn decode_reports_truncated_push() {
        assert_eq!(decode(&[0x02, 0x01]), Err(VmError::Truncated { offset: 1 }));
    }

    #[test]
    fn run_the_traced_program() {
        let program = decode(&[0x01, 0x02, 0x01, 0x03, 0x02, 0x04, 0x03, 0x00]).unwrap();
        assert_eq!(run(&program), Ok(vec![25]));
    }

    #[test]
    fn run_swap_and_pop() {
        let program = [Op::Push(1), Op::Push(2), Op::Swap, Op::Pop, Op::Halt];
        assert_eq!(run(&program), Ok(vec![2]));
    }

    #[test]
    fn run_stops_at_halt() {
        let program = [Op::Push(1), Op::Halt, Op::Add];
        assert_eq!(run(&program), Ok(vec![1]));
    }

    #[test]
    fn run_reports_underflow_with_pc() {
        assert_eq!(
            run(&[Op::Push(1), Op::Add, Op::Halt]),
            Err(VmError::StackUnderflow { pc: 1 })
        );
        assert_eq!(run(&[Op::Dup]), Err(VmError::StackUnderflow { pc: 0 }));
        assert_eq!(
            run(&[Op::Push(1), Op::Swap]),
            Err(VmError::StackUnderflow { pc: 1 })
        );
        assert_eq!(run(&[Op::Pop]), Err(VmError::StackUnderflow { pc: 0 }));
    }

    #[test]
    fn run_requires_halt() {
        assert_eq!(run(&[Op::Push(1)]), Err(VmError::NoHalt));
        assert_eq!(run(&[]), Err(VmError::NoHalt));
    }
}
