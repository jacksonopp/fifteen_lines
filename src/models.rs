use nannou::prelude::*;

use crate::lines::{common::Line, multi::MultiLine, solid::SolidLine};

pub struct Model {
    pub window: WindowId,
    pub lines: Vec<MultiLine>,
}

const MAX_LINES: u8 = 10;
const OFFSET: f32 = 50.0;
const START_X: f32 = 100.0;
const RANGE: f32 = 5.0;

impl Model {
    pub fn new(w: WindowId) -> Model {
        let mut lines = vec![];

        for i in 0..MAX_LINES {
            // create_solid_lines(i, &mut lines);
            let (x1, y1, x2, y2) = create_start_end(i);
            let weight = (i + 1) as f32;
            let value = map_range((i + 1) as f32, 0.0f32, MAX_LINES as f32, 0.0f32, 1.0f32);
            let line = MultiLine::line(x1, y1, x2, y2, weight, value);
            lines.push(line);
        }

        Model { window: w, lines }
    }
}

fn create_solid_lines(i: u8, lines: &mut Vec<SolidLine>) {
    let (x1, y1, x2, y2) = create_start_end(i);
    let weight = i as f32;
    let value = map_range((i + 1) as f32, 0.0f32, MAX_LINES as f32, 0.0f32, 1.0f32);
    let line = SolidLine::line(x1, y1, x2, y2, weight, value);
    lines.push(line);
}

fn create_start_end(i: u8) -> (f32, f32, f32, f32) {
    let offset = ((i as f32) - (MAX_LINES as f32) / 2.0) * OFFSET;
    let x1 = -START_X;
    let y1 = offset + random_range(-RANGE, RANGE);
    let x2 = START_X;
    let y2 = offset + random_range(-RANGE, RANGE);
    (x1, y1, x2, y2)
}
