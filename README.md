# Bonus_13522097
Travelling Salesman Problem Solver Using Rust

## Table of Contents

- [General Info](#general-information)
- [Requirements](#requirements)
- [Setup](#setup)
- [Usage](#usage)
- [Reference](#reference)
- [Author](#author)

## General Information

The travelling salesman problem, also known as the travelling salesperson problem (TSP), asks the following question: "Given a list of cities and the distances between each pair of cities, what is the shortest possible route that visits each city exactly once and returns to the origin city?" It is an NP-hard problem in combinatorial optimization, important in theoretical computer science and operations research. 

TSP can be modeled as an undirected weighted graph, such that cities are the graph's vertices, paths are the graph's edges, and a path's distance is the edge's weight. It is a minimization problem starting and finishing at a specified vertex after having visited each other vertex exactly once. Often, the model is a complete graph (i.e., each pair of vertices is connected by an edge). If no path exists between two cities, then adding a sufficiently long edge will complete the graph without affecting the optimal tour. 
  
## Requirements

- Rustc 1.78.0
- Visual Studio Code

## Setup

First of all, you'll first need to make sure that you have installed [Rust](https://rustup.rs/) and [WSL](https://learn.microsoft.com/en-us/windows/wsl/install). 

![rust-extension](https://i.ibb.co/RhhF7Xz/rustextension.jpg)

Then, install the Rust extension above for setting up in VS Code

You can find more info on the VSC documentation on [rust](https://code.visualstudio.com/docs/languages/rust).

## Usage
1. Run `wsl` in terminal to activate Windows Subsystem for Linux (WSL)
1. Navigate to the directory where the project will be stored
2. Run `git clone https://github.com/HenryofSkalitz1202/TSP.git` in terminal
4. Run `cargo build`
5. Run `cargo run`
6. Input the path to the txt file

Make sure that the txt file is a valid adjacency matrix of `n x n` with distance to itself as 0 and distance to unadjacent nodes set to 9999 (unreachable/infinite length)

Example:
```
0 12 10 9999 9999 9999 12
12 0 8 12 9999 9999 9999
10 8 0 11 3 9999 9
9999 12 11 0 11 10 9999
9999 9999 3 11 0 6 7
9999 9999 9999 10 6 0 9
12 9999 9 9999 7 9 0
```

# Reference
The project is inspired from the following [tutorial](https://www.tutorialspoint.com/data_structures_algorithms/travelling_salesman_problem_dynamic_programming.htm)

# Author
|          Name                | NIM |
|--------------------------------|------------|
| Ellijah Darrellshane Suryanegara      | 13522097  |
