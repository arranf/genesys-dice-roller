# genesys-dice-roller

A simple Rust library for taking a Genesys dice string as input and calculating a result.

## Usage

```rust
use dice_roller::dice::{Dice, RollType};
use std::str::FromStr;

let dice = Dice::from_str("2p2g1y")?;
// Roll dice uses thread RNG
let result = dice.roll_dice();
```

## Example input

```
2p2g1y
```
