# Rosalind in Rust
Playing around with `Rust` using the `bioinformatics` code challenges form https://rosalind.info.

# Structure
- Solutions can be found in modules starting with `_`, eg. `_01_rna.rs`.
- IO workflows used by the solutions are in the `workflows` module.
- `main.rs` runs the selected problem.

# Run
1. Select a problem on Rosalind and reference the related module in `main.rs`.
2. Get new test data for the problem via `Download datasheet` option in Rosalind and save it as `input` in the project root.
3. ```cargo run``` in the project root for a quick test.
