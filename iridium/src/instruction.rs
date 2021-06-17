#[derive(Debug, PartialEq)]
pub enum Opcode {
  LOAD, // load
  ADD,  // addition
  SUB,  // subtraction
  MUL,  // multiplication
  DIV,  // division
  HLT,  // halt
  JMP,  // jump
  JMPF, // jump forward
  JMPB, // jump backwad
  EQ,   // equal
  NEQ,  // not equal
  GT,   // greater than
  LT,   // less than
  GTQ,  // greater than or equal to
  LTQ,  // less than or equal to
  JEQ,  // jump if equal (from previous comparison)
  IGL,  // illegal
}

impl From<u8> for Opcode {
  fn from(v: u8) -> Self {
    match v {
      0 => Opcode::HLT,
      1 => Opcode::LOAD,
      2 => Opcode::ADD,
      3 => Opcode::SUB,
      4 => Opcode::MUL,
      5 => Opcode::DIV,
      6 => Opcode::JMP,
      7 => Opcode::JMPF,
      8 => Opcode::JMPB,
      9 => Opcode::EQ,
      10 => Opcode::NEQ,
      11 => Opcode::GT,
      12 => Opcode::LT,
      13 => Opcode::GTQ,
      14 => Opcode::LTQ,
      15 => Opcode::JEQ,
      _ => Opcode::IGL,
    }
  }
}

impl From<Opcode> for u8 {
  fn from(v: Opcode) -> Self {
    match v {
      Opcode::HLT => 0,
      Opcode::LOAD => 1,
      Opcode::ADD => 2,
      Opcode::SUB => 3,
      Opcode::MUL => 4,
      Opcode::DIV => 5,
      Opcode::JMP => 6,
      Opcode::JMPF => 7,
      Opcode::JMPB => 8,
      Opcode::EQ => 9,
      Opcode::NEQ => 10,
      Opcode::GT => 11,
      Opcode::LT => 12,
      Opcode::GTQ => 13,
      Opcode::LTQ => 14,
      Opcode::JEQ => 15,
      Opcode::IGL => 255,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
  opcode: Opcode,
}

impl Instruction {
  pub fn new(opcode: Opcode) -> Instruction {
    Instruction { opcode: opcode }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_hlt() {
    let opcode = Opcode::HLT;
    assert_eq!(opcode, Opcode::HLT);
  }

  #[test]
  fn test_create_instruction() {
    let instruction = Instruction::new(Opcode::HLT);
    assert_eq!(instruction.opcode, Opcode::HLT);
  }
}
