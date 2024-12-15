#![allow(dead_code)]

use nom::{
  bytes::complete::tag,
  character::complete::{char, digit1, space0},
  combinator::{map, map_res, opt},
  multi::separated_list1,
  sequence::{pair, preceded, separated_pair, tuple},
  IResult,
};

use crate::object_zoo::{get_duration, prelude::*, ProblemDefinition};

fn signed_int(input: &str) -> IResult<&str, i8> {
  map_res(
    pair(opt(char('-')), digit1),
    |(sign, digits): (Option<_>, &str)| {
      digits
        .parse::<i8>()
        .map_err(|_| nom::error::Error::new(input, nom::error::ErrorKind::Fail))
        .map(|num| if sign.is_some() { -num } else { num })
    },
  )(input)
}

fn unsigned_int(input: &str) -> IResult<&str, u8> {
  map_res(digit1, |s: &str| s.parse::<u8>())(input)
}

fn point_u8(input: &str) -> IResult<&str, Point<u8>> {
  map(
    separated_pair(unsigned_int, char(','), unsigned_int),
    |(x, y)| Point { x, y },
  )(input)
}

fn point_i8(input: &str) -> IResult<&str, Point<i8>> {
  map(
    separated_pair(signed_int, char(','), signed_int),
    |(x, y)| Point { x, y },
  )(input)
}

fn robot(input: &str) -> IResult<&str, Robot> {
  map(
    tuple((
      preceded(tag("p="), point_u8),
      preceded(preceded(space0, tag("v=")), point_i8),
    )),
    |(position, velocity)| Robot { position, velocity },
  )(input)
}

pub fn parse_problem_definition(
  input: &str,
) -> IResult<&str, ProblemDefinition> {
  map(separated_list1(char('\n'), robot), |robots| {
    ProblemDefinition { robots, duration: get_duration() }
  })(input)
}
