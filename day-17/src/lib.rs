use cached::proc_macro::cached;
use nom::{
  bytes::complete::tag,
  character::complete::{digit1, multispace0, newline},
  combinator::map_res,
  multi::separated_list0,
  sequence::{preceded, tuple},
  IResult,
};

const DATA: &str = include_str!("../input.txt");

#[derive(Debug, Clone)]
pub struct Computer {
  pub register_a: i64,
  pub register_b: i64,
  pub register_c: i64,
  pub instruction_pointer: usize,
}

impl Computer {
  pub fn reset(&mut self, a: i64, b: i64, c: i64) {
    self.register_a = a;
    self.register_b = b;
    self.register_c = c;
    self.instruction_pointer = 0;
  }

  pub fn run_program(&mut self, program: &[u8]) -> Vec<i64> {
    let mut output = Vec::new();

    while self.instruction_pointer + 1 < program.len() {
      let opcode = program[self.instruction_pointer];
      let operand = program[self.instruction_pointer + 1];
      let instruction = match opcode {
        0 => Instruction::Adv(operand),
        1 => Instruction::Bxl(operand),
        2 => Instruction::Bst(operand),
        3 => Instruction::Jnz(operand),
        4 => Instruction::Bxc(operand),
        5 => Instruction::Out(operand),
        6 => Instruction::Bdv(operand),
        7 => Instruction::Cdv(operand),
        _ => break,
      };

      let (new_a, new_b, new_c, next_pointer, mut partial_output) =
        execute_instruction(
          instruction,
          self.register_a,
          self.register_b,
          self.register_c,
          self.instruction_pointer,
        );

      self.register_a = new_a;
      self.register_b = new_b;
      self.register_c = new_c;
      self.instruction_pointer = next_pointer;

      output.append(&mut partial_output);
    }

    output
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
#[cfg(not(feature = "part2"))]
pub type Consequent = String;
#[cfg(feature = "part2")]
pub type Consequent = u64;


#[cached(size = 1_000_000)]
pub fn execute_instruction(
  instruction: Instruction,
  register_a: i64,
  register_b: i64,
  register_c: i64,
  instruction_pointer: usize,
) -> (i64, i64, i64, usize, Vec<i64>) {
  let mut output = Vec::new();
  let mut next_pointer = instruction_pointer + 2;

  match instruction {
    Instruction::Adv(operand) => {
      let divisor = 2_i64.pow(evaluate_combo_operand(
        register_a, register_b, register_c, operand,
      ) as u32);
      let new_register_a = if divisor != 0 {
        register_a / divisor
      } else {
        register_a
      };
      (new_register_a, register_b, register_c, next_pointer, output)
    }
    Instruction::Bxl(operand) => {
      let new_register_b = register_b ^ operand as i64;
      (register_a, new_register_b, register_c, next_pointer, output)
    }
    Instruction::Bst(operand) => {
      let value =
        evaluate_combo_operand(register_a, register_b, register_c, operand) % 8;
      (register_a, value, register_c, next_pointer, output)
    }
    Instruction::Jnz(operand) => {
      if register_a != 0 {
        next_pointer = operand as usize;
      }
      (register_a, register_b, register_c, next_pointer, output)
    }
    Instruction::Bxc(_) => {
      let new_register_b = register_b ^ register_c;
      (register_a, new_register_b, register_c, next_pointer, output)
    }
    Instruction::Out(operand) => {
      let value =
        evaluate_combo_operand(register_a, register_b, register_c, operand) % 8;
      output.push(value);
      (register_a, register_b, register_c, next_pointer, output)
    }
    Instruction::Bdv(operand) => {
      let divisor = 2_i64.pow(evaluate_combo_operand(
        register_a, register_b, register_c, operand,
      ) as u32);
      let new_register_b = if divisor != 0 {
        register_a / divisor
      } else {
        register_b
      };
      (register_a, new_register_b, register_c, next_pointer, output)
    }
    Instruction::Cdv(operand) => {
      let divisor = 2_i64.pow(evaluate_combo_operand(
        register_a, register_b, register_c, operand,
      ) as u32);
      let new_register_c = if divisor != 0 {
        register_a / divisor
      } else {
        register_c
      };
      (register_a, register_b, new_register_c, next_pointer, output)
    }
  }
}

fn evaluate_combo_operand(
  register_a: i64,
  register_b: i64,
  register_c: i64,
  operand: u8,
) -> i64 {
  match operand {
    0..=3 => operand as i64,
    4 => register_a,
    5 => register_b,
    6 => register_c,
    _ => unreachable!(),
  }
}

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

#[cfg(feature = "part2")]
pub fn find_min_register_a_to_duplicate_output(
  program: &[u8],
  a: u64,
  b: i64,
  c: i64,
  prg_pos: usize,
) -> Option<u64> {
  fn delta(digit: usize) -> u64 {
    2u64.pow(3 * digit as u32)
  }

  if prg_pos == usize::MAX {
    return Some(a);
  }

  for i in 0..8 {
    let candidate_a = a * 8 + i;

    let mut test_computer = Computer {
      register_a: candidate_a as i64,
      register_b: b,
      register_c: c,
      instruction_pointer: 0,
    };

    let output = test_computer.run_program(program);

    if let Some(first_digit_out) = output.first() {
      if *first_digit_out == program[prg_pos] as i64 {
        if prg_pos == 0 {
          return Some(candidate_a);
        }
        if let Some(result) = find_min_register_a_to_duplicate_output(
          program,
          candidate_a,
          b,
          c,
          prg_pos - 1,
        ) {
          return Some(result);
        }
      }
    }
  }

  None
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
  #[cfg(feature = "part2")]
  use crate::find_min_register_a_to_duplicate_output;
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

  #[cfg(not(feature = "part2"))]
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
  #[cfg(feature = "part2")]
  pub fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
    let program_len = data.program.len() as isize;

    match find_min_register_a_to_duplicate_output(
      &data.program,
      0, // Initial `a` value
      data.computer.register_b,
      data.computer.register_c,
      program_len - 1, // Start checking from the last position
    ) {
      Some(a) => Ok(a), // Return the valid `a` value
      None => Err("No valid register_a found".into()),
    }
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

  #[cfg(feature = "part2")]
  #[test]
  #[mry::lock(src_provider)] // Lock the function for mocking.
  fn test_transform() {
    mock_src_provider()
      .returns(Ok(include_str!("../sample.part2.txt").to_string()));

    let data = extract().expect("failed to extract data");
    dbg!(&data);

    let result = transform(data).expect("failed to transform data");

    assert_eq!(result, 117440);
  }

  // MARK load
}
