#![allow(dead_code)]

use z3::ast::{Ast, Bool};

pub trait BooleanOperations<'ctx> {
  fn and(self, other: &Bool<'ctx>) -> Bool<'ctx>;
  fn or(self, other: &Bool<'ctx>) -> Bool<'ctx>;
  fn not(self) -> Bool<'ctx>;
  fn implies(self, other: &Bool<'ctx>) -> Bool<'ctx>;
  fn iff(self, other: &Bool<'ctx>) -> Bool<'ctx>;
  fn ite<T: Ast<'ctx>>(self, then_value: &T, else_value: &T) -> T;

  fn tag(self, tag: &str) -> Bool<'ctx>;
}

// Implement the trait for Bool<'ctx>
impl<'ctx> BooleanOperations<'ctx> for Bool<'ctx> {
  fn and(self, other: &Bool<'ctx>) -> Bool<'ctx> {
    // Use the existing z3::ast::Bool::and function under the hood
    Bool::and(self.get_ctx(), &[&self, other])
  }

  fn or(self, other: &Bool<'ctx>) -> Bool<'ctx> {
    Bool::or(self.get_ctx(), &[&self, other])
  }

  fn not(self) -> Bool<'ctx> {
    Bool::not(&self)
  }

  fn implies(self, other: &Bool<'ctx>) -> Bool<'ctx> {
    Bool::implies(&self, other)
  }

  fn iff(self, other: &Bool<'ctx>) -> Bool<'ctx> {
    Bool::iff(&self, other)
  }

  fn ite<T: Ast<'ctx>>(self, then_value: &T, else_value: &T) -> T {
    Bool::ite(&self, then_value, else_value)
  }

  fn tag(self, tag: &str) -> Bool<'ctx> {
    let tagged = Bool::new_const(self.get_ctx(), tag);

    Bool::and(self.get_ctx(), &[&self, &tagged])
  }
}
