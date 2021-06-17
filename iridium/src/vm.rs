use crate::instruction::Opcode;

pub struct VM {
  // Array that simulates having hardware registers
  registers: [i32; 32],
  // Program counter that tracks which byte is being executed
  pc: usize,
  // The bytecode of the program being run
  program: Vec<u8>,
  // Contains remainder of module division ops
  remainder: u32,
  // Contains the result of the last comparison ops
  equal_flag: bool,
}

impl VM {
  pub fn new() -> VM {
    VM {
      registers: [0; 32],
      pc: 0,
      program: vec![],
      remainder: 0,
      equal_flag: false,
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
      }
      Opcode::ADD => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 + r2;
      }
      Opcode::SUB => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 - r2;
      }
      Opcode::MUL => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 * r2;
      }
      Opcode::DIV => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = r1 / r2;
        self.remainder = (r1 % r2) as u32;
      }
      Opcode::JMP => {
        let target = self.registers[self.next_8_bits() as usize];
        self.pc = target as usize;
      }
      Opcode::JMPF => {
        let value = self.registers[self.next_8_bits() as usize] as usize;
        self.pc += value;
      }
      Opcode::JMPB => {
        let value = self.registers[self.next_8_bits() as usize] as usize;
        self.pc -= value;
      }
      Opcode::EQ => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 == r2;
        self.next_8_bits(); // eat remaining 8 bits
      }
      Opcode::NEQ => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 != r2;
        self.next_8_bits(); // eat remaining 8 bits
      }
      Opcode::JEQ => {
        let target = self.registers[self.next_8_bits() as usize];
        if self.equal_flag {
          self.pc = target as usize; // jumps, remaining bits are skipped
        } else {
          self.next_16_bits(); // eat remaining 16 bits
        }
      }
      Opcode::GT => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 > r2;
        self.next_8_bits(); // eat remaining 8 bits
      }
      Opcode::LT => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 < r2;
        self.next_8_bits(); // eat remaining 8 bits
      }
      Opcode::GTQ => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 >= r2;
        self.next_8_bits(); // eat remaining 8 bits
      }
      Opcode::LTQ => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = r1 <= r2;
        self.next_8_bits(); // eat remaining 8 bits
      }
      Opcode::HLT => {
        println!("HLT encountered");
        return true;
      }
      _ => {
        println!("Unrecognized opcode found! Terminating!");
        return true;
      }
    }
    false
  }

  pub fn get_test_vm() -> VM {
    let mut test_vm = VM::new();
    test_vm.registers[0] = 5;
    test_vm.registers[1] = 10;
    return test_vm;
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
  fn test_hlt_opcode() {
    let mut test_vm = VM::new();
    let test_bytes = vec![u8::from(Opcode::HLT), 0, 0, 0];
    test_vm.program = test_bytes;
    test_vm.run();
    assert_eq!(test_vm.pc, 1);
  }

  #[test]
  fn test_igl_opcode() {
    let mut test_vm = VM::new();
    let test_bytes = vec![200, 0, 0, 0];
    test_vm.program = test_bytes;
    test_vm.run();
    assert_eq!(test_vm.pc, 1);
  }

  #[test]
  fn test_load_opcode() {
    let mut test_vm = VM::new();
    // storing 500 (1 1111 0100)
    // 0000 0001 (1) will be left shifted 8
    // 1111 0100 (244)
    test_vm.program = vec![u8::from(Opcode::LOAD), 0, 1, 244];
    test_vm.run();
    assert_eq!(test_vm.registers[0], 500);
  }

  #[test]
  fn test_add_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.program = vec![u8::from(Opcode::ADD), 0, 1, 2];
    test_vm.registers[0] = 2;
    test_vm.registers[1] = 3;
    test_vm.run();
    assert_eq!(test_vm.registers[2], 5);
  }

  #[test]
  fn test_sub_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.program = vec![u8::from(Opcode::SUB), 0, 1, 2];
    test_vm.registers[0] = 30;
    test_vm.registers[1] = 12;
    test_vm.run();
    assert_eq!(test_vm.registers[2], 18);
  }

  #[test]
  fn test_mul_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.program = vec![u8::from(Opcode::MUL), 0, 1, 2];
    test_vm.registers[0] = 15;
    test_vm.registers[1] = 4;
    test_vm.run();
    assert_eq!(test_vm.registers[2], 60);
  }

  #[test]
  fn test_div_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.program = vec![u8::from(Opcode::DIV), 0, 1, 2];
    test_vm.registers[0] = 17;
    test_vm.registers[1] = 5;
    test_vm.run();
    assert_eq!(test_vm.registers[2], 3);
    assert_eq!(test_vm.remainder, 2);
  }

  #[test]
  fn test_jump_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.registers[0] = 2;
    test_vm.program = vec![u8::from(Opcode::JMP), 0, 0, 0];
    test_vm.run_once();
    assert_eq!(test_vm.pc, 2);
  }

  #[test]
  fn test_jmpf_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.registers[0] = 2;
    test_vm.program = vec![u8::from(Opcode::JMPF), 0, 0, 0, 6, 0, 0, 0];
    test_vm.run_once();
    assert_eq!(test_vm.pc, 4);
  }

  #[test]
  fn test_jmpb_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.registers[1] = 6;
    test_vm.program = vec![
      u8::from(Opcode::LOAD),
      0,
      0,
      1,
      u8::from(Opcode::JMPB),
      1,
      0,
      0,
      0,
    ];
    test_vm.run_once();
    test_vm.run_once();
    assert_eq!(test_vm.pc, 0);
  }

  #[test]
  fn test_eq_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    let eq = u8::from(Opcode::EQ);
    test_vm.program = vec![eq, 0, 1, 0, eq, 0, 1, 0];
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
    test_vm.registers[1] = 11;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
  }

  #[test]
  fn test_neq_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    let neq = u8::from(Opcode::NEQ);
    test_vm.program = vec![neq, 0, 1, 0, neq, 0, 1, 0];
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
    test_vm.registers[1] = 11;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
  }

  #[test]
  fn test_jeq_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.registers[0] = 10;
    let jeq = u8::from(Opcode::JEQ);
    test_vm.program = vec![jeq, 0, 0, 0, jeq, 0, 0, 0, 0, 0, 0, 0];
    test_vm.equal_flag = false;
    test_vm.run_once();
    assert_ne!(test_vm.pc, 10);
    test_vm.equal_flag = true;
    test_vm.run_once();
    assert_eq!(test_vm.pc, 10);
  }

  #[test]
  fn test_gt_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    let op = u8::from(Opcode::GT);
    test_vm.program = vec![op, 0, 1, 0, op, 0, 1, 0];
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
    test_vm.registers[1] = 9;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
  }

  #[test]
  fn test_lt_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    let op = u8::from(Opcode::LT);
    test_vm.program = vec![op, 0, 1, 0, op, 0, 1, 0];
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
    test_vm.registers[1] = 11;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
  }

  #[test]
  fn test_gtq_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    let op = u8::from(Opcode::GTQ);
    test_vm.program = vec![op, 0, 1, 0, op, 0, 1, 0, op, 0, 1, 0];
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
    test_vm.registers[1] = 11;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
    test_vm.registers[1] = 9;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
  }

  #[test]
  fn test_ltq_opcode() {
    let mut test_vm = VM::get_test_vm();
    test_vm.registers[0] = 10;
    test_vm.registers[1] = 10;
    let op = u8::from(Opcode::LTQ);
    test_vm.program = vec![op, 0, 1, 0, op, 0, 1, 0, op, 0, 1, 0];
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
    test_vm.registers[1] = 11;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, true);
    test_vm.registers[1] = 9;
    test_vm.run_once();
    assert_eq!(test_vm.equal_flag, false);
  }
}
