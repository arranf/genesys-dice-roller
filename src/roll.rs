use std::str::FromStr;

use genesys_dice_command_parser::parse_line;
use rand::Rng;

use crate::{dice::Dice, dice_result::DiceSetResults, dice_set::DiceSet, error::DiceError};
/// Represents a set of non-homogenous dice, potentially grouped into multiple separate results - each grouping being a `DiceSet`.
///  e.g. Rolling a ability + difficulty would be a `Roll` of a single `DiceSet`.
///  e.g. Rolling a 2p2g1y, 2p2g1y, 2p2g1y for three separate results (e.g. three separate skill checks) would be a single `Roll` of three `DiceSet`.
#[derive(Debug)]
pub struct Roll {
    dice_sets: Vec<DiceSet>,
}

impl Roll {
    /// Creates a new `Roll`
    pub fn new(dice_sets: Vec<DiceSet>) -> Self {
        Self { dice_sets }
    }

    /// Rolls one more sets of dice and produces a `Vec<DiceSetResults>`. Using underlying OS RNG for the dice roll.
    ///
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use dnd_dice_roller::roll::Roll;
    /// # use dnd_dice_roller::error::DiceError;
    ///
    /// let roll = Roll::from_str("2p2g1y")?;
    /// let result = roll.roll();
    /// # Ok::<(), DiceError>(())
    /// ```
    #[must_use]
    pub fn roll(&self) -> Vec<DiceSetResults> {
        let mut rng = rand::thread_rng();
        self.roll_from_rng(&mut rng)
    }

    /// Rolls one more sets of and produces `Vec<DiceSetResults>`. Uses a source of RNG passed in. Useful for testing.
    ///
    /// # Example
    /// ```
    /// use rand::SeedableRng;
    /// use std::str::FromStr;
    /// use dnd_dice_roller::roll::Roll;
    /// # use dnd_dice_roller::error::DiceError;
    ///
    /// let rng = rand_pcg::Pcg64Mcg::seed_from_u64(42);
    /// let roll = Roll::from_str("3d6 + 1")?;
    /// let result = roll.roll_from_rng(rng);
    /// # Ok::<(), DiceError>(())
    /// ```
    pub fn roll_from_rng<R: Rng + Sized>(&self, mut rng: R) -> Vec<DiceSetResults> {
        self.dice_sets
            .iter()
            .map(|d| d.roll_dice_set_from_rng(&mut rng))
            .collect()
    }
}

impl FromStr for Roll {
    type Err = DiceError;
    /// Creates a `Roll` from an input string.
    /// Useful for taking a blind dice input string e.g. `d100, d100, 3d6+2` and producing results.
    ///
    /// # Examples
    /// ```
    /// use dnd_dice_roller::roll::Roll;
    /// use std::str::FromStr;
    /// # use dnd_dice_roller::error::DiceError;
    ///
    /// let roll = Roll::from_str("2p2g1y").unwrap();
    ///
    /// # Ok::<(), DiceError>(())
    /// ```
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use dnd_dice_roller::roll::Roll;
    /// # use dnd_dice_roller::error::DiceError;
    ///
    /// let roll = Roll::from_str("2p2g1y, 2p2g1y")?;
    ///
    /// # Ok::<(), DiceError>(())
    /// ```
    /// # Errors
    /// Errors can occur if the dice input string is in the wrong format `DiceError::ParseError`.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let roll = parse_line(&input)?
            .iter()
            .map(|dice| {
                DiceSet::new(
                    dice.iter()
                        .map(|d| Dice::from_parsed_dice_roll(d))
                        .collect(),
                )
            })
            .collect();
        Ok(Self::new(roll))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{dice_result::RollResult, die_face::DieFace};
    use rand::SeedableRng;

    const SEED: u64 = 42;

    // #[test]
    // fn produces_predictable_results_one_d6_parsed_equals_two() {
    //     let rng = rand_pcg::Pcg64Mcg::seed_from_u64(SEED);
    //     let dice = Roll::from_str("2p2g1y").expect("No error parsing dice");
    //     let result = dice.roll_from_rng(rng);
    //     let expected = vec![DiceSetResults::new(
    //         vec![RollResult::new(vec![DieFace::])],
    //         2,
    //     )];
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn produces_predictable_results_one_d6_parsed_with_advantage_equals_three() {
    //     let rng = rand_pcg::Pcg64Mcg::seed_from_u64(SEED);
    //     let dice = Roll::from_str("1d6 a").expect("No error parsing dice");
    //     let result = dice.roll_from_rng(rng);
    //     let expected = vec![DiceSetResults::new(
    //         vec![RollResult::new(vec![2], Some(vec![6]), 6)],
    //         6,
    //     )];
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn produces_predictable_results_one_d6_parsed_with_disadvantage_equals_two() {
    //     let rng = rand_pcg::Pcg64Mcg::seed_from_u64(SEED);
    //     let dice = Roll::from_str("1d6 d").expect("No error parsing dice");
    //     let result = dice.roll_from_rng(rng);
    //     let expected = vec![DiceSetResults::new(
    //         vec![RollResult::new(vec![2], Some(vec![6]), 2)],
    //         2,
    //     )];
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn produces_predictable_results_three_d6_plus_two_parsed() {
    //     let rng = rand_pcg::Pcg64Mcg::seed_from_u64(SEED);
    //     let dice = Roll::from_str("3d6+2").expect("No error parsing dice");
    //     let result = dice.roll_from_rng(rng);
    //     let expected = vec![DiceSetResults::new(
    //         vec![RollResult::new(vec![2, 6, 5], None, 15)],
    //         15,
    //     )];
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn produces_predictable_results_dice_addition() {
    //     let rng = rand_pcg::Pcg64Mcg::seed_from_u64(SEED);
    //     let dice = Roll::from_str("2d6+2 + d4").expect("No error parsing dice");
    //     let result = dice.roll_from_rng(rng);
    //     let expected = vec![DiceSetResults::new(
    //         vec![
    //             RollResult::new(vec![2, 6], None, 10),
    //             RollResult::new(vec![4], None, 4),
    //         ],
    //         14,
    //     )];
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn produces_predictable_results_dice_subtraction() {
    //     let rng = rand_pcg::Pcg64Mcg::seed_from_u64(SEED);
    //     let dice = Roll::from_str("2d6+2 - d4").expect("No error parsing dice");
    //     let result = dice.roll_from_rng(rng);
    //     let expected = vec![DiceSetResults::new(
    //         vec![
    //             RollResult::new(vec![2, 6], None, 10),
    //             RollResult::new(vec![4], None, 4),
    //         ],
    //         6,
    //     )];
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn produces_predictable_results_dice_combined() {
    //     let rng = rand_pcg::Pcg64Mcg::seed_from_u64(SEED);
    //     let dice = Roll::from_str("2d6+2 + d10+2 - 2d4-1").expect("No error parsing dice");
    //     let result = dice.roll_from_rng(rng);
    //     let expected = vec![DiceSetResults::new(
    //         vec![
    //             RollResult::new(vec![2, 6], None, 10),
    //             RollResult::new(vec![2], None, 4),
    //             RollResult::new(vec![3, 3], None, 5),
    //         ],
    //         9,
    //     )];
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn produces_predictable_results_separate_dice() {
    //     let rng = rand_pcg::Pcg64Mcg::seed_from_u64(SEED);
    //     let dice = Roll::from_str("2d6, d10 , 2d4").expect("No error parsing dice");
    //     let result = dice.roll_from_rng(rng);
    //     let expected = vec![
    //         DiceSetResults::new(vec![RollResult::new(vec![2, 6], None, 8)], 8),
    //         DiceSetResults::new(vec![RollResult::new(vec![2], None, 2)], 2),
    //         DiceSetResults::new(vec![RollResult::new(vec![3, 3], None, 6)], 6),
    //     ];
    //     assert_eq!(result, expected);
    // }
}
