use geom::{Circle, Positioned, Rectangle, Scalar, Transform, Vector};
use graphics::{Backend, Color, Image, Vertex};
use std::cmp::Ordering;

#[derive(Clone)]
pub(crate) enum DrawPayload {
    Image((Image, Vector)),
    Rectangle(Rectangle),
    Circle(Circle),
}

/// A single drawable item, with a transform, a blend color, and a depth
#[derive(Clone)]
pub struct DrawCall {
    item: DrawPayload,
    color: Color,
    transform: Transform,
    z: f32
}

impl DrawCall {
    /// Create a draw call with an image
    pub fn image(image: &Image, center: Vector) -> DrawCall {
        DrawCall {
            item: DrawPayload::Image((image.clone(), center)),
            color: Color::white(),
            transform: Transform::identity(),
            z: 0.0
        }
    }

    /// Create a draw call with a rectangle
    pub fn rectangle(rectangle: Rectangle) -> DrawCall {
        DrawCall {
            item: DrawPayload::Rectangle(rectangle),
            color: Color::white(),
            transform: Transform::identity(),
            z: 0.0
        }
    }

    /// Create a draw call with a circle
    pub fn circle(circle: Circle) -> DrawCall {
        DrawCall {
            item: DrawPayload::Circle(circle),
            color: Color::white(),
            transform: Transform::identity(),
            z: 0.0
        }
    }

    /// Change the color of a draw call
    pub fn with_color(self, color: Color) -> DrawCall {
        DrawCall {
            color,
            ..self
        }
    }

    /// Change the transform of a draw call
    pub fn with_transform(self, transform: Transform) -> DrawCall {
        DrawCall {
            transform,
            ..self
        }
    }

    /// Change the depth of a draw call
    pub fn with_z<T: Scalar>(self, z: T) -> DrawCall {
        DrawCall {
            z: z.float(),
            ..self
        }
    }

    pub(crate) fn apply(&self, camera: Transform, backend: &mut Backend) {
        match self.item {
            DrawPayload::Image(ref data) => {
                let &(ref image, center) = data;
                let area = image.area().with_center(center);
                let trans = camera
                    * Transform::translate(area.top_left() + area.size() / 2) 
                    * self.transform
                    * Transform::translate(-area.size() / 2)
                    * Transform::scale(area.size());
                let recip_size = image.source_size().recip();
                let normalized_pos = image.area().top_left().times(recip_size);
                let normalized_size = image.area().size().times(recip_size);
                let get_vertex = |v: Vector| {
                    Vertex {
                        pos: trans * v,
                        tex_pos: normalized_pos + v.times(normalized_size),
                        col: self.color,
                        use_texture: true,
                    }
                };
                backend.add(
                    image.get_id(),
                    &[
                        get_vertex(Vector::zero()),
                        get_vertex(Vector::zero() + Vector::x()),
                        get_vertex(Vector::zero() + Vector::one()),
                        get_vertex(Vector::zero() + Vector::y()),
                    ],
                    &[0, 1, 2, 2, 3, 0],
                );
            }
            DrawPayload::Rectangle(ref rect) => {
                let points = &[Vector::zero(), rect.size().x_comp(), rect.size(), rect.size().y_comp()];
                apply_polygon(points, self.color, camera * Transform::translate(rect.top_left()) * self.transform, backend);
            }
            DrawPayload::Circle(ref circ) => {
                let mut points = [Vector::zero(); 24];
                let rotation = Transform::rotate(360f32 / points.len() as f32);
                let mut arrow = Vector::new(0f32, -circ.radius);
                for i in 0..points.len() {
                    points[i] = arrow;
                    arrow = rotation * arrow;
                }
                apply_polygon(&points, self.color, camera * Transform::translate(circ.center()) * self.transform, backend);
            }
        }
    }
}

fn apply_polygon(vertices: &[Vector], col: Color, trans: Transform, backend: &mut Backend) {
    let first_index = backend.num_vertices() as u32;
    for vertex in vertices {
        backend.add_vertex(&Vertex {
            pos: trans * vertex.clone(),
            tex_pos: Vector::zero(),
            col,
            use_texture: false
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

impl PartialEq for DrawPayload {
    fn eq(&self, other: &DrawPayload) -> bool {
        match (self, other) {
            (&DrawPayload::Image(ref a), &DrawPayload::Image(ref b)) => a.0.get_id() == b.0.get_id(),
            (&DrawPayload::Rectangle(_), &DrawPayload::Rectangle(_)) => true,
            (&DrawPayload::Circle(_), &DrawPayload::Circle(_)) => true,
            _ => false
        }
    }
}

impl Eq for DrawPayload {}

impl PartialOrd for DrawPayload {
    fn partial_cmp(&self, other: &DrawPayload) -> Option<Ordering> {
        Some(match (self, other) {
            (&DrawPayload::Image(ref a), &DrawPayload::Image(ref b)) => a.0.get_id().cmp(&b.0.get_id()),
            (&DrawPayload::Image(_), _) => Ordering::Greater,
            (_,  &DrawPayload::Image(_)) => Ordering::Less,
            _ => Ordering::Equal
        })
    }
}

impl Ord for DrawPayload {
    fn cmp(&self, other: &DrawPayload) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


impl PartialEq for DrawCall {
    fn eq(&self, other: &DrawCall) -> bool {
        self.z == other.z && self.item == other.item
    }
}

impl Eq for DrawCall {}

impl PartialOrd for DrawCall {
    fn partial_cmp(&self, other: &DrawCall) -> Option<Ordering> {
        match self.z.partial_cmp(&other.z) {
            None | Some(Ordering::Equal) => self.item.partial_cmp(&other.item),
            x => x
        }
    }
}

impl Ord for DrawCall {
    fn cmp(&self, other: &DrawCall) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
