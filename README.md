# Rust Undefined Behavior Example

This repository demonstrates a common source of undefined behavior in Rust: accessing memory after the owning object has been dropped.  The `bug.rs` file contains code that exhibits this behavior. The `bugSolution.rs` file offers a corrected version.