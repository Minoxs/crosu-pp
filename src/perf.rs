use rosu_pp::{Beatmap, BeatmapExt, DifficultyAttributes};

use crate::{CGameMode, CharPtr, CScore};
use crate::diff::get_difficulty;

#[inline]
/// Helper function to get pp from already calculated attributes
pub fn get_pp(attr: DifficultyAttributes, score: CScore) -> f64 {
    Beatmap::default()
        .pp()
        .attributes(attr)
        .combo(score.combo)
        .n300(score.n300)
        .n100(score.n100)
        .n50(score.n050)
        .n_misses(score.misses)
        .calculate()
        .pp()
}

#[no_mangle]
/// Gets PP from map without having to deal with receiving/sending struct with difficulty attributes
/// Should be less performant since will re-calculate the same map over and over, but faster to get up and running
pub extern "C" fn GetPPFromMap(ptr: CharPtr, is_file: bool, mods: u32, mode: CGameMode, score: CScore) -> f64 {
    return match get_difficulty(ptr, is_file, mods,mode.to_rust()) {
        Ok(m) => { get_pp(m, score) }
        Err(_) => { 0.0 }
    }
}
