advent_of_code_2024
---
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
Just my aoc repo this year.
![logo](assets/aoc24.jpg)

### highlights:

- [day-2](day-2/) when brute force is enough :D
- [day-3](day-3/) [nom](https://github.com/rust-bakery/nom) is probably heavy for this regex problem. First day I actually used [mry](https://github.com/ryo33/mry) in testing.
- [day-5](day-5/) first day where a DAG made sense.
- [day-6](day-6/) [game_grid](https://docs.rs/game-grid/latest/game_grid/) , which looks useful for bevy, I could have written a visualization, made the setup a breeze 😀
- [day-13](/day-13) [z3](https://github.com/Z3Prover/z3) to the rescue! 🚀
- [day-14](/day-14/) A fun [bevy](https://bevyengine.org) day!
![day-14 screenshot](assets/Dia14.jpg)
- [day-16](/day-16) a natural place to use [pathfinding](https://github.com/evenfurther/pathfinding) crate! This one led to my first rust crate [pr](https://github.com/oilandrust/game-grid/pull/1) (to game-grid) from AoC this year.
- [day-18](/day-18) Since the problem didnt fully lend itself naturally to game_grid, I just used [glam](https://github.com/bitshifter/glam-rs) for coordinates.
- [day 19](/day-19) [Trie](https://github.com/laysakura/trie-rs) as I might, I could not avoid using [cached](https://github.com/jaemk/cached).
- [day 23](/day-23) Finally found a strong case for [petgraph](https://github.com/petgraph/petgraph)