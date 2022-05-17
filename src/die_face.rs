use std::fmt::Display;

use genesys_dice_command_parser::{
    dice::Dice as DiceType
};

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
pub enum DieFace {
    Blank,
    Threat,
    Advantage,
    Success,
    Failure,
    Triumph,
    Despair,
    SuccessAndAdvantage,
    FailureAndThreat,
    DoubleAdvantage,
    DoubleSuccess,
    DoubleFailure,
    DoubleThreat,
    SingleBlackPip,
    SingleWhitePip,
    DoubleBlackPip,
    DoubleWhitePip
}

pub const fn get_die_face_for_dice(face_index: usize, dice_type: &DiceType) -> DieFace {
    match dice_type {
        DiceType::Boost => [DieFace::Blank, DieFace::Blank, DieFace::Success, DieFace::SuccessAndAdvantage, DieFace::DoubleAdvantage, DieFace::Advantage][face_index],
        DiceType::Ability => [DieFace::Blank, DieFace::Success, DieFace::Success, DieFace::DoubleSuccess, DieFace::Advantage, DieFace::Advantage, DieFace::SuccessAndAdvantage, DieFace::DoubleAdvantage][face_index],
        DiceType::Proficiency => [DieFace::Blank, DieFace::Success, DieFace::Success, DieFace::DoubleSuccess, DieFace::DoubleSuccess, DieFace::Advantage, DieFace::SuccessAndAdvantage, DieFace::SuccessAndAdvantage, DieFace::SuccessAndAdvantage, DieFace::DoubleAdvantage, DieFace::DoubleAdvantage, DieFace::Triumph][face_index],
        DiceType::Setback => [DieFace::Blank, DieFace::Blank, DieFace::Failure, DieFace::Failure, DieFace::Threat, DieFace::Threat][face_index],
        DiceType::Difficulty => [DieFace::Blank, DieFace::Failure, DieFace::DoubleFailure, DieFace::Threat, DieFace::Threat, DieFace::Threat, DieFace::DoubleThreat, DieFace::FailureAndThreat][face_index],
        DiceType::Challenge => [DieFace::Blank, DieFace::Failure, DieFace::Failure, DieFace::DoubleFailure, DieFace::DoubleFailure, DieFace::Threat,DieFace::Threat,DieFace::FailureAndThreat,DieFace::FailureAndThreat,DieFace::DoubleThreat,DieFace::DoubleThreat,DieFace::Despair][face_index],
        DiceType::Force => [DieFace::SingleBlackPip,DieFace::SingleBlackPip,DieFace::SingleBlackPip,DieFace::SingleBlackPip,DieFace::SingleBlackPip,DieFace::SingleBlackPip,DieFace::DoubleBlackPip,DieFace::SingleWhitePip,DieFace::SingleWhitePip,DieFace::DoubleWhitePip,DieFace::DoubleWhitePip,DieFace::DoubleWhitePip][face_index],
    }
}