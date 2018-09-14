// An example that demonstrates a basic 2D lighting effect

extern crate nalgebra;
extern crate ncollide2d;
extern crate quicksilver;

use nalgebra::{zero, Isometry2};
use ncollide2d::query::{Ray, RayCast};
use quicksilver::{
    Result,
    geom::{Rectangle, Vector},
    graphics::{Color, GpuTriangle, Mesh, Vertex},
    lifecycle::{Event, Settings, State, Window, run},
};
use std::cmp::Ordering;

struct Raycast {
    // the rectangles to raycast against
    regions: Vec<Rectangle>,
    // the points to send rays to
    targets: Vec<Vector>,
    // the vertices to draw to the screen
    mesh: Mesh
}

impl State for Raycast {
    fn new() -> Result<Raycast> {
        //The different squares that cast shadows
        let regions = vec![
            Rectangle::new_sized((800, 600)),
            // Feel free to add or remove rectangles to this list
            // to see the effect on the lighting
            Rectangle::new((200, 200), (100, 100)),
            Rectangle::new((400, 200), (100, 100)),
            Rectangle::new((400, 400), (100, 100)),
            Rectangle::new((200, 400), (100, 100)),
            Rectangle::new((50, 50), (50, 50)),
            Rectangle::new((550, 300), (64, 64))
        ];
        // Build the list of targets to cast rays to
        let targets = regions
            .iter()
            .flat_map(|region| {
                vec![
                    region.top_left(),
                    region.top_left() + region.size().x_comp(),
                    region.top_left() + region.size().y_comp(),
                    region.top_left() + region.size(),
                ].into_iter()
            })
            .collect();
        Ok(Raycast {
            regions,
            targets,
            mesh: Mesh::new(),
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        if let &Event::MouseMoved(_) = event {
            let mouse = window.mouse().pos();
            self.mesh.clear();
            let distance_to = |point: &Vector| (*point - mouse).len();
            let angle_to = |point: &Vertex| (point.pos - mouse).angle();
            // Raycast towards all targets and find the vertices
            for i in 0..self.targets.len() {
                let angle = (self.targets[i] - mouse).angle();
                let mut cast_ray = |direction: f32| {
                    // Create a Ray from the mouse to the target
                    let start = mouse.into_point();
                    let direction = Vector::from_angle(direction).into_vector();
                    let ray = Ray::new(start, direction);
                    // Perform the actual raycast, returning the target and an iterator of collisions
                    let identity = Isometry2::new(zero(), zero());
                    let cast = self.regions
                        .iter()
                        .filter_map(|region| {
                            region.into_aabb().toi_with_ray(&identity, &ray, false)
                        })
                        .map(|toi: f32| (ray.origin + toi * ray.dir).into())
                        .min_by(|a: &Vector, b: &Vector| {
                            distance_to(a)
                                .partial_cmp(&distance_to(b))
                                .unwrap_or(Ordering::Equal)
                        });
                    if let Some(pos) = cast {
                        self.mesh.vertices.push(Vertex {
                            pos,
                            tex_pos: None,
                            col: Color::WHITE,
                        });
                    }
                };
                // Make sure to cast rays around corners to avoid jitteriness
                cast_ray(angle - 0.001);
                cast_ray(angle);
                cast_ray(angle + 0.001);
            }
            // Sort the vertices to form a visibility polygon
            self.mesh.vertices.sort_by(|a, b| {
                angle_to(a)
                    .partial_cmp(&angle_to(b))
                    .unwrap_or(Ordering::Equal)
            });
            // Insert the mouse as a vertex for the center of the polygon
            self.mesh.vertices.insert(
                0,
                Vertex {
                    pos: mouse,
                    tex_pos: None,
                    col: Color::WHITE,
                },
            );
             // Calculate the number of triangles needed to draw the poly
            let triangle_count = self.mesh.vertices.len() as u32 - 1;
            for index in 0..triangle_count {
                self.mesh.triangles.push(GpuTriangle {
                    z: 0.0,
                    indices: [
                        0, 
                        index as u32 + 1,
                        (index as u32 + 1) % triangle_count + 1
                    ],
                    image: None
                });
            }
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        window.mesh().extend(&self.mesh);
        Ok(())
    }
}

fn main() {
    run::<Raycast>("Raycast", Vector::new(800, 600), Settings::default());
}
