pub type CharPtr = *const libc::c_char;

mod diff;
mod perf;
mod osu;

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
}
