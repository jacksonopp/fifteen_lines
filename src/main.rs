use nannou::{prelude::*, color::named, lyon::lyon_tessellation::LineCap};

mod models;
use models::Model;

mod events;

mod lines;

fn main() {
    nannou::app(model).event(event).run()
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(1920, 1080)
        .view(view)
        .build()
        .unwrap();
    
    Model::new(window)
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => {
            if let Some(event) = simple {
                match event {
                    KeyReleased(Key::S) => events::screenshot(app),
                    _ => ()
                }
            }
        },

        Event::Update(update) => events::update(app, model, update),
        _ => ()
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background()
        .color(CORNSILK);

    model.lines.iter().for_each(|line| {
        line.lines.iter().for_each(|l| {
            draw.line()
                .points(l.start, l.end)
                .caps(LineCap::Round)
                .weight(l.weight)
                .color(Rgba::new(0.0, 0.0, 0.0, l.value));
        })
    });

    draw.to_frame(app, &frame).unwrap();

}