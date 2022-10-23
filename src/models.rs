use nannou::prelude::*;

use crate::lines::{solid::SolidLine, common::Line};

pub struct Model {
    pub window: WindowId,
    pub lines: Vec<SolidLine>
}

const MAX_LINES: u8 = 10;
const OFFSET: f32 = 50.0;
const START_X: f32 = 100.0;
const RANGE: f32 = 5.0;

impl Model {
    pub fn new(w: WindowId) -> Model {
        let mut lines = vec![];

        for i in 0..MAX_LINES {
            let offset = ((i as f32) - (MAX_LINES as f32) / 2.0) * OFFSET;

            let x1 = -START_X;
            let y1 = offset + random_range(-RANGE, RANGE);
            
            let x2 = START_X;
            let y2 = offset + random_range(-RANGE, RANGE);

            let weight = i as f32;
            let value = map_range((i + 1) as f32, 0.0f32, MAX_LINES as f32, 0.0f32, 1.0f32);

            let line = SolidLine::line(x1, y1, x2, y2, weight, value);
            println!("{value}");
            lines.push(line);
        }

        Model { window: w, lines }
    }
}
