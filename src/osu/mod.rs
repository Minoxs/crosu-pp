use rosu_pp::osu::OsuDifficultyAttributes;

mod diff;
mod perf;

#[repr(C)]
#[derive(Default)]
pub struct COsuDifficultyAttributes {
    /// The aim portion of the total strain.
    pub aim: f64,
    /// The speed portion of the total strain.
    pub speed: f64,
    /// The flashlight portion of the total strain.
    pub flashlight: f64,
    /// The ratio of the aim strain with and without considering sliders
    pub slider_factor: f64,
    /// The number of clickable objects weighted by difficulty.
    pub speed_note_count: f64,
    /// The approach rate.
    pub ar: f64,
    /// The overall difficulty
    pub od: f64,
    /// The health drain rate.
    pub hp: f64,
    /// The amount of circles.
    pub n_circles: usize,
    /// The amount of sliders.
    pub n_sliders: usize,
    /// The amount of spinners.
    pub n_spinners: usize,
    /// The final star rating
    pub stars: f64,
    /// The maximum combo.
    pub max_combo: usize,
}

// Macro crimes to implement the conversion between Rust <-> C Types
macro_rules! impl_cosu {
    ($type1:ident, $type2:ident) => {
        impl From<$type1> for $type2 {
            #[inline]
            fn from(attr: $type1) -> Self {
                Self {
                    aim: attr.aim,
                    speed: attr.speed,
                    flashlight: attr.flashlight,
                    slider_factor: attr.slider_factor,
                    speed_note_count: attr.speed_note_count,
                    ar: attr.ar,
                    od: attr.od,
                    hp: attr.hp,
                    n_circles: attr.n_circles,
                    n_sliders: attr.n_sliders,
                    n_spinners: attr.n_spinners,
                    stars: attr.stars,
                    max_combo: attr.max_combo,
                }
            }
        }
    }
}

impl_cosu!(OsuDifficultyAttributes, COsuDifficultyAttributes);
impl_cosu!(COsuDifficultyAttributes, OsuDifficultyAttributes);

#[repr(C)]
#[derive(Default)]
pub struct COsuDiffResult {
    /// Flag that indicates whether difficulty calculation succeeded
    pub success: bool,
    /// PP value if it was calculated along map difficulty, 0 otherwise
    pub pp: f64,
    /// Calculated map difficulty attributes
    pub attr: COsuDifficultyAttributes,
}
