use z3::{
  ast::{self, Ast, Bool, Int},
  Config, Context, SatResult, Solver,
};

use crate::boolean_operations::BooleanOperations;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
  // Numeric output keys (new integer mappings needed)
  Zero,  // 0
  One,   // 1
  Two,   // 2
  Three, // 3
  Four,  // 4
  Five,  // 5
  Six,   // 6
  Seven, // 7
  Eight, // 8
  Nine,  // 9

  A, // 10

  // Movement keys (already mapped in constraints)
  Left,  // 11
  Right, // 12
  Up,    // 13
  Down,  // 14
}

impl std::fmt::Display for Key {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Key::One => write!(f, "1"),
      Key::Two => write!(f, "2"),
      Key::Three => write!(f, "3"),
      Key::Four => write!(f, "4"),
      Key::Five => write!(f, "5"),
      Key::Six => write!(f, "6"),
      Key::Seven => write!(f, "7"),
      Key::Eight => write!(f, "8"),
      Key::Nine => write!(f, "9"),
      Key::Zero => write!(f, "0"),
      Key::A => write!(f, "A"),
      Key::Left => write!(f, "<"),
      Key::Right => write!(f, ">"),
      Key::Up => write!(f, "^"),
      Key::Down => write!(f, "v"),
    }
  }
}

impl Key {
  pub fn from_i64(value: i64) -> Option<Key> {
    match value {
      0 => Some(Key::Zero),
      1 => Some(Key::One),
      2 => Some(Key::Two),
      3 => Some(Key::Three),
      4 => Some(Key::Four),
      5 => Some(Key::Five),
      6 => Some(Key::Six),
      7 => Some(Key::Seven),
      8 => Some(Key::Eight),
      9 => Some(Key::Nine),
      10 => Some(Key::A),
      11 => Some(Key::Left),
      12 => Some(Key::Right),
      13 => Some(Key::Up),
      14 => Some(Key::Down),
      _ => None,
    }
  }

  pub fn from_char(c: char) -> Option<Key> {
    match c {
      '1' => Some(Key::One),
      '2' => Some(Key::Two),
      '3' => Some(Key::Three),
      '4' => Some(Key::Four),
      '5' => Some(Key::Five),
      '6' => Some(Key::Six),
      '7' => Some(Key::Seven),
      '8' => Some(Key::Eight),
      '9' => Some(Key::Nine),
      '0' => Some(Key::Zero),
      'A' => Some(Key::A),
      '<' => Some(Key::Left),
      '>' => Some(Key::Right),
      '^' => Some(Key::Up),
      'v' => Some(Key::Down),
      _ => None,
    }
  }
}

#[derive(Debug, Clone)]
pub enum KeypadType {
  Numeric,
  Direction,
  Input,
}

impl KeypadType {
  fn initial_pos(&self) -> (i64, i64) {
    match self {
      KeypadType::Numeric => (2, 3),
      KeypadType::Direction => (2, 0),
      KeypadType::Input => (0, 0),
    }
  }

  fn is_in_bounds<'ctx>(
    &self,
    pos: (ast::Int<'ctx>, ast::Int<'ctx>),
  ) -> ast::Bool<'ctx> {
    let (x, y) = pos;
    let ctx = x.get_ctx();

    match self {
      KeypadType::Numeric => {
        // For layout:
        // 7 8 9  (0,0) (1,0) (2,0)
        // 4 5 6  (0,1) (1,1) (2,1)
        // 1 2 3  (0,2) (1,2) (2,2)
        //   0 A      (1,3) (2,3)

        // Valid x-coordinate based on y position
        let valid_x = y.le(&Int::from_i64(ctx, 2)).ite(
          // For y <= 2 (main grid): 0 <= x <= 2
          &x.ge(&Int::from_i64(ctx, 0))
            .and(&x.le(&Int::from_i64(ctx, 2))),
          // For y = 3 (bottom row): 1 <= x <= 2
          &x.ge(&Int::from_i64(ctx, 1))
            .and(&x.le(&Int::from_i64(ctx, 2))),
        );

        // Valid y-coordinate: 0 <= y <= 3
        let valid_y = y
          .ge(&Int::from_i64(ctx, 0))
          .and(&y.le(&Int::from_i64(ctx, 3)));

        // Position is valid if both x and y are valid
        valid_x.and(&valid_y)
      }
      KeypadType::Direction => {
        // For layout:
        //   ^ A    (1,0) (2,0)
        // < v >  (0,1) (1,1) (2,1)

        // Top row: (x >= 1 && x <= 2 && y == 0)
        let top_row = x
          .ge(&Int::from_i64(ctx, 1))
          .and(&x.le(&Int::from_i64(ctx, 2)))
          .and(&y._eq(&Int::from_i64(ctx, 0)));

        // Bottom row: (x >= 0 && x <= 2 && y == 1)
        let bottom_row = x
          .ge(&Int::from_i64(ctx, 0))
          .and(&x.le(&Int::from_i64(ctx, 2)))
          .and(&y._eq(&Int::from_i64(ctx, 1)));

        top_row.or(&bottom_row)
      }
      KeypadType::Input => {
        // No movement for input keypad
        Bool::from_bool(ctx, true)
      }
    }
  }

  fn get_key<'ctx>(
    &self,
    pos: (ast::Int<'ctx>, ast::Int<'ctx>),
  ) -> Option<ast::Int<'ctx>> {
    let (x, y) = pos;
    let ctx = x.get_ctx();

    match self {
      KeypadType::Direction => {
        // Direction keypad layout:
        //   ^ A    (1,0) (2,0)
        // < v >  (0,1) (1,1) (2,1)

        let at_up = x
          ._eq(&Int::from_i64(ctx, 1))
          .and(&y._eq(&Int::from_i64(ctx, 0)));
        let at_a = x
          ._eq(&Int::from_i64(ctx, 2))
          .and(&y._eq(&Int::from_i64(ctx, 0)));
        let at_left = x
          ._eq(&Int::from_i64(ctx, 0))
          .and(&y._eq(&Int::from_i64(ctx, 1)));
        let at_down = x
          ._eq(&Int::from_i64(ctx, 1))
          .and(&y._eq(&Int::from_i64(ctx, 1)));
        let at_right = x
          ._eq(&Int::from_i64(ctx, 2))
          .and(&y._eq(&Int::from_i64(ctx, 1)));

        let key = at_up.ite(
          &Int::from_i64(ctx, Key::Up as i64),
          &at_a.ite(
            &Int::from_i64(ctx, Key::A as i64),
            &at_left.ite(
              &Int::from_i64(ctx, Key::Left as i64),
              &at_down.ite(
                &Int::from_i64(ctx, Key::Down as i64),
                &at_right.ite(
                  &Int::from_i64(ctx, Key::Right as i64),
                  &Int::from_i64(ctx, -1),
                ),
              ),
            ),
          ),
        );

        Some(key)
      }
      KeypadType::Numeric => {
        // For layout:
        // 7 8 9  (0,0) (1,0) (2,0)
        // 4 5 6  (0,1) (1,1) (2,1)
        // 1 2 3  (0,2) (1,2) (2,2)
        //   0 A      (1,3) (2,3)

        let at_7 = x
          ._eq(&Int::from_i64(ctx, 0))
          .and(&y._eq(&Int::from_i64(ctx, 0)));
        let at_8 = x
          ._eq(&Int::from_i64(ctx, 1))
          .and(&y._eq(&Int::from_i64(ctx, 0)));
        let at_9 = x
          ._eq(&Int::from_i64(ctx, 2))
          .and(&y._eq(&Int::from_i64(ctx, 0)));

        let at_4 = x
          ._eq(&Int::from_i64(ctx, 0))
          .and(&y._eq(&Int::from_i64(ctx, 1)));
        let at_5 = x
          ._eq(&Int::from_i64(ctx, 1))
          .and(&y._eq(&Int::from_i64(ctx, 1)));
        let at_6 = x
          ._eq(&Int::from_i64(ctx, 2))
          .and(&y._eq(&Int::from_i64(ctx, 1)));

        let at_1 = x
          ._eq(&Int::from_i64(ctx, 0))
          .and(&y._eq(&Int::from_i64(ctx, 2)));
        let at_2 = x
          ._eq(&Int::from_i64(ctx, 1))
          .and(&y._eq(&Int::from_i64(ctx, 2)));
        let at_3 = x
          ._eq(&Int::from_i64(ctx, 2))
          .and(&y._eq(&Int::from_i64(ctx, 2)));

        let at_0 = x
          ._eq(&Int::from_i64(ctx, 1))
          .and(&y._eq(&Int::from_i64(ctx, 3)));
        let at_a = x
          ._eq(&Int::from_i64(ctx, 2))
          .and(&y._eq(&Int::from_i64(ctx, 3)));

        let key = at_7.ite(
          &Int::from_i64(ctx, Key::Seven as i64),
          &at_8.ite(
            &Int::from_i64(ctx, Key::Eight as i64),
            &at_9.ite(
              &Int::from_i64(ctx, Key::Nine as i64),
              &at_4.ite(
                &Int::from_i64(ctx, Key::Four as i64),
                &at_5.ite(
                  &Int::from_i64(ctx, Key::Five as i64),
                  &at_6.ite(
                    &Int::from_i64(ctx, Key::Six as i64),
                    &at_1.ite(
                      &Int::from_i64(ctx, Key::One as i64),
                      &at_2.ite(
                        &Int::from_i64(ctx, Key::Two as i64),
                        &at_3.ite(
                          &Int::from_i64(ctx, Key::Three as i64),
                          &at_0.ite(
                            &Int::from_i64(ctx, Key::Zero as i64),
                            &at_a.ite(
                              &Int::from_i64(ctx, Key::A as i64),
                              &Int::from_i64(ctx, -1),
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        );

        Some(key)
      }
      KeypadType::Input => None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Robot {
  pub keypad: KeypadType,
  pub id: usize,
}

#[derive(Debug, Clone)]
pub struct StepState<'ctx> {
  // Current positions of all robots
  positions: Vec<(ast::Int<'ctx>, ast::Int<'ctx>)>,
  // The input keys to be processed by each robot
  input_keys: Vec<ast::Int<'ctx>>,
  // Output keys for each robot at this step
  output: ast::Int<'ctx>,
}

pub struct SolutionStep {
  pub robot_inputs: Vec<i64>, // Input for each robot at this step
  pub output: i64,            // Output value for this step
  pub positions: Vec<(i64, i64)>, // Position of each robot
}

pub struct KeypadSolver<'ctx> {
  ctx: &'ctx Context,
  solver: Solver<'ctx>,
  robots: Vec<Robot>,
  max_steps: usize,
  step_states: Vec<StepState<'ctx>>,
  target_sequence: Vec<Key>,
}

impl<'ctx> KeypadSolver<'ctx> {
  pub fn new(
    ctx: &'ctx Context,
    robots: Vec<Robot>,
    target_sequence: Vec<Key>,
    max_steps: usize,
  ) -> Self {
    let solver = Solver::new(ctx);
    let step_states = Vec::with_capacity(max_steps);

    Self { ctx, solver, robots, target_sequence, step_states, max_steps }
  }

  pub fn solve_detailed(&mut self) -> Result<Vec<SolutionStep>, String> {
    // Run the solver
    if self.solver.check() == SatResult::Sat {
      let model = self.solver.get_model().unwrap();
      let mut solution = Vec::new();

      // Extract all state for each step
      for step in 0..self.max_steps {
        // Get inputs for all robots
        let mut robot_inputs = Vec::new();
        for input_key in &self.step_states[step].input_keys {
          let input_val =
            model.eval(input_key, true).unwrap().as_i64().unwrap();
          robot_inputs.push(input_val);
        }

        // Get output
        let output = model
          .eval(&self.step_states[step].output, true)
          .unwrap()
          .as_i64()
          .unwrap();

        // Get positions
        let mut positions = Vec::new();
        for (x, y) in &self.step_states[step].positions {
          let x_val = model.eval(x, true).unwrap().as_i64().unwrap();
          let y_val = model.eval(y, true).unwrap().as_i64().unwrap();
          positions.push((x_val, y_val));
        }

        solution.push(SolutionStep { robot_inputs, output, positions });
      }

      Ok(solution)
    } else {
      Err("no solution found".to_string())
    }
  }

  pub fn minimize(&mut self) -> Result<Vec<SolutionStep>, String> {
    // Count non -1 inputs for Robot 0 across all steps
    let sequence_length = (0..self.step_states.len())
      .map(|step| {
        let input = &self.step_states[step].input_keys[0]; // Robot 0's input
        let is_actual_input = input._eq(&Int::from_i64(self.ctx, -1)).not();
        is_actual_input
          .ite(&Int::from_i64(self.ctx, 1), &Int::from_i64(self.ctx, 0))
      })
      .fold(Int::from_i64(self.ctx, 0), |acc, x| acc + x);

    // Create optimizer and add all existing constraints
    let optimizer = z3::Optimize::new(self.ctx);
    for assertion in self.solver.get_assertions() {
      optimizer.assert(&assertion);
    }

    // Add minimization objective
    optimizer.minimize(&sequence_length);

    // Check and extract solution
    if optimizer.check(&[]) == SatResult::Sat {
      let model = optimizer.get_model().unwrap();

      // Extract the detailed solution steps
      let mut solution = Vec::new();
      for step in 0..self.max_steps {
        // Get inputs for all robots
        let mut robot_inputs = Vec::new();
        for input_key in &self.step_states[step].input_keys {
          let input_val =
            model.eval(input_key, true).unwrap().as_i64().unwrap();
          robot_inputs.push(input_val);
        }

        // Get output
        let output = model
          .eval(&self.step_states[step].output, true)
          .unwrap()
          .as_i64()
          .unwrap();

        // Get positions
        let mut positions = Vec::new();
        for (x, y) in &self.step_states[step].positions {
          let x_val = model.eval(x, true).unwrap().as_i64().unwrap();
          let y_val = model.eval(y, true).unwrap().as_i64().unwrap();
          positions.push((x_val, y_val));
        }

        solution.push(SolutionStep { robot_inputs, output, positions });
      }

      Ok(solution)
    } else {
      Err("no solution found".to_string())
    }
  }

  pub fn bs_minimize(
    &mut self,
    seed_inputs: Option<Vec<Option<Key>>>,
  ) -> Result<Vec<SolutionStep>, String> {
    let mut min_steps = self.target_sequence.len();
    let mut max_steps = self.max_steps;
    let mut best_solution = None;

    #[cfg(feature = "debug")]
    println!(
      "Starting binary search (min_steps: {}, max_steps: {})",
      min_steps, max_steps
    );

    while min_steps <= max_steps {
      let try_steps = (min_steps + max_steps) / 2;
      #[cfg(feature = "debug")]
      println!(
        "Trying {} steps (range: {}-{})",
        try_steps, min_steps, max_steps
      );

      // Create fresh context and config for each attempt
      let cfg = Config::new();
      let ctx = Context::new(&cfg);
      let mut solver = KeypadSolver::new(
        &ctx,
        self.robots.clone(),
        self.target_sequence.clone(),
        try_steps,
      );

      match solver.prep(seed_inputs.clone()) {
        Ok(_) => match solver.solver.check() {
          SatResult::Sat => {
            if let Ok(solution) = solver.solve_detailed() {
              let actual_input_count = solution
                .iter()
                .filter(|step| step.robot_inputs[0] != -1)
                .count();

              let outputs: Vec<Key> = solution
                .iter()
                .filter(|step| step.output != -1)
                .map(|step| Key::from_i64(step.output).unwrap())
                .collect();

              if outputs == self.target_sequence {
                #[cfg(feature = "debug")]
                println!(
                  "Valid solution found with {} actual inputs",
                  actual_input_count
                );
                best_solution = Some(solution);
                max_steps = try_steps - 1;
              } else {
                #[cfg(feature = "debug")]
                {
                  println!(
                    "Invalid solution - outputs don't match or wrong input \
                     count"
                  );
                  println!("Generated outputs: {:?}", outputs);
                  println!("Expected outputs: {:?}", self.target_sequence);
                  println!("Actual input count: {}", actual_input_count);
                }
                min_steps = try_steps + 1;
              }
            } else {
              min_steps = try_steps + 1;
            }
          }
          SatResult::Unsat => {
            #[cfg(feature = "debug")]
            println!("No solution with {} steps", try_steps);
            min_steps = try_steps + 1;
          }
          SatResult::Unknown => {
            #[cfg(feature = "debug")]
            println!("Solver returned unknown state");
            min_steps = try_steps + 1;
          }
        },
        Err(e) => return Err(e),
      }
    }

    best_solution.ok_or_else(|| "no valid solution found".to_string())
  }
  pub fn prep(
    &mut self,
    seed_inputs: Option<Vec<Option<Key>>>,
  ) -> Result<(), String> {
    self.initialize_step_states(seed_inputs)?;

    self.add_model_level_constraints();
    self.add_transitional_constraints();
    self.add_target_constraints();

    Ok(())
  }

  fn initialize_step_states(
    &mut self,
    seed_inputs: Option<Vec<Option<Key>>>,
  ) -> Result<(), String> {
    for step in 0..self.max_steps {
      let mut positions = Vec::new();
      let mut input_keys = Vec::new();

      for robot in &self.robots {
        let x = Int::new_const(self.ctx, format!("x_{}_{}", robot.id, step));
        let y = Int::new_const(self.ctx, format!("y_{}_{}", robot.id, step));
        positions.push((x.clone(), y.clone()));

        // Assert initial positions
        if step == 0 {
          let (initial_x, initial_y) = robot.keypad.initial_pos();
          self.solver.assert(
            &x._eq(&Int::from_i64(self.ctx, initial_x))
              .and(&y._eq(&Int::from_i64(self.ctx, initial_y)))
              .tag("initial position"),
          );
        }
        let input_key =
          Int::new_const(self.ctx, format!("input_{}_{}", robot.id, step));
        input_keys.push(input_key.clone());
      }

      if let Some(seed_inputs) = &seed_inputs {
        if step < seed_inputs.len() {
          if let Some(seed_input) = seed_inputs[step] {
            let key_value = seed_input as i64;
            let input_key = &input_keys[0];
            self.solver.assert(
              &input_key
                ._eq(&Int::from_i64(self.ctx, key_value))
                .tag(format!("seed input {step}").as_str()),
            );
          }
        }
      }

      let output = Int::new_const(self.ctx, format!("output_{}", step));

      self
        .step_states
        .push(StepState { positions, input_keys, output });
    }

    Ok(())
  }

  fn add_model_level_constraints(&mut self) {
    // Ensure robots are always at valid positions
    for step in 0..self.step_states.len() {
      for (robot_idx, robot) in self.robots.iter().enumerate() {
        let pos = &self.step_states[step].positions[robot_idx];
        let in_bounds =
          robot.keypad.is_in_bounds((pos.0.clone(), pos.1.clone()));
        self.solver.assert(&in_bounds.tag("in bounds"));
      }
    }

    for step in 0..self.step_states.len() {
      for (robot_idx, _robot) in self.robots.iter().enumerate() {
        let input_key = &self.step_states[step].input_keys[robot_idx];

        // Input must be a movement key, 'A', or -1
        let is_movement = input_key
          .ge(&Int::from_i64(self.ctx, Key::Left as i64))
          .and(&input_key.le(&Int::from_i64(self.ctx, Key::Down as i64)));
        let is_a = input_key._eq(&Int::from_i64(self.ctx, Key::A as i64));
        let is_no_input = input_key._eq(&Int::from_i64(self.ctx, -1));
        self.solver.assert(
          &is_movement
            .or(&is_a)
            .or(&is_no_input)
            .tag("assert valid inputs"),
        );

        // If input to last robot is not A, output must also be -1
        if robot_idx == self.robots.len() - 1 {
          let output = &self.step_states[step].output;
          self.solver.assert(
            &is_a
              .clone()
              .not()
              .implies(&output._eq(&Int::from_i64(self.ctx, -1)))
              .tag("no input implies no output"),
          );
          self
            .solver
            .assert(&output.gt(&Int::from_i64(self.ctx, -1)).iff(&is_a));

          let numeric_pos = self.step_states[step].positions.last().unwrap(); // Numeric robot's position
          let numeric_input = self.step_states[step].input_keys.last().unwrap(); // Numeric robot's input

          // Get the key at the numeric robot's position
          let numeric_key = self
            .robots
            .last()
            .unwrap()
            .keypad
            .get_key(numeric_pos.clone())
            .unwrap();

          // Rule 1: Output equals 7 iff numeric robot is at (0, 0)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::Seven as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::Seven as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals 7 iff at (0,0)"),
          );

          // Rule 2: Output equals 8 iff numeric robot is at (1, 0)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::Eight as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::Eight as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals 8 iff at (1,0)"),
          );

          // Rule 3: Output equals 9 iff numeric robot is at (2, 0)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::Nine as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::Nine as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals 9 iff at (2,0)"),
          );

          // Rule 4: Output equals 4 iff numeric robot is at (0, 1)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::Four as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::Four as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals 4 iff at (0,1)"),
          );

          // Rule 5: Output equals 5 iff numeric robot is at (1, 1)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::Five as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::Five as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals 5 iff at (1,1)"),
          );

          // Rule 6: Output equals 6 iff numeric robot is at (2, 1)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::Six as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::Six as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals 6 iff at (2,1)"),
          );

          // Rule 7: Output equals 1 iff numeric robot is at (0, 2)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::One as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::One as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals 1 iff at (0,2)"),
          );

          // Rule 8: Output equals 2 iff numeric robot is at (1, 2)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::Two as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::Two as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals 2 iff at (1,2)"),
          );

          // Rule 9: Output equals 3 iff numeric robot is at (2, 2)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::Three as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::Three as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals 3 iff at (2,2)"),
          );

          // Rule 10: Output equals 0 iff numeric robot is at (1, 3)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::Zero as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::Zero as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals 0 iff at (1,3)"),
          );

          // Rule 11: Output equals A iff numeric robot is at (2, 3)
          self.solver.assert(
            &output
              ._eq(&Int::from_i64(self.ctx, Key::A as i64))
              .iff(
                &numeric_key
                  ._eq(&Int::from_i64(self.ctx, Key::A as i64))
                  .and(
                    &numeric_input._eq(&Int::from_i64(self.ctx, Key::A as i64)),
                  ),
              )
              .tag("output equals A iff at (2,3)"),
          );
        }
      }
    }
  }

  fn add_position_transitions(&mut self) {
    // Handle transitions between steps (needs length-1 because we need a next state)
    for step in 0..self.step_states.len() - 1 {
      let current_state = &self.step_states[step];
      let next_state = &self.step_states[step + 1];

      for robot_idx in 0..self.robots.len() {
        let robot = &self.robots[robot_idx];
        let current_pos = &current_state.positions[robot_idx];
        let next_pos = &next_state.positions[robot_idx];
        let current_input = &current_state.input_keys[robot_idx];

        // Handle movement based on input
        match robot.keypad {
          KeypadType::Input => {
            // Input robot doesn't move
            self.solver.assert(&next_pos.0._eq(&current_pos.0));
            self.solver.assert(&next_pos.1._eq(&current_pos.1));
          }
          KeypadType::Direction | KeypadType::Numeric => {
            let is_left =
              current_input._eq(&Int::from_i64(self.ctx, Key::Left as i64));
            let is_right =
              current_input._eq(&Int::from_i64(self.ctx, Key::Right as i64));
            let is_up =
              current_input._eq(&Int::from_i64(self.ctx, Key::Up as i64));
            let is_down =
              current_input._eq(&Int::from_i64(self.ctx, Key::Down as i64));

            let new_x = current_pos.0.clone()
              + is_left
                .ite(&Int::from_i64(self.ctx, -1), &Int::from_i64(self.ctx, 0))
              + is_right
                .ite(&Int::from_i64(self.ctx, 1), &Int::from_i64(self.ctx, 0));
            let new_y = current_pos.1.clone()
              + is_up
                .ite(&Int::from_i64(self.ctx, -1), &Int::from_i64(self.ctx, 0))
              + is_down
                .ite(&Int::from_i64(self.ctx, 1), &Int::from_i64(self.ctx, 0));

            self.solver.assert(&next_pos.0._eq(&new_x));
            self.solver.assert(&next_pos.1._eq(&new_y));

            let is_movement = current_input
              .ge(&Int::from_i64(self.ctx, Key::Left as i64))
              .and(
                &current_input.le(&Int::from_i64(self.ctx, Key::Down as i64)),
              );
            self.solver.assert(
              &next_pos
                .0
                ._eq(&current_pos.0)
                .not()
                .or(&next_pos.1._eq(&current_pos.1).not())
                .iff(&is_movement),
            );
          }
        }
      }
    }
  }

  fn add_robot_cascading(&mut self) {
    // Handle cascading between robots (needs full length because it's within each step)
    for step in 0..self.step_states.len() {
      let current_state = &self.step_states[step];

      for robot_idx in 0..self.robots.len() - 1 {
        let robot = &self.robots[robot_idx];
        let current_pos = &current_state.positions[robot_idx];
        let current_input = &current_state.input_keys[robot_idx];
        let next_robot_input = &current_state.input_keys[robot_idx + 1];

        let is_movement = current_input
          .ge(&Int::from_i64(self.ctx, Key::Left as i64))
          .and(&current_input.le(&Int::from_i64(self.ctx, Key::Down as i64)));

        if robot_idx > 0 {
          // If the current input to directional robot is a movement key, the next robot gets -1
          self.solver.assert(
            &is_movement
              .clone()
              .implies(&next_robot_input._eq(&Int::from_i64(self.ctx, -1)))
              .tag("stop cascading movement keys"),
          );

          // Otherwise, cascade the input as usual
          let is_a = current_input._eq(&Int::from_i64(self.ctx, Key::A as i64));
          if let Some(next_key) = robot.keypad.get_key(current_pos.clone()) {
            self.solver.assert(
              &is_a
                .clone()
                .implies(&next_robot_input._eq(&next_key))
                .tag("cascade a"),
            );
          }

          // iff for the above two:
          self.solver.assert(
            &next_robot_input
              ._eq(&Int::from_i64(self.ctx, -1))
              .iff(&is_movement.or(&is_a.not()))
              .tag("default to -1"),
          );
        } else {
          // if the current input is to the top robot, ensure it is given as input to next robot
          self.solver.assert(
            &current_input
              ._eq(&Int::from_i64(self.ctx, -1))
              .not()
              .implies(&next_robot_input._eq(current_input))
              .tag("unconditional cascade"),
          );
          self.solver.assert(
            &next_robot_input
              ._eq(&Int::from_i64(self.ctx, -1))
              .not()
              .iff(&next_robot_input._eq(current_input))
              .tag("unconditional cascade iff"),
          );
        }
      }
    }
  }

  fn add_transitional_constraints(&mut self) {
    self.add_position_transitions();
    self.add_robot_cascading();
  }

  fn add_target_constraints(&mut self) {
    let outputs: Vec<_> = (0..self.step_states.len())
      .map(|step| &self.step_states[step].output)
      .collect();
    let negative_one = Int::from_i64(self.ctx, -1);
    // Create boolean sequence for non-negative outputs
    let is_non_negative: Vec<_> = outputs
      .iter()
      .map(|output| output.gt(&negative_one))
      .collect();
    // Count of non-negative outputs before each position
    let mut count_before = Vec::new();
    let mut running_sum = Int::from_i64(self.ctx, 0);
    count_before.push(running_sum.clone());
    for b in is_non_negative.iter().take(outputs.len() - 1) {
      running_sum +=
        b.ite(&Int::from_i64(self.ctx, 1), &Int::from_i64(self.ctx, 0));
      count_before.push(running_sum.clone());
    }
    // For each target position k, ensure there exists a non-negative output at position i
    // where count_before[i] equals k and output[i] equals target[k]
    for k in 0..self.target_sequence.len() {
      let target_val = Int::from_i64(self.ctx, self.target_sequence[k] as i64);
      // Create a disjunction of all positions that could match this target
      let matches_at_positions: Vec<_> = (0..outputs.len())
        .map(|i| {
          let output = &outputs[i];
          let is_used = &is_non_negative[i];
          let count = &count_before[i];
          // This position matches if:
          // 1. It's non-negative
          // 2. It's the kth non-negative output
          // 3. Its value equals the target
          is_used
            .clone()
            .and(&count._eq(&Int::from_i64(self.ctx, k as i64)))
            .and(&output._eq(&target_val))
        })
        .collect();
      // Assert that at least one position matches this target
      self.solver.assert(&Bool::or(
        self.ctx,
        &matches_at_positions.iter().collect::<Vec<_>>(),
      ));
    }
    // For positions that are used (non-negative), ensure they match their corresponding target
    for i in 0..outputs.len() {
      let output = &outputs[i];
      let is_used = &is_non_negative[i];
      let count = &count_before[i];
      // If this position is used, it must match the target at its count position
      for k in 0..self.target_sequence.len() {
        let is_kth = count._eq(&Int::from_i64(self.ctx, k as i64));
        let target_val =
          Int::from_i64(self.ctx, self.target_sequence[k] as i64);
        self.solver.assert(
          &(is_used
            .clone()
            .and(&is_kth)
            .implies(&output._eq(&target_val))),
        );
      }
      // If not used, must be -1
      self
        .solver
        .assert(&is_used.not().implies(&output._eq(&negative_one)));

      // NEW CONSTRAINT: If the count before this position is >= target_sequence.len(),
      // then this position must be -1
      let target_len =
        Int::from_i64(self.ctx, self.target_sequence.len() as i64);
      self
        .solver
        .assert(&count.ge(&target_len).implies(&output._eq(&negative_one)));
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use z3::{Config, Context};

  #[test]
  fn test_initial_positions() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let robots = vec![
      Robot { keypad: KeypadType::Input, id: 0 },
      Robot { keypad: KeypadType::Direction, id: 1 },
      Robot { keypad: KeypadType::Numeric, id: 2 },
    ];

    let mut solver = KeypadSolver::new(&ctx, robots.clone(), vec![], 1);
    let _ = solver.prep(None);

    // We need to check satisfiability first
    assert_eq!(solver.solver.check(), SatResult::Sat);

    // Then get the model
    let model = solver.solver.get_model().unwrap();

    // Now verify the initial positions
    for (robot_idx, robot) in robots.iter().enumerate() {
      let initial_pos = robot.keypad.initial_pos();
      let (x, y) = &solver.step_states[0].positions[robot_idx];

      // Use the model to evaluate the variables
      let x_val = model.eval(x, true).unwrap().as_i64().unwrap();
      let y_val = model.eval(y, true).unwrap().as_i64().unwrap();

      assert_eq!(
        x_val, initial_pos.0,
        "Robot {} initial x position is incorrect",
        robot_idx
      );
      assert_eq!(
        y_val, initial_pos.1,
        "Robot {} initial y position is incorrect",
        robot_idx
      );
    }
  }

  #[test]
  fn test_a_key_cascading() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // Define the robots
    let robots = vec![
      Robot { keypad: KeypadType::Input, id: 0 },
      Robot { keypad: KeypadType::Direction, id: 1 },
      Robot { keypad: KeypadType::Numeric, id: 2 },
    ];

    println!("=== Test Setup ===");
    println!("Number of robots: {}", robots.len());
    for (i, robot) in robots.iter().enumerate() {
      println!("Robot {}: {:?} keypad at id {}", i, robot.keypad, robot.id);
    }

    // Define the target sequence (just pressing 'A' once)
    let target_sequence = vec![Key::A];
    println!("\nTarget sequence: {:?}", target_sequence);

    // Create the solver with a maximum of 10 steps
    let max_steps = 1;
    println!("\nInitializing solver with max_steps: {}", max_steps);
    let mut solver =
      KeypadSolver::new(&ctx, robots, target_sequence, max_steps);

    // Add constraints
    let _ = solver.prep(Some(vec![Some(Key::A)]));

    // Print all assertions in the solver
    println!("\n=== Current Solver Assertions ===");
    println!("{}", solver.solver);

    // Check satisfiability before solving
    println!("\n=== Checking Solver State ===");
    match solver.solver.check() {
      z3::SatResult::Sat => {
        println!("Constraints are satisfiable");
        if let Some(model) = solver.solver.get_model() {
          println!("\nInitial model state:");
          // Print values for step 0
          if !solver.step_states.is_empty() {
            for (i, state) in solver.step_states.iter().enumerate() {
              println!("\nStep {i} state:");

              // Print positions
              for (i, (x, y)) in state.positions.iter().enumerate() {
                if let (Some(x_val), Some(y_val)) =
                  (model.eval(x, true), model.eval(y, true))
                {
                  println!("Robot {} position: ({:?}, {:?})", i, x_val, y_val);
                }
              }

              // Print input keys
              for (i, input) in state.input_keys.iter().enumerate() {
                if let Some(val) = model.eval(input, true) {
                  println!("Robot {} input: {:?}", i, val);
                }
              }

              // Print outputs
              if let Some(val) = model.eval(&state.output, true) {
                println!("Output: {:?}", val);
              }
            }
          }
        }
      }
      z3::SatResult::Unsat => println!("Constraints are unsatisfiable"),
      z3::SatResult::Unknown => println!("Satisfiability unknown"),
    }

    // Solve the sequence
    println!("\n=== Attempting to Solve ===");
    match solver.solve_detailed() {
      Ok(solution) => {
        println!("Found solution!");
        println!("Solution length: {}", solution.len());

        // Verify solution
        let actual_inputs: Vec<_> = solution
          .iter()
          .filter(|step| step.robot_inputs[0] != -1)
          .map(|step| Key::from_i64(step.robot_inputs[0]).unwrap())
          .collect();

        assert_eq!(
          actual_inputs.len(),
          1,
          "Solution should have exactly 1 step"
        );
        assert_eq!(actual_inputs[0], Key::A, "Solution should press 'A'");
      }
      Err(e) => {
        // Print the final state of all variables in the solver
        println!("\n=== Error State ===");
        println!("Solver error: {}", e);
        println!("Number of step states: {}", solver.step_states.len());

        if let Some(model) = solver.solver.get_model() {
          println!("\nFinal model state:");
          // Try to print the first step's state
          if !solver.step_states.is_empty() {
            let first_state = &solver.step_states[0];
            println!("\nFirst step state:");
            for (i, (x, y)) in first_state.positions.iter().enumerate() {
              if let (Some(x_val), Some(y_val)) =
                (model.eval(x, true), model.eval(y, true))
              {
                println!("Robot {} position: ({:?}, {:?})", i, x_val, y_val);
              }
            }

            // Print variables that might be causing the failure
            println!("\nConstraint variables for first step:");
            for (i, input) in first_state.input_keys.iter().enumerate() {
              if let Some(val) = model.eval(input, true) {
                println!("Robot {} input: {:?}", i, val);
              }
            }
            if let Some(val) = model.eval(&first_state.output, true) {
              println!("Robot output: {:?}", val);
            }
          }
        }

        panic!("Test failed: {}", e);
      }
    }
  }

  #[test]
  fn test_directional_button_cascading() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    println!("\n=== Initial Setup ===");
    let robots = vec![
      Robot { keypad: KeypadType::Input, id: 0 },
      Robot { keypad: KeypadType::Direction, id: 1 },
      Robot { keypad: KeypadType::Numeric, id: 2 },
    ];
    for (i, robot) in robots.iter().enumerate() {
      println!(
        "Robot {}: {:?} starting at {:?}",
        i,
        robot.keypad,
        robot.keypad.initial_pos()
      );
    }

    println!("\n=== Creating Solver ===");
    // We need 2 steps: initial and after Left press
    let max_steps = 2;
    let mut solver = KeypadSolver::new(&ctx, robots.clone(), vec![], max_steps);

    println!("\n=== Adding Constraints ===");
    // Add initial Left press
    let _ = solver.prep(Some(vec![Some(Key::Left)]));

    println!("\n=== Solver State ===");
    println!("{}", solver.solver);

    match solver.solver.check() {
      SatResult::Sat => {
        let model = solver.solver.get_model().unwrap();

        println!("\n=== Model State ===");
        for step in 0..max_steps {
          println!("\nStep {} state:", step);

          // Print all robot positions
          for (i, (x, y)) in
            solver.step_states[step].positions.iter().enumerate()
          {
            let x_val = model.eval(x, true).unwrap().as_i64().unwrap();
            let y_val = model.eval(y, true).unwrap().as_i64().unwrap();
            println!("Robot {} position: ({}, {})", i, x_val, y_val);
          }

          // Print all robot inputs
          for (i, input) in
            solver.step_states[step].input_keys.iter().enumerate()
          {
            let input_val = model.eval(input, true).unwrap().as_i64().unwrap();
            println!("Robot {} input: {}", i, input_val);
          }

          // Print step output
          let output_val = model
            .eval(&solver.step_states[step].output, true)
            .unwrap()
            .as_i64()
            .unwrap();
          println!("Step output: {}", output_val);
        }

        println!("\n=== Running Assertions ===");
        // Now run the original assertions with more context
        for (robot_idx, robot) in robots.iter().enumerate() {
          let pos = &solver.step_states[1].positions[robot_idx];
          let x = model.eval(&pos.0, true).unwrap().as_i64().unwrap();
          let y = model.eval(&pos.1, true).unwrap().as_i64().unwrap();

          println!("Checking Robot {} ({:?}):", robot_idx, robot.keypad);
          println!("  Final position: ({}, {})", x, y);

          match robot.keypad {
            KeypadType::Input => {
              println!("  Expecting: (0, 0)");
              assert_eq!(x, 0, "Input robot should not move from x=0");
              assert_eq!(y, 0, "Input robot should not move from y=0");
            }
            KeypadType::Direction => {
              println!("  Expecting: (1, 0)");
              assert_eq!(
                x, 1,
                "Direction robot should move to x=1 after Left press"
              );
              assert_eq!(y, 0, "Direction robot should maintain y=0");
            }
            KeypadType::Numeric => {
              println!("  Expecting: (2, 3)");
              assert_eq!(x, 2, "Numeric robot should stay at x=2");
              assert_eq!(y, 3, "Numeric robot should stay at y=3");
            }
          }
        }
      }
      SatResult::Unsat => {
        panic!(
          "Constraints are unsatisfiable - this likely indicates a bug in the \
           constraint system"
        );
      }
      SatResult::Unknown => {
        panic!(
          "Z3 solver returned Unknown - might need to adjust solver parameters"
        );
      }
    }
  }

  #[test]
  fn test_layer_movement_relationship() {
    // Set up Z3 context and our robot configuration
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // Define our robot system - each with a specific role
    let robots = vec![
      Robot { keypad: KeypadType::Input, id: 0 }, // Controls input sequence
      Robot { keypad: KeypadType::Direction, id: 1 }, // Translates movement
      Robot { keypad: KeypadType::Numeric, id: 2 }, // Produces final output
    ];

    println!("\n=== Test Setup ===");
    println!("Number of robots: {}", robots.len());
    for (i, robot) in robots.iter().enumerate() {
      println!("Robot {}: {:?} keypad at id {}", i, robot.keypad, robot.id);
    }

    // We need 3 steps to test the sequence
    let max_steps = 3;
    println!("\nInitializing solver with max_steps: {}", max_steps);
    let mut solver = KeypadSolver::new(&ctx, robots.clone(), vec![], max_steps);

    println!("\nSetting up initial constraints with Left, A sequence");
    let _ = solver.prep(Some(vec![Some(Key::Left), Some(Key::A)]));

    // Print all assertions in the solver for debugging
    println!("\n=== Current Solver Assertions ===");
    println!("{}", solver.solver);

    println!("\n=== Checking Solver State ===");
    match solver.solver.check() {
      SatResult::Sat => {
        let model = solver.solver.get_model().unwrap();
        println!("\n=== Model State ===");

        // Print detailed state for each step
        for step in 0..max_steps {
          println!("\nStep {} complete state:", step);

          // Print positions
          println!("Robot positions:");
          for (i, (x, y)) in
            solver.step_states[step].positions.iter().enumerate()
          {
            if let (Some(x_val), Some(y_val)) = (
              model.eval(x, true).and_then(|v| v.as_i64()),
              model.eval(y, true).and_then(|v| v.as_i64()),
            ) {
              println!("  Robot {} at position ({}, {})", i, x_val, y_val);
            } else {
              println!("  Robot {} position unavailable", i);
            }
          }

          // Print inputs
          println!("Robot inputs:");
          for (i, input) in
            solver.step_states[step].input_keys.iter().enumerate()
          {
            if let Some(val) = model.eval(input, true).and_then(|v| v.as_i64())
            {
              println!("  Robot {} input: {}", i, val);
            } else {
              println!("  Robot {} input unavailable", i);
            }
          }

          // Print output
          if let Some(val) = model
            .eval(&solver.step_states[step].output, true)
            .and_then(|v| v.as_i64())
          {
            println!("Output: {}", val);
          } else {
            println!("Output unavailable");
          }
        }

        // Now run our verification functions
        println!("\n=== Running Verifications ===");
        verify_robot_positions(&solver, &model, &robots);
        verify_output_sequence(&solver, &model);
      }
      SatResult::Unsat => {
        println!("\n=== Error: Unsatisfiable Constraints ===");
        println!("Solver state at failure:");
        println!("{}", solver.solver);

        // Try to print the last known state
        println!("\nLast known state:");
        for (i, step) in solver.step_states.iter().enumerate() {
          println!("\nStep {}:", i);
          println!("Number of positions: {}", step.positions.len());
          println!("Number of inputs: {}", step.input_keys.len());
        }

        panic!(
          "Constraints are unsatisfiable - check movement rules and cascading \
           logic"
        );
      }
      SatResult::Unknown => {
        println!("\n=== Error: Unknown Solver State ===");
        println!("Solver state at failure:");
        println!("{}", solver.solver);

        panic!(
          "Z3 solver returned Unknown - might need to adjust solver timeout \
           or memory parameters"
        );
      }
    }
  }

  // Helper function to verify robot positions at each step
  fn verify_robot_positions<'ctx>(
    solver: &KeypadSolver<'ctx>,
    model: &z3::Model<'ctx>,
    robots: &[Robot],
  ) {
    for step in 0..3 {
      println!("\nStep {}:", step);
      for (robot_idx, _) in robots.iter().enumerate() {
        let pos = &solver.step_states[step].positions[robot_idx];
        let x = model.eval(&pos.0, true).unwrap().as_i64().unwrap();
        let y = model.eval(&pos.1, true).unwrap().as_i64().unwrap();

        println!("Robot {} at position ({}, {})", robot_idx, x, y);

        // Verify positions based on the expected sequence
        match (robot_idx, step) {
          // Input robot (robot 0) stays fixed
          (0, _) => {
            assert_eq!(x, 0, "Input robot should remain at x=0");
            assert_eq!(y, 0, "Input robot should remain at y=0");
          }
          // Direction robot (robot 1) moves left after step 0
          (1, 0) => {
            assert_eq!(x, 2, "Direction robot starts at x=2");
            assert_eq!(y, 0, "Direction robot starts at y=0");
          }
          (1, 1..=2) => {
            assert_eq!(
              x, 1,
              "Direction robot should move to x=1 after Left press"
            );
            assert_eq!(y, 0, "Direction robot should maintain y=0");
          }
          // Numeric robot (robot 2) moves up after receiving Up command
          (2, 0..=1) => {
            assert_eq!(x, 2, "Numeric robot maintains x=2 initially");
            assert_eq!(y, 3, "Numeric robot starts at y=3");
          }
          (2, 2) => {
            assert_eq!(x, 2, "Numeric robot maintains x=2");
            assert_eq!(
              y, 2,
              "Numeric robot should move up to y=2 after receiving Up"
            );
          }
          _ => panic!("Unexpected robot/step combination"),
        }
      }
    }
  }

  #[test]
  fn test_out_of_bounds_movement() {
    // Initialize Z3 context
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // Define the robots
    let robots = vec![
      Robot { keypad: KeypadType::Input, id: 0 },
      Robot { keypad: KeypadType::Direction, id: 1 },
      Robot { keypad: KeypadType::Numeric, id: 2 },
    ];

    // Define the target sequence (not relevant for this test)
    let target_sequence = vec![];

    // Create the solver with a maximum of 3 steps
    let mut solver = KeypadSolver::new(&ctx, robots, target_sequence, 3);

    // Seed an input sequence that would cause an out-of-bounds movement
    // For example, moving left twice from the initial position (2,0) would go to (0,0), which is invalid
    let seed_inputs = vec![Some(Key::Left), Some(Key::Left)];

    // Prepare the solver with the seeded inputs
    let _ = solver.prep(Some(seed_inputs));

    // Check if the constraints are satisfiable
    match solver.solver.check() {
      SatResult::Unsat => {
        // The solver correctly identified that the sequence is invalid
        println!(
          "Solver correctly returned unsatisfiable for out-of-bounds movement."
        );
      }
      SatResult::Sat => {
        // The solver incorrectly found a solution for an invalid sequence
        panic!(
          "Solver incorrectly returned satisfiable for out-of-bounds movement."
        );
      }
      SatResult::Unknown => {
        // The solver couldn't determine satisfiability
        panic!("Solver returned unknown for out-of-bounds movement.");
      }
    }
  }

  // Helper function to verify the output sequence
  fn verify_output_sequence<'ctx>(
    solver: &KeypadSolver<'ctx>,
    model: &z3::Model<'ctx>,
  ) {
    for step in 0..3 {
      let output = &solver.step_states[step].output;
      let output_value = model.eval(output, true).unwrap().as_i64().unwrap();
      println!("Step {} output: {}", step, output_value);

      assert_eq!(output_value, -1, "No output during Left press");
    }
  }

  #[test]
  fn test_solver_0a_solve() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let robots = vec![
      Robot { keypad: KeypadType::Input, id: 0 },
      Robot { keypad: KeypadType::Direction, id: 1 },
      Robot { keypad: KeypadType::Numeric, id: 2 },
    ];

    let target_sequence = vec![Key::Zero, Key::A];
    println!("\nTarget sequence: {:?}", target_sequence);

    let mut solver =
      KeypadSolver::new(&ctx, robots, target_sequence.clone(), 12);
    let _ = solver.prep(None);

    match solver.solve_detailed() {
      Ok(solution) => {
        println!("\nStep by step solution:");
        for (step_idx, step) in solution.iter().enumerate() {
          println!("\nStep {}:", step_idx);
          println!("Robot positions: {:?}", step.positions);
          println!("Robot inputs: {:?}", step.robot_inputs);
          println!("Output: {:?}", step.output);
        }

        println!("\nStarting verification...\n");
        let mut dir_x = 2; // Direction robot starts at (2,0)
        let mut dir_y = 0;
        let mut num_x = 2; // Numeric robot starts at (2,3)
        let mut num_y = 3;
        let mut outputs = Vec::new();
        let mut prev_dir_input = None;
        let mut prev_num_input = None;

        for (step_idx, step) in solution.iter().enumerate() {
          println!("\nVerifying step {}", step_idx);

          // Process previous step's movements first
          if let Some(key) = prev_dir_input {
            println!("Processing previous direction input: {:?}", key);
            match key {
              Key::Left => dir_x -= 1,
              Key::Right => dir_x += 1,
              Key::Up => dir_y -= 1,
              Key::Down => dir_y += 1,
              _ => {}
            }
          }

          if let Some(key) = prev_num_input {
            println!("Processing previous numeric input: {:?}", key);
            match key {
              Key::Left => num_x -= 1,
              Key::Right => num_x += 1,
              Key::Up => num_y -= 1,
              Key::Down => num_y += 1,
              _ => {}
            }
          }

          println!(
            "Current positions - Direction: ({},{}), Numeric: ({},{})",
            dir_x, dir_y, num_x, num_y
          );
          println!("Step inputs: {:?}", step.robot_inputs);

          // Verify current positions
          assert_eq!(
            step.positions[1].0, dir_x,
            "Direction robot x position mismatch at step {}",
            step_idx
          );
          assert_eq!(
            step.positions[1].1, dir_y,
            "Direction robot y position mismatch at step {}",
            step_idx
          );
          assert_eq!(
            step.positions[2].0, num_x,
            "Numeric robot x position mismatch at step {}",
            step_idx
          );
          assert_eq!(
            step.positions[2].1, num_y,
            "Numeric robot y position mismatch at step {}",
            step_idx
          );

          // Store this step's inputs for next step's movement
          prev_dir_input = if step.robot_inputs[0] != -1 {
            let key = Key::from_i64(step.robot_inputs[0]).unwrap();
            match key {
              Key::Left | Key::Right | Key::Up | Key::Down => Some(key),
              Key::A => {
                // Handle A press for output generation
                let dir_key = match (dir_x, dir_y) {
                  (1, 0) => Some(Key::Up),
                  (2, 0) => Some(Key::A),
                  (0, 1) => Some(Key::Left),
                  (1, 1) => Some(Key::Down),
                  (2, 1) => Some(Key::Right),
                  _ => None,
                };

                if let Some(Key::A) = dir_key {
                  let output = match (num_x, num_y) {
                    (0, 0) => Some(Key::Seven),
                    (1, 0) => Some(Key::Eight),
                    (2, 0) => Some(Key::Nine),
                    (0, 1) => Some(Key::Four),
                    (1, 1) => Some(Key::Five),
                    (2, 1) => Some(Key::Six),
                    (0, 2) => Some(Key::One),
                    (1, 2) => Some(Key::Two),
                    (2, 2) => Some(Key::Three),
                    (1, 3) => Some(Key::Zero),
                    (2, 3) => Some(Key::A),
                    _ => None,
                  };
                  if let Some(key) = output {
                    println!("Adding output: {:?}", key);
                    outputs.push(key);
                  }
                }
                None
              }
              _ => None,
            }
          } else {
            None
          };

          prev_num_input = if step.robot_inputs[2] != -1 {
            let key = Key::from_i64(step.robot_inputs[2]).unwrap();
            match key {
              Key::Left | Key::Right | Key::Up | Key::Down => Some(key),
              _ => None,
            }
          } else {
            None
          };

          // Validate bounds for next positions
          if let Some(key) = prev_dir_input {
            let next_dir_x = match key {
              Key::Left => dir_x - 1,
              Key::Right => dir_x + 1,
              _ => dir_x,
            };
            let next_dir_y = match key {
              Key::Up => dir_y - 1,
              Key::Down => dir_y + 1,
              _ => dir_y,
            };
            let valid = ((1..=2).contains(&next_dir_x) && next_dir_y == 0)
              || ((0..=2).contains(&next_dir_x) && next_dir_y == 1);
            assert!(
              valid,
              "Invalid direction movement: {:?} would lead to ({},{})",
              key, next_dir_x, next_dir_y
            );
          }

          if let Some(key) = prev_num_input {
            let next_num_x = match key {
              Key::Left => num_x - 1,
              Key::Right => num_x + 1,
              _ => num_x,
            };
            let next_num_y = match key {
              Key::Up => num_y - 1,
              Key::Down => num_y + 1,
              _ => num_y,
            };
            let valid = ((0..=2).contains(&next_num_x)
              && (0..=2).contains(&next_num_y))
              || (next_num_x == 1 && next_num_y == 3)
              || (next_num_x == 2 && next_num_y == 3);
            assert!(
              valid,
              "Invalid numeric movement: {:?} would lead to ({},{})",
              key, next_num_x, next_num_y
            );
          }
        }

        println!("\nFinal outputs: {:?}", outputs);
        println!("Expected outputs: {:?}", target_sequence);

        assert_eq!(
          outputs, target_sequence,
          "Output sequence does not match target"
        );
      }
      Err(e) => panic!("Failed to solve sequence: {}", e),
    }
  }

  #[test]
  fn test_zero_sequence_corrected() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let robots = vec![
      Robot { keypad: KeypadType::Input, id: 0 },
      Robot { keypad: KeypadType::Direction, id: 1 },
      Robot { keypad: KeypadType::Numeric, id: 2 },
    ];

    // Target: Zero
    let target_sequence = vec![Key::Zero];
    println!("\nTarget: Zero at position (1,3) on numeric keypad");

    // Need enough steps for:
    // 1. Move direction robot left
    // 2. Press A to send Left to numeric robot
    // 3. Press A to output Zero when numeric robot is at (1,3)
    let mut solver =
      KeypadSolver::new(&ctx, robots.clone(), target_sequence, 8);

    println!("\nPreparing solver...");
    let _ = solver.prep(Some(vec![
      Some(Key::Down),  // (2,0) -> (2,1)
      Some(Key::Left),  // (2,1) -> (1,1)
      Some(Key::Left),  // (1,1) -> (0,1)
      Some(Key::A),     // Send Left to numeric
      Some(Key::Right), // (0,1) -> (1,1)
      Some(Key::Right), // (1,1) -> (2,1)
      Some(Key::Up),    // (2,1) -> (2,0)
      Some(Key::A),     // Get Zero output
    ]));

    match solver.solver.check() {
      SatResult::Sat => {
        let model = solver.solver.get_model().unwrap();

        // Print full state sequence
        for step in 0..8 {
          println!("\nStep {}:", step);

          // Print positions
          println!("Positions:");
          for (i, (x, y)) in
            solver.step_states[step].positions.iter().enumerate()
          {
            let x_val = model.eval(x, true).unwrap().as_i64().unwrap();
            let y_val = model.eval(y, true).unwrap().as_i64().unwrap();
            println!("  Robot {} at ({}, {})", i, x_val, y_val);
          }

          // Print input cascade
          println!("Input cascade:");
          for (i, input) in
            solver.step_states[step].input_keys.iter().enumerate()
          {
            let val = model.eval(input, true).unwrap().as_i64().unwrap();
            println!(
              "  Robot {} receives: {} ({:?})",
              i,
              val,
              if val == -1 { None } else { Key::from_i64(val) }
            );
          }

          // Print output
          let output = model
            .eval(&solver.step_states[step].output, true)
            .unwrap()
            .as_i64()
            .unwrap();
          println!(
            "Output: {} ({:?})",
            output,
            if output == -1 {
              None
            } else {
              Key::from_i64(output)
            }
          );
        }
      }
      SatResult::Unsat => {
        println!("Constraints unsatisfiable:");
        println!("{}", solver.solver);
        panic!("No solution exists");
      }
      SatResult::Unknown => panic!("Solver returned unknown state"),
    }
  }

  #[test]
  fn test_solver_029a_solve() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let robots = vec![
      Robot { keypad: KeypadType::Input, id: 0 },
      Robot { keypad: KeypadType::Direction, id: 1 },
      Robot { keypad: KeypadType::Direction, id: 2 },
      Robot { keypad: KeypadType::Numeric, id: 3 },
    ];

    let target_sequence = vec![Key::Zero, Key::Two, Key::Nine, Key::A];
    println!("\nTarget sequence: {:?}", target_sequence);

    let mut solver =
      KeypadSolver::new(&ctx, robots.clone(), target_sequence.clone(), 68);
    let _ = solver.prep(None);

    match solver.solve_detailed() {
      Ok(solution) => {
        // Extract all non-negative outputs
        let outputs: Vec<Key> = solution
          .iter()
          .filter(|step| step.output != -1)
          .map(|step| Key::from_i64(step.output).unwrap())
          .collect();

        println!("\nStep by step solution:");
        for (step_idx, step) in solution.iter().enumerate() {
          if step.robot_inputs.iter().any(|&input| input != -1)
            || step.output != -1
          {
            println!("\nStep {}:", step_idx);
            println!("Robot positions: {:?}", step.positions);
            println!("Robot inputs: {:?}", step.robot_inputs);
            println!("Output: {:?}", step.output);
          }
        }

        println!("\nFinal outputs: {:?}", outputs);
        println!("Expected outputs: {:?}", target_sequence);

        assert_eq!(
          outputs, target_sequence,
          "Output sequence does not match target"
        );

        // Verify the length requirement
        let action_count =
          solution.iter().filter(|s| s.robot_inputs[0] != -1).count();

        assert_eq!(
          action_count, 68,
          "Expected 68 movement inputs but got {}",
          action_count
        );
      }
      Err(e) => panic!("Failed to solve sequence: {}", e),
    }
  }
}
