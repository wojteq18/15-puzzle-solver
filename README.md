# 15 Puzzle Solver in Rust 🧩

This is a Rust implementation of a solver for the classic **15 Puzzle** (4x4 sliding puzzle). It uses the **A\* search algorithm** with the **Manhattan distance** heuristic to find the shortest sequence of moves that leads to the solved configuration.

## Features

- Represents the board as a 4x4 array of integers
- Allows random shuffling of the board
- Finds a solution path using an A\*-like search
- Prints each state along the path from the initial configuration to the solution
- Uses `HashMap` to avoid revisiting states with higher or equal cost
- Priority queue ensures optimal path is found based on estimated cost (A*)

## Usage

Clone the repository and run the project using Cargo:

```bash
cargo run


This project was created for educational purposes as part of a university assignment at Wrocław University of Science and Technology for the course _Introduction to Artificial Intelligence_.**  
**Final grade: 5.5**

