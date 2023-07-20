mod opcode;
use opcode::Opcode;

#[derive(Debug, PartialEq)]
pub enum IR {
  SHR(u32), // >>>> === SHR(4)
  SHL(u32),
  ADD(u8), // ++++ === ADD(4)
  SUB(u8),
  PUTCHAR,
  GETCHAR,
  JIZ(u32), // Jump if zero
  JNZ(u32), // Jump if not zero
}

pub struct Code{
  pub instrs: Vec<IR>,
}
impl Code {
   pub fn from(data: Vec<opcode::Opcode>) -> Result<Self,Box<dyn std::error::Error>> {
    let mut instrs: Vec<IR> = Vec::new();
    let mut jstack: Vec<u32> = Vec::new();
    for e in data{
      match e {
        Opcode::SHR => {
          match instrs.last_mut() {
            Some(IR::SHR(x)) =>{
              *x +=1;
            }
            _ => {
              instrs.push(IR::SHR(1));
            }
          }
        },
        Opcode::SHL => {
          match instrs.last_mut() {
            Some(IR::SHL(x)) =>{
              *x +=1;
            }
            _=>{
              instrs.push(IR::SHL(1));
            }
          }
        },
        Opcode::ADD => {
          match instrs.last_mut() {
            Some(IR::ADD(x)) =>{
              let (b,_) = x.overflowing_add(1);
              *x = b;
            }
            _=>{
              instrs.push(IR::ADD(1));
            }
          }
        },
        Opcode::SUB => {
          match instrs.last_mut() {
            Some(IR::SUB(x)) =>{
              let (b,_) = x.overflowing_sub(1);
              *x = b;
            }
            _=>{
              instrs.push(IR::SUB(1));
            }
          }
        },
        Opcode::PUTCHAR => {
          instrs.push(IR::PUTCHAR);
        },
        Opcode::GETCHAR => {
          instrs.push(IR::GETCHAR);
        },
        Opcode::LB => {
          instrs.push(IR::JIZ(0));
          jstack.push((instrs.len()-1) as u32);
        },
        Opcode::RB => {
          let j = jstack.pop().ok_or("pop from empty list")?;
          instrs.push(IR::JIZ(j));
          let instrs_len = instrs.len();
          match &mut instrs[j as usize] {
            IR::JIZ(x) =>{
              *x=(instrs_len-1) as u32;
            }
            _=> {
              unreachable!()
            }
          }
        },
      }
    }
    Ok(Code { instrs })
  }
}

fn main(){

}