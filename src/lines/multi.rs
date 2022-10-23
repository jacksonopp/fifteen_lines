use crate::lines::{common::Line, solid::SolidLine};
use nannou::prelude::*;

pub struct Cap {
    pub pos: Point2,
    pub radius: f32,
}

impl Cap {
    pub fn new(pos: Point2, weight: f32) -> Self {
        Self {
            pos,
            radius: weight,
        }
    }
}

pub struct MultiLine {
    pub start: Cap,
    pub end: Cap,
    pub weight: f32,
    pub value: f32,
    pub lines: Vec<SolidLine>,
}

impl Line for MultiLine {
    fn line(x1: f32, y1: f32, x2: f32, y2: f32, weight: f32, value: f32) -> Self {
        let start_cap_pos = Point2::new(x1, y1);
        let start_cap = Cap::new(start_cap_pos, weight);

        let end_cap_pos = Point2::new(x2, y2);
        let end_cap = Cap::new(end_cap_pos, weight);

        let mut lines = vec![];

        let density = weight.floor() as u32 * 3;

        println!("density {density}");

        for _ in 0..density {
            let start = get_pos_in_circle(&start_cap.pos, start_cap.radius);
            let end = get_pos_in_circle(&end_cap.pos, end_cap.radius);
            let line = SolidLine::line(
                start.x,
                start.y,
                end.x,
                end.y,
                1.0,
                value,
            );

            lines.push(line);
        }

        Self {
            start: start_cap,
            end: end_cap,
            weight,
            value,
            lines,
        }
    }
}

fn get_pos_in_circle(center: &Point2, radius: f32) -> Point2 {
    let r = radius * random_f32().sqrt();
    let theta = random_f32() * 2.0 * PI;

    Point2::new(center.x + r + theta.cos(), center.y + r + theta.sin())
}