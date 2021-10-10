#![allow(unused)]
// #![forbid(unsafe_code)]
// #![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
// #![warn(clippy::all, rust_2018_idioms)]
// #![cfg(not(target_arch = "wasm32"))]

// use std::path::PathBuf;

// When compiling natively:
use crate::plot::PlotDemo;
use nannou::lyon::path::PathEvent;
use nannou::{color::rgb_u32, rand::thread_rng};
use nannou::{prelude::*, rand::prelude::SliceRandom};
use nannou_egui::{self, egui, Egui};

pub use crate::appearance::{Color, Visual, WithVisual};
pub use crate::geom::Path;
pub use crate::geom::{pt, Angle, Point, Pose, Rect, Size, Spatial, Vector, WithSpatial};

mod appearance;
mod data;
mod geom;
mod math;
mod plot;
mod vehicle;

pub type S = f32;

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: Hsv,
}

struct Settings {
    min_radius: f32,
    max_radius: f32,
    circle_count: usize,
}

struct Model {
    circles: Vec<Circle>,
    settings: Settings,
    egui: Egui,
    plot_demo: PlotDemo,
    image: wgpu::Texture,
    svg: SvgPath,
}
#[derive(Clone)]
struct SvgPath {
    events: Vec<PathEvent>,
    weight: f32,
    color: Rgba,
    width: f32,
    height: f32,
}

impl SvgPath {
    fn new(events: Vec<PathEvent>, weight: f32, color: Rgba, width: f32, height: f32) -> Self {
        SvgPath {
            events,
            weight,
            color,
            width,
            height,
        }
    }
}

fn point(x: &f64, y: &f64) -> Point2D<f32, UnknownUnit> {
    Point2D::new((*x) as f32, (*y) as f32)
}

use nannou::lyon::geom::euclid::{Point2D, UnknownUnit};
pub struct PathConvIter<'a> {
    iter: std::slice::Iter<'a, usvg::PathSegment>,
    prev: Point2D<f32, UnknownUnit>,
    first: Point2D<f32, UnknownUnit>,
    needs_end: bool,
    deferred: Option<PathEvent>,
}

impl<'l> Iterator for PathConvIter<'l> {
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        if self.deferred.is_some() {
            return self.deferred.take();
        }

        let next = self.iter.next();
        match next {
            Some(usvg::PathSegment::MoveTo { x, y }) => {
                if self.needs_end {
                    let last = self.prev;
                    let first = self.first;
                    self.needs_end = false;
                    self.prev = point(x, y);
                    self.deferred = Some(PathEvent::Begin { at: self.prev });
                    self.first = self.prev;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    self.first = point(x, y);
                    Some(PathEvent::Begin { at: self.first })
                }
            }
            Some(usvg::PathSegment::LineTo { x, y }) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point(x, y);
                Some(PathEvent::Line {
                    from,
                    to: self.prev,
                })
            }
            Some(usvg::PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            }) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point(x, y);
                Some(PathEvent::Cubic {
                    from,
                    ctrl1: point(x1, y1),
                    ctrl2: point(x2, y2),
                    to: self.prev,
                })
            }
            Some(usvg::PathSegment::ClosePath) => {
                self.needs_end = false;
                self.prev = self.first;
                Some(PathEvent::End {
                    last: self.prev,
                    first: self.first,
                    close: true,
                })
            }
            None => {
                if self.needs_end {
                    self.needs_end = false;
                    let last = self.prev;
                    let first = self.first;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    None
                }
            }
        }
    }
}
pub fn convert_path<'a>(p: &'a usvg::Path) -> PathConvIter<'a> {
    PathConvIter {
        iter: p.segments.iter(),
        first: Point2D::new(0.0, 0.0),
        prev: Point2D::new(0.0, 0.0),
        deferred: None,
        needs_end: false,
    }
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let plot_demo = PlotDemo::default();

    let asset = app.assets_path().unwrap().join("car.svg");
    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_file(&asset, &opt).unwrap();
    let view_box = rtree.svg_node().view_box;

    let mut svg: Option<SvgPath> = None;
    for node in rtree.root().descendants() {
        if let usvg::NodeKind::Path(ref p) = *node.borrow() {
            if let Some(ref stroke) = p.stroke {
                let color = match stroke.paint {
                    usvg::Paint::Color(c) => rgba(
                        c.red as f32 / 255.0,
                        c.green as f32 / 255.0,
                        c.blue as f32 / 255.0,
                        1.0,
                    ),
                    _ => rgba(0.0, 0.0, 0.0, 1.0),
                };

                let path_events = convert_path(p);
                let mut v = Vec::new();
                for e in path_events {
                    v.push(e);
                }
                let w = view_box.rect.size().width as f32;
                let h = view_box.rect.size().height as f32;
                svg = Some(SvgPath::new(v, stroke.width.value() as f32, color, w, h));
            }
        }
    }
    let img_path = app.assets_path().unwrap().join("car.png");
    // let image = image::open(img_path).unwrap();
    let image = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        circles: Vec::new(),
        egui,
        settings: Settings {
            min_radius: 10.0,
            max_radius: 100.0,
            circle_count: 10,
        },
        plot_demo,
        image,
        svg: svg.unwrap(),
    }
}

struct Image {
    image: wgpu::Texture,
    pub width: usize,
    pub height: usize,
    pub angle: f32,
}

// impl Image {
//     fn new(img_path: PathBuf, app: &App) -> Self {
//         let image = wgpu::Texture::from_path(app, img_path).unwrap();
//     }
// }

fn update(_app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut settings,
        ref mut circles,
        ref mut plot_demo,
        ..
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Workshop window").show(&ctx, |ui| {
        let mut changed = false;
        changed |= ui
            .add(egui::Slider::new(&mut settings.min_radius, 0.0..=20.0).text("min radius"))
            .changed();
        changed |= ui
            .add(egui::Slider::new(&mut settings.max_radius, 0.0..=200.0).text("max radius"))
            .changed();
        changed |= ui
            .add(egui::Slider::new(&mut settings.circle_count, 0..=2000).text("circle count"))
            .changed();
        changed |= ui.button("Generate").clicked();
        if changed {
            *circles = generate_circles(settings);
        }
    });
    plot_demo.show(&ctx, &mut true);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for circle in model.circles.iter() {
        draw.ellipse()
            .x_y(circle.x, circle.y)
            .radius(circle.radius)
            .color(circle.color);
    }

    let e = model.svg.events.iter().cloned();

    let img_size = model.image.size();
    let scale = 5.0;
    draw.texture(&model.image)
        .z_turns(0.125)
        .w_h((img_size[0] as f32) / scale, (img_size[1] as f32) / scale);

    draw.path().fill().color(WHITE).events(e).x_y(0.0, 0.0);

    draw.line()
        .points(pt2(0.0, 0.0), pt2(10.0, 10.0))
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();

    model.egui.draw_to_frame(&frame);
}

fn intersects(circle: &Circle, circles: &[Circle]) -> bool {
    for other in circles.iter() {
        let dist: f32 =
            ((other.x - circle.x).pow(2) as f32 + (other.y - circle.y).pow(2) as f32).sqrt();
        if dist < circle.radius + other.radius {
            return true;
        }
    }
    false
}

fn generate_circles(settings: &mut Settings) -> Vec<Circle> {
    let colors = [
        hsv_from_hex_rgb(0x264653),
        hsv_from_hex_rgb(0x2a9d8f),
        hsv_from_hex_rgb(0xe9c46a),
        hsv_from_hex_rgb(0xf4a261),
        hsv_from_hex_rgb(0xe76f51),
    ];

    let mut circles = Vec::new();

    let mut rng = thread_rng();

    let mut loops = 0;
    loop {
        let x = random_range(-WIDTH / 2.0, WIDTH / 2.0);
        let y = random_range(-HEIGHT / 2.0, HEIGHT / 2.0);
        let radius = random_range(settings.min_radius, settings.max_radius);
        let color = *colors.choose(&mut rng).unwrap();
        let mut circle = Circle {
            x,
            y,
            radius,
            color,
        };

        loops += 1;
        if loops > 20000 {
            break;
        }

        if intersects(&circle, &circles) {
            continue;
        }

        let mut prev_radius = circle.radius;
        while !intersects(&circle, &circles) {
            // Grow the circle
            prev_radius = circle.radius;
            circle.radius += 10.0;

            if circle.radius >= settings.max_radius {
                break;
            }
        }
        circle.radius = prev_radius;

        circles.push(circle);

        if circles.len() >= settings.circle_count {
            break;
        }
    }

    circles
}

fn hsv_from_hex_rgb(color: u32) -> Hsv {
    let color = rgb_u32(color);
    rgba(
        color.red as f32 / 255.0,
        color.green as f32 / 255.0,
        color.blue as f32 / 255.0,
        1.0,
    )
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pass() {
        assert_eq!(1, 1);
    }
}
