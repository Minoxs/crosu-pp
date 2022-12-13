use rosu_pp::{Beatmap, BeatmapExt, DifficultyAttributes};

use crate::CScore;

/// Helper function to get pp from already calculated attributes
pub fn get_pp(attr: DifficultyAttributes, score: CScore) -> f64 {
    Beatmap::default()
        .pp()
        .attributes(attr)
        .combo(score.combo)
        .n300(score.n300)
        .n100(score.n100)
        .n50(score.n050)
        .calculate()
        .pp()
}
