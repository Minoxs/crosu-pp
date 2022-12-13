use rosu_pp::GameMode;
use rosu_pp::osu::OsuAttributeProvider;

use crate::CharPtr;
use crate::diff::get_difficulty;
use crate::osu::{COsuDifficultyAttributes, COsuDiffResult};

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
        attr: COsuDifficultyAttributes::from(diff),
    }
}
