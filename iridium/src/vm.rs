use crate::instruction::Opcode;

pub struct VM {
  registers: [i32; 32],
  pc: usize,        // program counter
  program: Vec<u8>, // program bytecodes
}

impl VM {
  pub fn new() -> VM {
    VM {
      registers: [0; 32],
      program: vec![],
      pc: 0,
    }
  }

  pub fn run(&mut self) {
    loop {
      // If our program counter has exceeded the length of the program itself,
      // something has gone awry
      if self.pc >= self.program.len() {
        break;
      }
      match self.decode_opcode() {
        Opcode::HLT => {
          println!("HLT encountered");
          return;
        }
        _ => {
          println!("Unrecognized opcode found! Terminating!");
          return;
        }
      }
    }
  }

  fn decode_opcode(&mut self) -> Opcode {
    let opcode = Opcode::from(self.program[self.pc]);
    self.pc += 1;
    opcode
  }

  fn next_8_bits(&mut self) -> u8 {
    let result = self.program[self.pc];
    self.pc += 1;
    result
  }

  fn next_16_bits(&mut self) -> u16 {
    let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
    self.pc += 2;
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_vm() {
    let test_vm = VM::new();
    assert_eq!(test_vm.registers[0], 0)
  }

  #[test]
  fn test_opcode_hlt() {
    let mut test_vm = VM::new();
    let test_bytes = vec![0, 0, 0, 0];
    test_vm.program = test_bytes;
    test_vm.run();
    assert_eq!(test_vm.pc, 1);
  }

  #[test]
  fn test_opcode_igl() {
    let mut test_vm = VM::new();
    let test_bytes = vec![200, 0, 0, 0];
    test_vm.program = test_bytes;
    test_vm.run();
    assert_eq!(test_vm.pc, 1);
  }
}
