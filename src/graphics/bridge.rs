extern crate sdl2;

use geom::{Circle, Rectangle, Vector, Transform};
use graphics::{Backend, Color, TextureRegion, Vertex};
use sdl2::video::Window;
use std::sync::mpsc::{channel, Receiver, Sender};


pub enum Drawable<'a> {
    Clear,
    Present,
    Image(TextureRegion<'a>),
    Rect(Rectangle),
    Circ(Circle)
}

pub type Payload<'a> = (Drawable<'a>, Transform, Color);
pub type BridgeFront<'a> = Sender<Payload<'a>>;
pub type BridgeBack<'a> = Receiver<Payload<'a>>;

pub struct Bridge<'a> {
    front: BridgeFront<'a>,
    back: BridgeBack<'a>
}

const CIRCLE_POINTS: usize = 32; //the number of points in the poly to simulate the circle

impl<'a> Bridge<'a> {
    pub fn new() -> Bridge<'a> {
        let (tx, rx) = channel::<Payload>();
        Bridge { front: tx, back: rx }
    }

    pub fn get_front(&self) -> BridgeFront<'a> {
        self.front.clone()
    }

    pub fn process_drawable(&self, backend: &mut Backend, window: &Window) {
        let (drawable, transform, color) = self.back.recv().unwrap();
        match drawable {
            Drawable::Clear => backend.clear(color),
            Drawable::Present => {
                backend.flip();
                window.gl_swap_window();
            },
            Drawable::Image(texture) => {
                let recip_size = texture.source_size().recip();
                let normalized_pos = texture.get_region().top_left().times(recip_size);
                let normalized_size = texture.get_region().size().times(recip_size);
                let get_vertex = |v: Vector| {
                    Vertex {
                        pos: transform * v,
                        tex_pos: normalized_pos + v.times(normalized_size),
                        col: color
                    }
                };
                backend.add(texture.get_id(), &[get_vertex(Vector::zero()),
                            get_vertex(Vector::zero() + Vector::x()),
                            get_vertex(Vector::zero() + Vector::one()),
                            get_vertex(Vector::zero() + Vector::y())],
                            &[0, 1, 2, 2, 3, 0]);
            },
            Drawable::Rect(rect) => {
                self.process_polygon(backend, &[rect.top_left(), 
                                  rect.top_left() + rect.size().x_comp(),
                                  rect.top_left() + rect.size(),
                                  rect.top_left() + rect.size().y_comp()], transform, color);
            },
            Drawable::Circ(circ) => {
                let mut points = [Vector::zero(); CIRCLE_POINTS];
                let rotation = Transform::rotate(360f32 / CIRCLE_POINTS as f32);
                let mut arrow = Vector::new(0f32, -circ.radius);
                for i in 0..CIRCLE_POINTS {
                    points[i] = circ.center() + arrow;
                    arrow = rotation * arrow;
                }
                self.process_polygon(backend, &points, transform, color);
            }
        }
    }
    
    fn process_polygon(&self, backend: &mut Backend, vertices: &[Vector], trans: Transform, col: Color) {
        let first_index = backend.num_vertices() as u32;
        for vertex in vertices {
            backend.add_vertex(&Vertex {
                pos: trans * vertex.clone(),
                tex_pos: Vector::zero(),
                col: col
            });
        }
        let mut current = 1;
        let mut i = 0;
        let indices = (vertices.len() - 2) * 3;
        while i < indices {
            backend.add_index(first_index);
            backend.add_index(first_index + current);
            backend.add_index(first_index + current + 1);
            current += 1;
            i += 3;
        }
    }
}

