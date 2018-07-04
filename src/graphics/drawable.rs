use geom::{Circle, Positioned, Rectangle, Scalar, Shape, Transform, Vector};
use graphics::{Color, GpuTriangle, Image, Vertex, Window};
use std::iter;

/// Some object that can be drawn to the screen
pub trait Drawable {
    /// Draw the object to the window
    fn draw(&self, &mut Window);
}

#[derive(Clone, Debug)]
enum DrawPayload {
    Image(Image),
    Rectangle(Vector),
    Circle(f32),
}

/// A single drawable item, with a transform, a blend color, and a depth
#[derive(Clone, Debug)]
pub struct Sprite {
    item: DrawPayload,
    position: Vector,
    color: Color,
    transform: Transform,
    z: f32,
}

impl Sprite {
    /// Create a sprite with an image
    pub fn image(image: &Image, position: Vector) -> Sprite {
        Sprite { item: DrawPayload::Image(image.clone()),
                 position,
                 color: Color::white(),
                 transform: Transform::identity(),
                 z: 0.0, }
    }

    /// Create a sprite from a given shape
    pub fn shape(shape: Shape) -> Sprite {
        match shape {
            Shape::Circle(circ) => Sprite::circle(circ),
            Shape::Rectangle(rect) => Sprite::rectangle(rect),
            Shape::Vector(v) => Sprite::point(v),
        }
    }

    /// Create a sprite with a point
    pub fn point(position: Vector) -> Sprite {
        Sprite::rectangle(Rectangle::newv(position, Vector::one()))
    }

    /// Create a sprite with a line
    pub fn line(from: Vector, to: Vector, thickness: f32) -> Sprite {
        // create rectangle in right size
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        let rect = Rectangle::new(from.x, from.y + thickness / 2.0, distance, thickness);

        // shift position of rectangle
        let trans_x = (from.x + to.x) / 2.0 - rect.center().x;
        let trans_y = (from.y + to.y) / 2.0 - rect.center().y;

        Sprite { item: DrawPayload::Rectangle(rect.size()),
                 position: rect.center(),
                 color: Color::white(),
                 transform: Transform::translate(Vector::new(trans_x, trans_y))
                            * Transform::rotate(dy.atan2(dx).to_degrees()),
                 z: 0.0, }
    }

    /// Create a sprite with a rectangle
    pub fn rectangle(rectangle: Rectangle) -> Sprite {
        Sprite { item: DrawPayload::Rectangle(rectangle.size()),
                 position: rectangle.center(),
                 color: Color::white(),
                 transform: Transform::identity(),
                 z: 0.0, }
    }

    /// Create a sprite with a circle
    pub fn circle(circle: Circle) -> Sprite {
        Sprite { item: DrawPayload::Circle(circle.radius),
                 position: circle.center(),
                 color: Color::white(),
                 transform: Transform::identity(),
                 z: 0.0, }
    }

    /// Change the position of a sprite
    pub fn with_position(self, position: Vector) -> Sprite { Sprite { position, ..self } }

    /// Change the color of a sprite
    pub fn with_color(self, color: Color) -> Sprite { Sprite { color, ..self } }

    /// Change the transform of a sprite
    pub fn with_transform(self, transform: Transform) -> Sprite { Sprite { transform, ..self } }

    /// Change the depth of a sprite
    pub fn with_z<T: Scalar>(self, z: T) -> Sprite {
        Sprite { z: z.float(),
                 ..self }
    }
}

impl Drawable for Sprite {
    fn draw(&self, window: &mut Window) {
        match self.item {
            DrawPayload::Image(ref image) => {
                let area = image.area().with_center(self.position);
                let trans = Transform::translate(area.top_left() + area.size() / 2)
                            * self.transform
                            * Transform::translate(-area.size() / 2)
                            * Transform::scale(area.size());
                let recip_size = image.source_size().recip();
                let normalized_pos = image.area().top_left().times(recip_size);
                let normalized_size = image.area().size().times(recip_size);
                let get_vertex = |v: Vector| Vertex { pos: trans * v,
                                                      tex_pos: Some(normalized_pos
                                                                    + v.times(normalized_size)),
                                                      col: self.color, };
                let vertices = &[get_vertex(Vector::zero()),
                                 get_vertex(Vector::zero() + Vector::x()),
                                 get_vertex(Vector::zero() + Vector::one()),
                                 get_vertex(Vector::zero() + Vector::y())];
                let triangles = &[GpuTriangle { z: self.z,
                                                indices: [0, 1, 2],
                                                image: Some(image.clone()), },
                                  GpuTriangle { z: self.z,
                                                indices: [2, 3, 0],
                                                image: Some(image.clone()), }];
                window.add_vertices(vertices.iter().cloned(), triangles.iter().cloned());
            }
            DrawPayload::Rectangle(size) => {
                let area = Rectangle::newv_sized(size).with_center(self.position);
                let trans = Transform::translate(area.top_left() + area.size() / 2)
                            * self.transform
                            * Transform::translate(-area.size() / 2)
                            * Transform::scale(area.size());
                let get_vertex = |v: Vector| Vertex { pos: trans * v,
                                                      tex_pos: None,
                                                      col: self.color, };
                let vertices = &[get_vertex(Vector::zero()),
                                 get_vertex(Vector::zero() + Vector::x()),
                                 get_vertex(Vector::zero() + Vector::one()),
                                 get_vertex(Vector::zero() + Vector::y())];
                let triangles = &[GpuTriangle { z: self.z,
                                                indices: [0, 1, 2],
                                                image: None, },
                                  GpuTriangle { z: self.z,
                                                indices: [2, 3, 0],
                                                image: None, }];
                window.add_vertices(vertices.iter().cloned(), triangles.iter().cloned());
            }
            DrawPayload::Circle(radius) => {
                let transform = Transform::translate(self.position)
                                * self.transform
                                * Transform::translate(-self.position);
                let mut points = [Vector::zero(); 24]; // 24 = arbitrarily chosen number of points in the circle
                let rotation = Transform::rotate(360f32 / points.len() as f32);
                let mut arrow = Vector::new(0f32, -radius);
                for i in 0..points.len() {
                    points[i] = arrow + self.position;
                    arrow = rotation * arrow;
                }
                let vertices = points.iter()
                                     .map(|point| Vertex { pos: transform * point.clone(),
                                                           tex_pos: None,
                                                           col: self.color, });
                let indices = iter::repeat(self.z).take(points.len() - 1)
                                                  .enumerate()
                                                  .map(|(index, z)| GpuTriangle { z,
                                                                                  indices:
                                                                                      [0,
                                                                                       index as u32,
                                                                                       index as u32
                                                                                       + 1],
                                                                                  image: None, });
                window.add_vertices(vertices, indices);
            }
        }
    }
}
