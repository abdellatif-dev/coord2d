# coord2d
rust lib for coordinate in 2d system

## example
```rust
use coord2d::*;

fn main() {
    let coord1: coords::Coord = coords::new(3, 4);
    let coord2: coords::Coord = coords::new(5, 13);
    let sum: coords::Coord = coord1 + coord2;

    println!("{:?}", sum);

    let line : vectors::Vector = vectors::new(coord1, coord2);

    println!("{:?}", line);
}

```
# TO DO
- [X] basic math
- [ ] add fn give lenghts, modules of vector
- [ ] add vectors propeties