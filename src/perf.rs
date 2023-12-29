use std::ffi::CStr;
use rosu_pp::{Beatmap, BeatmapExt, DifficultyAttributes, ParseResult};

use crate::{CGameMode, CharPtr, CScore};

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
pub extern "C" fn GetPPFromMap(ptr: CharPtr, is_file: bool, mods: u32, c_mode: CGameMode, score: CScore) -> f64 {
    let mode = c_mode.to_rust();

    let parse: ParseResult<Beatmap>;
    let arr = unsafe { CStr::from_ptr(ptr) };

    if is_file {
        let path = match arr.to_str() {
            Ok(s) => s,
            Err(_) => return 0.0
        };
        parse = Beatmap::from_path(path);
    } else {
        let bytes = arr.to_bytes();
        parse = Beatmap::from_bytes(bytes);
    }

    return match parse {
        Ok(map) => {
            map.pp()
               .mode(mode)
               .mods(mods)
               .combo(score.combo)
               .n300(score.n300)
               .n100(score.n100)
               .n50(score.n050)
               .n_misses(score.misses)
               .calculate()
               .pp()
        }
        Err(_) => { 0.0 }
    };
}
