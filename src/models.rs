use nannou::prelude::*;

/// The common trait for drawing a line
trait Line {
    fn line(x1: f32, y1: f32, x2: f32, y2: f32, weight: f32, value: f32) -> Self;
}

pub struct SolidLine {
    pub start: Point2,
    pub end: Point2,
    pub weight: f32,
    pub value: f32,
}

impl Line for SolidLine {
    fn line(x1: f32, y1: f32, x2: f32, y2: f32, weight: f32, value: f32) -> Self {
        Self {
            start: Point2::new(x1, y1),
            end: Point2::new(x2, y2),
            weight: weight,
            value: value,
        }
    }
}

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
