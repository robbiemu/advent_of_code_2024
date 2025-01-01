## Keys
- Keys represent the possible inputs and outputs in the system
- A key has an associated position based on its keypad type
- For numeric keypad: 
  ```
  7,8,9
  4,5,6
  1,2,3
    0,A
  ```
- For directional keypad: 
  ```
   ^,A 
  <,v,>
  ```

## Keypad
- Defines the type and layout of keys
- Contains the initial position (always A) for any robot using this keypad
- Determines valid positions and movement rules
- Position rules don't apply to top-level input robot

## Robot
- Represents an entity at a specific keypad
- Properties are just the keypad type it uses
- The top-level robot (representing "You") doesn't move and directly receives inputs
- All other robots must follow position rules and execute actions based on position
- All dynamic state (position, inputs, outputs) belongs in StepState

## StepState
- Captures the complete state of the system at one step:
  - Position of each robot (except top-level which doesn't move)
  - Input received by each robot (-1 for no input)
  - Output produced by each robot (-1 for no output)

## StepState Constraints:
- Top level robot receives direct input without position constraints
- All other robots must:
  - Be at valid positions on their keypad
  - Move according to directional inputs
  - Press the key at their current position when receiving 'A'
- Bottom level robot must be at numeric keypad
- Non-negative outputs must be from numeric keypad
- Non-negative outputs must match target sequence order
- Position changes take effect in next state
- All cascading presses from one input happen in same state

## Solver
- Contains only the configuration needed to validate solutions:
  - Collection of robots and their keypads
  - Target sequence to achieve
- No transient state - all dynamic state belongs in StepState
