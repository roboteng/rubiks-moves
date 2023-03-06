# rubiks-moves

A crate for manipulating algorithms for the Rubik's cube, and speed-cubing

## Examples

```rust
use rubiks_moves::moves::Algorithm;

let sexy = Algorithm::from("R U R' U'").unwrap();

assert_eq!(sexy.order(), 6);
```

```rust
use rubiks_moves::moves::Algorithm;

let scramble = Algorithm::from("R' U' F D2 L2 F R2 U2 R2 B D2 L B2 D' B2 L' R' B D2 B U2 L U2 R' U' F").unwrap();
let solution = Algorithm::from("D2 F' D2 U2 F' L2 D R2 D B2 F L2 R' F' D U'").unwrap();

assert!(solution.solves(&scramble));
```

License: MIT
