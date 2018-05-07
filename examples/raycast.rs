extern crate ncollide;
extern crate quicksilver;

use quicksilver::{
    State, run,
    geom::Vector,
    input::Event,
    graphics::{Color, GpuTriangle, WindowBuilder, Window, Vertex}
};
use ncollide::{
    query::Ray,
    world::{CollisionGroups, CollisionWorld2}
};
use std::iter::repeat;

struct Raycast {
    // the collision world to raycast against
    world: CollisionWorld2<f32, ()>,
    // the different points in the world to send rays to
    raycast_targets: Vec<Vector>,
    // the vertices to draw to the screen
    vertices: Vec<Vertex>
}

impl State for Raycast {
    fn new() -> Raycast {
        Raycast {
            world: CollisionWorld2::new(1.0),
            raycast_targets: vec![Vector::zero(), Vector::new(400, 0)],
            vertices: Vec::new()
        }
    }

    fn event(&mut self, event: &Event, _: &mut Window) {
        if let &Event::MouseMoved(mouse) = event {
            self.vertices.clear();
            let all_objects = CollisionGroups::new();
            //Insert the mouse as a vertex
            self.vertices.push(Vertex {
                pos: mouse,
                tex_pos: None,
                col: Color::white()
            });
            // Raycast towards all targets and find the vertices
            for i in 0..self.raycast_targets.len() {
                // Create a Ray from the mouse to the target
                let start = mouse.into_point();
                let direction = (self.raycast_targets[i] - mouse).normalize().into_vector();
                let ray = Ray::new(start, direction);
                // Perform the actual raycast, returning the target and an iterator of collisions
                let collisions = self.world.interferences_with_ray(&ray, &all_objects);
                let pos = collisions
                    // Convert the collisions into their collision points
                    .map(|(_, intersect)| ray.origin + intersect.toi * ray.dir)
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .map(|closest| closest.into())
                    .unwrap_or(self.raycast_targets[i]);
                self.vertices.push(Vertex {
                    pos,
                    tex_pos: None,
                    col: Color::white()
                });
            }
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
