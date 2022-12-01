# Advent of Code 2022 Solution Actor

This wasmCloud actor implements a solution for the Advent of Code challenge, using a simple interface for retrieving a solution for part 1 or part 2.

You can launch and invoke this actor by:

1. Building the actor with `wash build`
1. Running `wash up` and using the wasmCloud dashboard at [http://localhost:4000](http://localhost:4000) to start this actor **From File**
1. Using `wash call <public_key> Aocsolution.PartOne '{}'` to call the part_one solution function
