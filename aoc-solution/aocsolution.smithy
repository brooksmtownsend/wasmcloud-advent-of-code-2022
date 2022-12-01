// aocsolution.smithy
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.example.interfaces.aocsolution", crate: "aoc_solution" } ]

namespace org.example.interfaces.aocsolution

use org.wasmcloud.model#wasmbus

/// Description of Aocsolution service
@wasmbus( actorReceive: true )
service Aocsolution {
  version: "0.1",
  operations: [ PartOne, PartTwo]
}

/// Used to retrive the solution for part one of an AdventOfCode solution
operation PartOne {
  output: String
}

/// Used to retrive the solution for part two of an AdventOfCode solution
operation PartTwo {
  output: String
}