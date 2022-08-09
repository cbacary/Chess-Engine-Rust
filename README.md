# Chess-Engine-Rust

A chess engine made in Rust.

# What it implements

It currently implements a Negamax search with alpha-beta pruning. It uses a transposition table with iterative deepening to increase depth ability.

# How is Move Generation done?

Move generation is done through a 3rd party library `chess`. This library produces moves incredibly quickly.


# How to use

Currently I implement a new struct I made, `MoveIterator` that requires the modifcation of the `chess` library. Becase of this you will have to clone my fork of `chess`

So make sure you include `--recurse-submodule` into the clone, otherwise you will have compiler errors.

`git clone --recurse-submodule https://github.com/cbacary/Chess-Engine-Rust.git`
