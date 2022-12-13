use rosu_pp::GameMode;
use rosu_pp::osu::OsuAttributeProvider;

use crate::{CharPtr, CScore};
use crate::diff::get_difficulty;
use crate::osu::{COsuDifficultyAttributes, COsuDiffResult};
use crate::perf::get_pp;

#[no_mangle]
/// Gets Difficulty attributes of Osu Map
pub extern "C" fn GetOsuDifficultyAttributes(ptr: CharPtr, is_file: bool, mods: u32) -> COsuDiffResult {
    let res = get_difficulty(ptr, is_file, mods, GameMode::Osu);

    let diff = match res {
        Ok(m) => { m.attributes().unwrap() }
        Err(why) => {
            println!("[crosu] Error while calculating map difficulty: {}", why);
            return COsuDiffResult::default();
        }
    };

    COsuDiffResult {
        success: true,
        pp: 0.0,
        attr: COsuDifficultyAttributes::from(diff),
    }
}

#[no_mangle]
/// Calculates osu map difficulty and returns pp in the same call
pub extern "C" fn GetOsuPPFromMap(ptr: CharPtr, is_file: bool, mods: u32, score: CScore) -> COsuDiffResult {
    let res = get_difficulty(ptr, is_file, mods, GameMode::Osu);

    let pp;
    let diff = match res {
        Ok(m) => { pp = get_pp(m.clone(), score); m.attributes().unwrap() }
        Err(why) => {
            println!("[crosu] Error while calculating map difficulty: {}", why);
            return COsuDiffResult::default();
        }
    };

    COsuDiffResult {
        success: true,
        pp,
        attr: COsuDifficultyAttributes::from(diff),
    }
}
