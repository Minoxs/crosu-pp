use rosu_pp::DifficultyAttributes;
use rosu_pp::osu::OsuDifficultyAttributes;

use crate::CScore;
use crate::osu::COsuDifficultyAttributes;
use crate::perf::get_pp;

#[no_mangle]
/// Gets PP from osu difficulty attributes
/// Mods must have already been calculated
pub extern "C" fn GetOsuPP(diff: COsuDifficultyAttributes, score: CScore) -> f64 {
    get_pp(DifficultyAttributes::from(OsuDifficultyAttributes::from(diff)), score)
}
