use nannou::prelude::*;
use crate::lines::common::Line;

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