#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use libc::c_char;
    use rosu_pp::{Beatmap, BeatmapExt};
    use rosu_pp::GameMode::Osu;
    use rosu_pp::osu::{OsuAttributeProvider};
    use crosu_pp::{CGameMode, CScore};
    use crosu_pp::diff::get_difficulty;
    use crosu_pp::perf::{GetPPFromMap};

    #[test]
    fn test_get_difficulty_from_file() {
        const FILE_TEST: &str = "./tests/files/2785319.osu";
        let expected = Beatmap::from_path(FILE_TEST).unwrap().stars().mods(0).mode(Osu).calculate();

        let c_str = CString::new(FILE_TEST).unwrap();
        let c_char_ptr = c_str.as_ptr() as *const c_char;

        let actual = get_difficulty(c_char_ptr, true, 0, Osu).unwrap();
        assert_eq!(expected.stars(), actual.stars());
        assert_eq!(expected.max_combo(), actual.max_combo());
        assert_eq!(OsuAttributeProvider::attributes(expected), OsuAttributeProvider::attributes(actual))
    }

    #[test]
    fn test_get_difficulty_from_bytes() {
        const FILE_TEST: &str = "./tests/files/2785319.osu";
        let file = std::fs::read(FILE_TEST).unwrap();
        let expected = Beatmap::from_bytes(&*file).unwrap().stars().mods(0).mode(Osu).calculate();

        let c_str = CString::new(file).unwrap();
        let c_char_ptr = c_str.as_ptr() as *const c_char;

        let actual = get_difficulty(c_char_ptr, false, 0, Osu).unwrap();
        assert_eq!(expected.stars(), actual.stars());
        assert_eq!(expected.max_combo(), actual.max_combo());
        assert_eq!(OsuAttributeProvider::attributes(expected), OsuAttributeProvider::attributes(actual))
    }

    #[test]
    fn test_get_pp_from_map() {
        const FILE_TEST: &str = "./tests/files/2785319.osu";
        let file = std::fs::read(FILE_TEST).unwrap();
        let attrs = Beatmap::from_bytes(&*file).unwrap().stars().mods(0).mode(Osu).calculate();

        let c_str = CString::new(file).unwrap();
        let c_char_ptr = c_str.as_ptr() as *const c_char;

        let actual = GetPPFromMap(
            c_char_ptr, false, 0, CGameMode::Osu,
            CScore{
                combo: attrs.max_combo(),
                n300: attrs.max_combo(),
                n100: 0,
                n050: 0,
                misses: 0,
            }
        );

        println!("{} {}", actual, attrs.max_combo());
        assert!(actual > 200.0);
    }
}
