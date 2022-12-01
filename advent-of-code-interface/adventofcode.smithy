// adventofcode.smithy
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.example.interfaces.adventofcode", crate: "wasmcloud_advent_of_code_interface" } ]

namespace org.example.interfaces.adventofcode

use org.wasmcloud.model#wasmbus

/// Description of AdventOfCode service
@wasmbus( actorReceive: true )
service AdventOfCode {
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