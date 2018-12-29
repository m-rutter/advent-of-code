use advent_of_code;
use serde_derive::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct ExportedSolution {
    pub part_one: String,
    pub part_two: String,
}

impl From<advent_of_code::Solution> for ExportedSolution {
    fn from(s: advent_of_code::Solution) -> Self {
        Self {
            part_one: s.part_one,
            part_two: s.part_two,
        }
    }
}

#[wasm_bindgen]
pub fn solve_day(year: u16, day: u8, input: String) -> Result<JsValue, JsValue> {

    let config = advent_of_code::Config::new(year, day, input);

    let solution =
        ExportedSolution::from(advent_of_code::solve_day(&config).map_err(|err| err.to_string())?);

    Ok(JsValue::from_serde(&solution).unwrap())
}
