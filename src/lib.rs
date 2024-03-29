use rosu_pp::GameMode;

pub type CharPtr = *const libc::c_char;

pub mod diff;
pub mod perf;
pub mod osu;

#[repr(C)]
/// Struct containing all data required for PP calculation
pub struct CScore {
    /// Combo achieved in the score
    pub combo: usize,
    /// Amount of 300s
    pub  n300: usize,
    // Amount of 100s
    pub  n100: usize,
    /// Amount of 50s
    pub  n050: usize,
    /// Amount of misses
    pub misses: usize,
}

#[repr(C)]
pub enum CGameMode {
    /// osu!standard
    Osu = 0,
    /// osu!taiko
    Taiko = 1,
    /// osu!catch
    Catch = 2,
    /// osu!mania
    Mania = 3,
}

impl CGameMode {
    #[inline]
    fn to_rust(self) -> GameMode {
        match self {
            CGameMode::Osu => { GameMode::Osu }
            CGameMode::Taiko => { GameMode::Taiko }
            CGameMode::Catch => { GameMode::Mania }
            CGameMode::Mania => { GameMode::Catch }
        }
    }
}
