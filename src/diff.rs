use std::ffi::CStr;

use rosu_pp::{Beatmap, BeatmapExt, DifficultyAttributes, GameMode, ParseResult};

use crate::CharPtr;

/// Helper function that gets difficulty from beatmap
/// If is_file is true, treats the ptr to a string to the file path
/// Otherwise treats the ptr as a byte array
pub fn get_difficulty(ptr: CharPtr, is_file: bool, mods: u32, mode: GameMode) -> Result<DifficultyAttributes, String> {
    let parse: ParseResult<Beatmap>;
    let arr = unsafe { CStr::from_ptr(ptr) };

    if is_file {
        let path = match arr.to_str() {
            Ok(s) => s,
            Err(why) => return Err(why.to_string())
        };
        parse = Beatmap::from_path(path);
    } else {
        let bytes = arr.to_bytes();
        parse = Beatmap::from_bytes(bytes);
    }

    return match parse {
        Ok(map) => Ok(map.stars().mods(mods).mode(mode).calculate()),
        Err(why) => Err(why.to_string())
    };
}
