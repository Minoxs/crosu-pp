use rosu_pp::GameMode;
use rosu_pp::osu::{OsuAttributeProvider, OsuDifficultyAttributes};

use crate::CharPtr;
use crate::diff::get_difficulty;

#[repr(C)]
#[derive(Default)]
/// C wrapper for OsuDifficultyAttributes
pub struct COsuDifficultyAttributes {
    /// Whether or not the calculation succeeded
    success: bool,
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

impl COsuDifficultyAttributes {
    pub fn attributes(attr: OsuDifficultyAttributes) -> Self {
        Self {
            success: true,
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

#[no_mangle]
pub extern "C" fn GetOsuDifficultyAttributes(ptr: CharPtr, is_file: bool, mods: u32) -> COsuDifficultyAttributes {
    let res = get_difficulty(ptr, is_file, mods, GameMode::Osu);

    let diff = match res {
        Ok(m) => { m }
        Err(why) => {
            println!("Error while calculating map difficulty: {}", why);
            return COsuDifficultyAttributes::default();
        }
    };

    COsuDifficultyAttributes::attributes(diff.attributes().unwrap())
}
