#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_doc_code_examples)]

pub mod dice;
pub mod dice_result;
pub mod dice_set;
pub mod error;
pub mod roll;
pub mod die_face;
pub use genesys_dice_command_parser::dice::Dice as DiceType;