extern crate nalgebra;
extern crate ncollide;
extern crate quicksilver;

use quicksilver::{
    State, run,
    geom::{Rectangle, Vector},
    input::Event,
    graphics::{Color, GpuTriangle, WindowBuilder, Window, Vertex}
};
use nalgebra::{Isometry2, zero};
use ncollide::{
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

impl State for Raycast {
    fn new() -> Raycast {
        let regions = vec![
            Rectangle::new_sized(800, 600),
            Rectangle::new(200, 200, 400, 400)
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
            let distance_to = |point: &Vector| (*point - mouse).len();
            let angle_to = |point: &Vertex| (point.pos - mouse).angle();
            self.vertices.clear();
            // Raycast towards all targets and find the vertices
            for i in 0..self.targets.len() {
                // Create a Ray from the mouse to the target
                let start = mouse.into_point();
                let direction = (self.targets[i] - mouse).normalize().into_vector();
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
                    .unwrap_or(self.targets[i]);
                self.vertices.push(Vertex {
                    pos,
                    tex_pos: None,
                    col: Color::white()
                });
            }
            // Sort the vertices to form a visibility polygon
            self.vertices.sort_by(|a, b| angle_to(a)
                .partial_cmp(&angle_to(b))
                .unwrap_or(Ordering::Equal));
            //Insert the mouse as a vertex
            self.vertices.push(Vertex {
                pos: mouse,
                tex_pos: None,
                col: Color::white()
            });
        }
    }

    fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        if self.vertices.len() >= 3 {
            let indices = repeat(0.0).take(self.vertices.len() - 1).enumerate().map(|(index, z)| GpuTriangle {
                z,
                indices: [0, index as u32, index as u32 + 1],
                image: None
            });
            window.add_vertices(self.vertices.iter().cloned(), indices);
        }
        window.present();
    }
}

fn main() {
    run::<Raycast>(WindowBuilder::new("Raycast", 800, 600));
}
