extern crate nalgebra;
extern crate ncollide2d;
extern crate quicksilver;

use quicksilver::{
    State, run,
    geom::{Rectangle, Vector},
    input::Event,
    graphics::{Color, GpuTriangle, WindowBuilder, Window, Vertex}
};
use nalgebra::{Isometry2, zero};
use ncollide2d::{
    query::{Ray, RayCast}
};
use std::{
    cmp::Ordering,
    iter::repeat
};

struct Raycast {
    // the rectangles to raycast against
    regions: Vec<Rectangle>,
    // the points to send rays to
    targets: Vec<Vector>,
    // the vertices to draw to the screen
    vertices: Vec<Vertex>
}

impl Raycast {
    fn lightsource(&mut self, source: Vector) {
        self.vertices.clear();
        let distance_to = |point: &Vector| (*point - source).len();
        let angle_to = |point: &Vertex| (point.pos - source).angle();
        // Raycast towards all targets and find the vertices
        for i in 0..self.targets.len() {
            let angle = (self.targets[i] - source).angle();
            let mut cast_ray = |direction: f32| {
                // Create a Ray from the source to the target
                let start = source.into_point();
                let direction = Vector::from_angle(direction).into_vector();
                let ray = Ray::new(start, direction);
                // Perform the actual raycast, returning the target and an iterator of collisions
                let identity = Isometry2::new(zero(), zero());
                let pos = self.regions
                    .iter()
                    .filter_map(|region| region
                        .into_aabb()
                        .toi_with_ray(&identity, &ray, false))
                    .map(|toi: f32| (ray.origin + toi * ray.dir).into())
                    .min_by(|a: &Vector, b: &Vector| distance_to(a)
                        .partial_cmp(&distance_to(b))
                        .unwrap_or(Ordering::Equal))
                    .unwrap();
                self.vertices.push(Vertex {
                    pos,
                    tex_pos: None,
                    col: Color::white()
                });
            };
            cast_ray(angle - 0.001);
            cast_ray(angle);
            cast_ray(angle + 0.001);
        }
        // Sort the vertices to form a visibility polygon
        self.vertices.sort_by(|a, b| angle_to(a)
            .partial_cmp(&angle_to(b))
            .unwrap_or(Ordering::Equal));
        //Insert the source as a vertex
        self.vertices.insert(0, Vertex {
            pos: source,
            tex_pos: None,
            col: Color::white()
        });
    }
}

impl State for Raycast {
    fn new() -> Raycast {
        let regions = vec![
            Rectangle::new_sized(800, 600),
            Rectangle::new(200, 200, 100, 100),
            Rectangle::new(400, 200, 100, 100),
            Rectangle::new(400, 400, 100, 100),
            Rectangle::new(200, 400, 100, 100),
            Rectangle::new(50, 50, 50, 50),
            Rectangle::new(550, 300, 64, 64)
        ];
        let targets = regions.iter().flat_map(|region| {
            vec![region.top_left(), 
                region.top_left() + region.size().x_comp(),
                region.top_left() + region.size().y_comp(),
                region.top_left() + region.size()].into_iter()
        }).collect();
        Raycast {
            regions,
            targets,
            vertices: Vec::new()
        }
    }

    fn event(&mut self, event: &Event, _: &mut Window) {
        if let &Event::MouseMoved(mouse) = event {
            self.lightsource(mouse);
        }
    }

    fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        let triangle_count = self.vertices.len() as u32 - 1;
        let indices = repeat(0.0)
            .take(triangle_count as usize)
            .enumerate()
            .map(|(index, z)| GpuTriangle {
                z,
                indices: [0, 
                    index as u32 + 1,
                    (index as u32 + 1) % triangle_count + 1],
                image: None
            });
        window.add_vertices(self.vertices.iter().cloned(), indices);
        window.present();
    }
}

fn main() {
    run::<Raycast>(WindowBuilder::new("Raycast", 800, 600));
}
