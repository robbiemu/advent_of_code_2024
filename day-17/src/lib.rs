use cached::proc_macro::cached;
use nom::{
  bytes::complete::tag,
  character::complete::{digit1, multispace0, newline},
  combinator::map_res,
  multi::{many0, separated_list0},
  number::complete::u8,
  sequence::{preceded, tuple},
  IResult,
};


const DATA: &str = include_str!("../input.txt");

#[derive(Debug)]
pub struct Computer {
  pub register_a: i64,
  pub register_b: i64,
  pub register_c: i64,
  pub instruction_pointer: usize,
}

impl Computer {
  pub fn run_program(&mut self, program: &[u8]) -> Vec<i64> {
    let mut output = Vec::new();

    while self.instruction_pointer < program.len() {
      let instruction = match self.decode_instruction(program) {
        Some(inst) => inst,
        None => break,
      };

      self.execute_instruction(&instruction, &mut output);
    }

    output
  }

  fn decode_instruction(&self, program: &[u8]) -> Option<Instruction> {
    if self.instruction_pointer + 1 >= program.len() {
      return None;
    }

    let opcode = program[self.instruction_pointer];
    let operand = program[self.instruction_pointer + 1];

    match opcode {
      0 => Some(Instruction::Adv(operand)),
      1 => Some(Instruction::Bxl(operand)),
      2 => Some(Instruction::Bst(operand)),
      3 => Some(Instruction::Jnz(operand)),
      4 => Some(Instruction::Bxc(operand)),
      5 => Some(Instruction::Out(operand)),
      6 => Some(Instruction::Bdv(operand)),
      7 => Some(Instruction::Cdv(operand)),
      _ => None,
    }
  }

  fn evaluate_combo_operand(&self, operand: u8) -> i64 {
    match operand {
      0..=3 => operand as i64,
      4 => self.register_a,
      5 => self.register_b,
      6 => self.register_c,
      _ => unreachable!(), // Combo operand 7 is reserved
    }
  }

  fn execute_instruction(
    &mut self,
    instruction: &Instruction,
    output: &mut Vec<i64>,
  ) {
    match instruction {
      Instruction::Adv(operand) => {
        let divisor = 2_i64.pow(self.evaluate_combo_operand(*operand) as u32);
        if divisor != 0 {
          self.register_a /= divisor;
        }
      }
      Instruction::Bxl(operand) => {
        self.register_b ^= *operand as i64;
      }
      Instruction::Bst(operand) => {
        let value = self.evaluate_combo_operand(*operand) % 8;
        self.register_b = value;
      }
      Instruction::Jnz(operand) => {
        if self.register_a != 0 {
          self.instruction_pointer = *operand as usize;
          return;
        }
      }
      Instruction::Bxc(_) => {
        self.register_b ^= self.register_c;
      }
      Instruction::Out(operand) => {
        let value = self.evaluate_combo_operand(*operand) % 8;
        output.push(value);
      }
      Instruction::Bdv(operand) => {
        let divisor = 2_i64.pow(self.evaluate_combo_operand(*operand) as u32);
        if divisor != 0 {
          self.register_b = self.register_a / divisor;
        }
      }
      Instruction::Cdv(operand) => {
        let divisor = 2_i64.pow(self.evaluate_combo_operand(*operand) as u32);
        if divisor != 0 {
          self.register_c = self.register_a / divisor;
        }
      }
    }

    self.instruction_pointer += 2;
  }
}

#[derive(Debug)]
pub enum Instruction {
  Adv(u8),
  Bxl(u8),
  Bst(u8),
  Jnz(u8),
  Bxc(u8),
  Out(u8),
  Bdv(u8),
  Cdv(u8),
}

#[derive(Debug)]
pub struct ProblemDefinition {
  computer: Computer,
  program: Vec<u8>,
}
pub type Consequent = String;


fn parse_register_line<'a>(
  label: &'a str,
) -> impl Fn(&'a str) -> IResult<&'a str, i64> {
  move |input: &str| {
    let (input, _) = tag(label)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, value) = map_res(digit1, |s: &str| s.parse::<i64>())(input)?;
    let (input, _) = newline(input)?;
    Ok((input, value))
  }
}

pub fn parse_input(input: &str) -> IResult<&str, (Computer, Vec<u8>)> {
  // Parse registers A, B, and C
  let (input, (register_a, register_b, register_c)) = tuple((
    parse_register_line("Register A"),
    parse_register_line("Register B"),
    parse_register_line("Register C"),
  ))(input)?;

  let (input, _) = multispace0(input)?; // Skip any extra whitespace

  // Parse the raw program bytes
  let (input, program_bytes) = preceded(
    tag("Program: "),
    separated_list0(tag(","), map_res(digit1, |s: &str| s.parse::<u8>())),
  )(input)?;

  let (input, _) = multispace0(input)?; // Consume any trailing whitespace or newlines

  let computer =
    Computer { register_a, register_b, register_c, instruction_pointer: 0 };

  Ok((input, (computer, program_bytes)))
}

fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
  let (input, opcode) = u8(input)?;
  let (input, operand) = u8(input)?;

  let instruction = match opcode {
    0 => Instruction::Adv(operand),
    1 => Instruction::Bxl(operand),
    2 => Instruction::Bst(operand),
    3 => Instruction::Jnz(operand),
    4 => Instruction::Bxc(operand),
    5 => Instruction::Out(operand),
    6 => Instruction::Bdv(operand),
    7 => Instruction::Cdv(operand),
    _ => unreachable!(), // Invalid opcode
  };

  Ok((input, instruction))
}

fn parse_program(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
  many0(u8)(input)
}

#[cfg(test)]
#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}
#[cfg(not(test))]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

pub mod prelude {
  use crate::{parse_input, src_provider, Consequent, ProblemDefinition};

  pub fn extract() -> Result<ProblemDefinition, String> {
    let input = src_provider()?;
    let (unparsed, (computer, program)) =
      parse_input(&input).map_err(|_| "failed to parse input")?;

    if !unparsed.is_empty() {
      return Err(format!("unparsed input found: {unparsed}"));
    }

    Ok(ProblemDefinition { computer, program })
  }

  pub fn transform(mut data: ProblemDefinition) -> Result<Consequent, String> {
    Ok(
      data
        .computer
        .run_program(&data.program)
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(","),
    )
  }

  pub fn load(result: Result<Consequent, String>) -> Result<(), String> {
    let Ok(output) = result else { unreachable!() };
    println!("Output: {}", output);

    Ok(())
  }
}


#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::{prelude::*, *};

  // MARK extract
  // #[mry::lock(src_provider)] // Lock the function for mocking.

  // MARK transform
  #[cfg(not(feature = "part2"))]
  #[test]
  #[mry::lock(src_provider)] // Lock the function for mocking.
  fn test_transform() {
    mock_src_provider().returns(Ok(include_str!("../sample.txt").to_string()));

    let data = extract().expect("failed to extract data");
    dbg!(&data);

    let result = transform(data).expect("failed to transform data");

    assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
  }

  // MARK load
}
