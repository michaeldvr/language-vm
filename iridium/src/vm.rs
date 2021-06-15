use crate::instruction::Opcode;

pub struct VM {
  registers: [i32; 32],
  pc: usize,        // program counter
  program: Vec<u8>, // program bytecodes
  remainder: u32,   // div remainder
}

impl VM {
  pub fn new() -> VM {
    VM {
      registers: [0; 32],
      pc: 0,
      program: vec![],
      remainder: 0,
    }
  }

  pub fn run(&mut self) {
    let mut is_done = false;
    while !is_done {
      is_done = self.execute_instruction();
    }
  }

  pub fn run_once(&mut self) {
    self.execute_instruction();
  }

  pub fn execute_instruction(&mut self) -> bool {
    // If our program counter has exceeded the length of the program itself,
    // something has gone awry
    // println!("{} {}", self.pc, self.program.len());
    if self.pc >= self.program.len() {
      return true;
    }
    match self.decode_opcode() {
      Opcode::LOAD => {
        let register = self.next_8_bits() as usize;
        let number = self.next_16_bits() as u32;
        self.registers[register] = number as i32;
        false
      }
      Opcode::ADD => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 + r2;
        true
      }
      Opcode::SUB => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 - r2;
        true
      }
      Opcode::MUL => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 * r2;
        true
      }
      Opcode::DIV => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 / r2;
        self.remainder = (r1 % r2) as u32;
        true
      }
      Opcode::HLT => {
        println!("HLT encountered");
        true
      }
      _ => {
        println!("Unrecognized opcode found! Terminating!");
        true
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
    let test_bytes = vec![Opcode::HLT as u8, 0, 0, 0];
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

  #[test]
  fn test_opcode_load() {
    let mut test_vm = VM::new();
    // storing 500 (1 1111 0100)
    // 0000 0001 (1) will be left shifted 8
    // 1111 0100 (244)
    test_vm.program = vec![1, 0, 1, 244];
    test_vm.run();
    assert_eq!(test_vm.registers[0], 500);
  }

  #[test]
  fn test_opcode_add() {
    let mut test_vm = VM::new();
    test_vm.program = vec![2, 0, 1, 2];
    test_vm.registers[0] = 200;
    test_vm.registers[1] = 300;
    test_vm.run();
    assert_eq!(test_vm.registers[2], 500);
  }

  #[test]
  fn test_opcode_sub() {
    let mut test_vm = VM::new();
    test_vm.program = vec![3, 0, 1, 2];
    test_vm.registers[0] = 30;
    test_vm.registers[1] = 12;
    test_vm.run();
    assert_eq!(test_vm.registers[2], 18);
  }

  #[test]
  fn test_opcode_mul() {
    let mut test_vm = VM::new();
    test_vm.program = vec![4, 0, 1, 2];
    test_vm.registers[0] = 15;
    test_vm.registers[1] = 4;
    test_vm.run();
    assert_eq!(test_vm.registers[2], 60);
  }

  #[test]
  fn test_opcode_div() {
    let mut test_vm = VM::new();
    test_vm.program = vec![5, 0, 1, 2];
    test_vm.registers[0] = 17;
    test_vm.registers[1] = 5;
    test_vm.run();
    assert_eq!(test_vm.registers[2], 3);
    assert_eq!(test_vm.remainder, 2);
  }
}
