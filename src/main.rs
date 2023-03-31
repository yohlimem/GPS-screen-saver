use std::{path::Path, ffi::OsStr, fs};

use nannou::{prelude::*};
use rand::Rng;


struct Model {
    _window: window::Id,
    last_mouse_pos: Vec2,
    random_number_x1:f32,
    random_number_y1:f32,
    random_number_x2:f32,
    random_number_y2:f32,
    random_number_x3:f32,
    random_number_y3:f32,
}

fn main() {

    nannou::app(model).fullscreen().update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mouse_pos = app.mouse.position();
    let mut thread = rand::thread_rng();
    let random_number_x1 = thread.gen_range(0.0..500.0);
    let random_number_y1 = thread.gen_range(0.0..200.0);
    let random_number_x2 = thread.gen_range(-500.0..500.0);
    let random_number_y2 = thread.gen_range(0.0..400.0);
    let random_number_x3 = thread.gen_range(-500.0..0.0);
    let random_number_y3 = thread.gen_range(-200.0..0.0);
    Model {
        _window,
        last_mouse_pos: mouse_pos,
        random_number_x1,
        random_number_y1,
        random_number_x2,
        random_number_y2,
        random_number_x3,
        random_number_y3,

    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.mouse.position() != model.last_mouse_pos && app.duration.since_start.secs() >= 1.0{
        app.quit();

    }
    if app.mouse.position() != model.last_mouse_pos {
        model.last_mouse_pos = app.mouse.position();
        // model.last_mouse_pos = app.mouse.position();
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    let bounds = app.window_rect();
    let x = map_range((app.time / 3.0).sin(), -1.0, 1.0, bounds.right() - 60.0, bounds.left() + 60.0);
    let y = map_range((app.time/5.0).cos(), -1.0, 1.0, bounds.top() - 100.0, bounds.bottom() + 100.0);
    let point = vec2(x, y);
    let point1 = vec2(model.random_number_x1,model.random_number_y1);
    let point2 = vec2(model.random_number_x2,model.random_number_y2);
    let point3 = vec2(model.random_number_x3,model.random_number_y3);
    let distance1 = point.distance(point1);
    let distance2 = point.distance(point2);
    let distance3 = point.distance(point3);
    
    draw.ellipse().radius(distance1).color(BLACK).xy(point1).no_fill().stroke_weight(1.0);
    draw.ellipse().radius(distance2).color(BLACK).xy(point2).no_fill().stroke_weight(1.0);
    draw.ellipse().radius(distance3).color(BLACK).xy(point3).no_fill().stroke_weight(1.0);
    draw.ellipse().radius(8.0).color(DODGERBLUE).xy(point);
    draw.background().color(WHITE);
    
    
    draw.to_frame(app, &frame).unwrap();

}
