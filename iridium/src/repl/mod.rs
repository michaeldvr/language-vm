use crate::vm::VM;
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

// Core structure for REPL for the Assembler
pub struct REPL {
  command_buffer: Vec<String>,
  // The VM the REPL will use to execute code
  vm: VM,
}

impl REPL {
  pub fn new() -> REPL {
    REPL {
      command_buffer: vec![],
      vm: VM::new(),
    }
  }

  pub fn run(&mut self) {
    println!("Welcome to Iridium! Let's be productive!");
    loop {
      // This allocates a new String in which to store whatever the user types
      // each iteration.
      // TODO: Move outside loop and re-use it every iteration
      let mut buffer = String::new();

      // Blocking call until the user types in a command
      let stdin = io::stdin();

      // Annoyingly, `print!` does not automatically flush stdout like `println!` does, so we
      // have to do that there for the user to see our `>>> ` prompt.
      print!(">>> ");
      io::stdout().flush().expect("Unable to flush stdout");

      // Here we'll look at the string the user gave us.
      stdin
        .read_line(&mut buffer)
        .expect("Unable to read line from user");
      let buffer = buffer.trim();

      self.command_buffer.push(buffer.to_string());

      match buffer {
        ".history" => {
          for cmd in &self.command_buffer {
            println!("{}", cmd);
          }
        }
        ".program" => {
          println!("Listing instructions currently in VM's program vector:");
          for instruction in &self.vm.program {
            println!("{}", instruction);
          }
          println!("End of program listing");
        }
        ".register" => {
          println!("Listing registers and all contents");
          println!("{:#?}", self.vm.registers);
          println!("End of Register Listing");
        }
        ".quit" => {
          println!("Bye!");
          std::process::exit(0);
        }
        _ => {
          let results = self.parse_hex(buffer);
          match results {
            Ok(bytes) => {
              for byte in bytes {
                self.vm.add_byte(byte);
              }
              self.vm.run_once();
            }
            Err(_e) => {
              println!("Unable to decode hex string.");
            }
          }
        }
      }
    }
  }

  fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
    let split = i.split(" ").collect::<Vec<&str>>();
    let mut results: Vec<u8> = vec![];
    for hex_string in split {
      let byte = u8::from_str_radix(&hex_string, 16);
      match byte {
        Ok(result) => {
          results.push(result);
        }
        Err(e) => {
          return Err(e);
        }
      }
    }
    Ok(results)
  }
}
