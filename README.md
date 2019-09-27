# 🔢 solve_rs 🔭

[![Build Status](https://travis-ci.org/mshieldsdunn/solve_rs.svg?branch=master)](https://travis-ci.org/mshieldsdunn/solve_rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview 🔎

**solve_rs** is a Rust script designed to solve Sudoku puzzles. At present, it
employs a brute-force based approach in which it tries each number combination
in order. The intent is for future iterations of the script to try to benefit
from ML/AI so that it might take a more "intelligent", and thus more efficient,
approach.

## TODOs and Ideas for Improvement 💡

- Script should be able to source external puzzle data

## An Important Note 📌

Although this solution is not particularly unique, **I must give credit to
[this Medium post](https://medium.com/@george.seif94/solving-sudoku-using-a-simple-search-algorithm-3ac44857fee8)**, in which the
author chose to implement the solution in C++. The article provides an
excellent overview of the approach and an even better explanation of the
details of the algorithm. After porting the C++ code to Python as a learning
exercise, this project represents an attempt to further optimize and build upon
those earlier efforts.
