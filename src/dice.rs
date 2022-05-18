use crate::{dice_result::RollResult, die_face::{get_die_face_for_dice, DieFace}};

use genesys_dice_command_parser::{
    dice_roll::DiceRoll as CommandDiceRoll,
    dice::Dice as DiceType
};
use rand::Rng;

/// Represents a set of homogenous dice. E.G. 3 Boost
#[derive(PartialEq, Debug)]
pub struct Dice {
    /// The number of dice in the set of homegenous dice.
    pub number_of_dice_to_roll: u32,
    /// How many sides each dice in the set has.
    pub sides: u32,
    /// The type of Dice
    pub variant: DiceType
}

impl Dice {
    pub(crate) fn from_parsed_dice_roll(parsed_roll: &CommandDiceRoll) -> Self {
        Dice::new(parsed_roll.number_of_dice_to_roll, parsed_roll.die.clone())
    }

    /// Constructs a new dice
    /// # Examples
    /// ```
    /// use dnd_dice_roller::dice::{Dice, RollType, Operation};
    /// // A single d20 with a plus five modifier and advantage
    /// let dice = Dice::new(1, 20, Some(5), RollType::Advantage, Operation::Addition);
    /// ```
    #[must_use]
    pub fn new(
        number_of_dice: u32,
        variant: DiceType
    ) -> Self {
        let sides=  match variant {
                DiceType::Boost | DiceType::Setback => 6,
                DiceType::Ability | DiceType::Difficulty => 8,
                DiceType::Proficiency | DiceType::Challenge | DiceType::Force => 12,
            };
        Dice {
            number_of_dice_to_roll: number_of_dice,
            sides,
            variant
        }
    }

    /// Rolls a dice and produces a `RollResult`. Using underlying OS RNG for the dice roll.
    ///
    /// # Examples
    /// ```
    /// use dnd_dice_roller::dice::{Dice, RollType, Operation};
    /// # use dnd_dice_roller::error::DiceError;
    ///
    /// let dice = Dice::new(1, 10, None, RollType::Regular, Operation::Addition);
    /// let result = dice.roll_dice();
    /// # Ok::<(), DiceError>(())
    /// ```
    #[must_use]
    pub fn roll_dice(&self) -> RollResult {
        let mut rng = rand::thread_rng();
        self.roll_dice_from_rng(&mut rng)
    }

    /// Rolls a dice and produces a `RollResult`. Uses a source of RNG passed in. Useful for testing.
    ///
    /// # Examples
    /// ```
    /// use rand::SeedableRng;
    /// use dnd_dice_roller::dice::{Dice, RollType, Operation};
    ///
    /// let rng = rand_pcg::Pcg64Mcg::seed_from_u64(42);
    /// let dice = Dice::new(1, 6, None, RollType::Regular, Operation::Addition);
    /// let result = dice.roll_dice_from_rng(rng);
    /// assert_eq!(result.result, 2);
    /// ```
    #[allow(clippy::cast_possible_wrap)]
    pub fn roll_dice_from_rng<R: Rng + Sized>(&self, mut rng: R) -> RollResult {
        let current_roll_set_size = self.number_of_dice_to_roll as usize;
        let mut results: Vec<DieFace> = Vec::with_capacity(current_roll_set_size);
        for _ in 0..self.number_of_dice_to_roll {
            results.push(get_die_face_for_dice(rng.gen_range(0..self.sides) as usize, &self.variant));
        }
        RollResult::new(self.variant, results)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use rand::SeedableRng;

    const SEED: u64 = 42;

    #[test]
    fn produces_predictable_results_boost_equals_blank() {
        let rng = rand_pcg::Pcg64Mcg::seed_from_u64(SEED);
        let dice = Dice::new(1, DiceType::Boost);
        let result = dice.roll_dice_from_rng(rng);
        let _expected = 1;
        assert_eq!(result.raw_results, [DieFace::Blank]);
        assert_eq!(result.success_fail_net, 0);
        assert_eq!(result.advantage_threat_net, 0);
    }

    #[test]
    fn produces_predictable_results_challenge_equals_double_success() {
        let rng = rand_pcg::Pcg64Mcg::seed_from_u64(SEED);
        let dice = Dice::new(1, DiceType::Challenge);
        let result = dice.roll_dice_from_rng(rng);
        let _expected = 3;
        assert_eq!(result.raw_results, [DieFace::DoubleFailure]);
        assert_eq!(result.success_fail_net, -2);
        assert_eq!(result.advantage_threat_net, 0);
    }


    // TODO: More of this
    #[test]
    fn roll_dice_within_range_check_occurences() {
        let dice = Dice::new(1, DiceType::Difficulty);
        
        let mut dice_counts: HashMap<DieFace, u32> = HashMap::new();
        
        let number_of_rolls = 100_000;
        for _ in 0..number_of_rolls {
            let roll_result = dice.roll_dice();
            let face = dice_counts.entry(roll_result.raw_results[0]).or_insert(0);
             *face += 1;
        }

        let threat = *dice_counts.get(&DieFace::Threat).unwrap();
        assert!(threat < 41_250 && threat > 33750)
    }
}
