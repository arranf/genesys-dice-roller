use crate::{die_face::DieFace};

use genesys_dice_command_parser::{
    dice::Dice as DiceType
};

/// Represents the result of rolling `DiceSet`.
#[derive(PartialEq, Debug)]
pub struct DiceSetResults {
    /// The actual results of the dice that were cast
    pub dice_results: Vec<RollResult>,
    pub success_fail_net: i32,
    pub advantage_threat_net: i32,
    pub triumph_count: u32,
    pub despair_count: u32,
    pub dark_pips: u32,
    pub light_pips: u32,
    pub dice_type: DiceType
}

impl DiceSetResults {
    pub(crate) fn new(dice_results: Vec<RollResult>, dice_type: DiceType) -> Self {
        DiceSetResults {
            dice_type,
            dice_results: dice_results.clone(),
            success_fail_net: dice_results.iter().map(|r| r.success_fail_net).sum(),
            advantage_threat_net: dice_results.iter().map(|r| r.advantage_threat_net).sum(),
            triumph_count: dice_results.iter().map(|r| r.triumph_count).sum(),
            despair_count: dice_results.iter().map(|r| r.despair_count).sum(),
            dark_pips: dice_results.iter().map(|r| r.dark_pips).sum(),
            light_pips: dice_results.iter().map(|r| r.light_pips).sum(),
        }
    }
}

/// Represents the result of rolling `Dice`, a homogenous set of Dice.
#[derive(PartialEq, Debug, Clone)]
pub struct RollResult {
    /// Each die's raw result
    pub raw_results: Vec<DieFace>,
    pub success_fail_net: i32,
    pub advantage_threat_net: i32,
    pub triumph_count: u32,
    pub despair_count: u32,
    pub dark_pips: u32,
    pub light_pips: u32
}

impl RollResult {
    pub(crate) fn new(raw_results: Vec<DieFace>) -> Self {
        let mut success = 0;
        let mut fail = 0;
        let mut threat = 0;
        let mut advantage = 0;
        let mut despair = 0;
        let mut triumph = 0;
        let mut dark_pips = 0;
        let mut light_pips = 0;

        for result in raw_results.iter() {
            match result {
                DieFace::Blank => {},
                DieFace::Threat => {threat += 1}
                DieFace::Advantage => {advantage += 1},
                DieFace::Success => {success += 1},
                DieFace::Failure => {fail += 1},
                DieFace::Triumph => {triumph += 1},
                DieFace::Despair => {despair += 1},
                DieFace::SuccessAndAdvantage => {success += 1; advantage +=1;},
                DieFace::FailureAndThreat => {fail += 1; threat +=1},
                DieFace::DoubleAdvantage => {advantage += 2},
                DieFace::DoubleSuccess => {success += 2},
                DieFace::DoubleFailure => {fail += 2},
                DieFace::DoubleThreat => {threat += 2},
                DieFace::SingleBlackPip => {dark_pips += 1},
                DieFace::SingleWhitePip => {light_pips += 1},
                DieFace::DoubleBlackPip => {dark_pips += 2},
                DieFace::DoubleWhitePip => {light_pips += 2},
            }
        }

        let success_fail_net = success - fail;
        let advantage_threat_net = advantage - threat;

        RollResult {
            raw_results,
            success_fail_net,
            advantage_threat_net,
            triumph_count: triumph,
            despair_count: despair,
            dark_pips,
            light_pips
        }
    }
}


// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn format_raw_result_with_only_one_roll() {
//         let raw_result = RollResult::new(vec![1, 2, 3, 4], None, 7);
//         assert_eq!("[1, 2, 3, 4]", format!("{}", raw_result));
//     }

//     #[test]
//     fn format_raw_result_with_two_rolls() {
//         let raw_result = RollResult::new(vec![4, 2, 1, 3], Some(vec![5, 2, 3, 4]), 14);
//         assert_eq!("[[4, 2, 1, 3], [5, 2, 3, 4]]", format!("{}", raw_result));
//     }
// }
