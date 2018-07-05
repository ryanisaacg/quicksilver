use graphics::Vertex;
use graphics::GpuTriangle;
use geom::Transform;
use geom::Vector;
use graphics::Color;
use graphics::Image;
use graphics::Window;
use graphics::Drawable;
use geom::Scalar;
use geom::Rectangle;
use geom::Circle;
use geom::Shape;
use geom::Positioned;

/// A single drawable item, with a transform, a blend color, and a depth
#[derive(Clone, Debug)]
pub struct Sprite {
    vertices: Vec<Vertex>,
    triangles: Vec<GpuTriangle>,
    offset: Vector,
    transform: Transform,
}

impl Sprite {
    /// Create a sprite from any `Drawable`
    pub fn new(drawable: impl Drawable) -> Sprite {
        Sprite {
            vertices: drawable.get_vertices(),
            triangles: drawable.get_triangles(),
            offset: Vector::zero(),
            transform: Transform::identity(),
        }
    }
    /// Create a sprite with an image
    pub fn image(image: &Image, position: Vector) -> Sprite {
        image.to_sprite().with_position(position)
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
        Rectangle::newv(position, Vector::one()).to_sprite()
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

        let transform = Transform::translate(Vector::new(trans_x, trans_y)) * Transform::rotate(dy.atan2(dx).to_degrees());

        rect.to_sprite().with_transform(transform)
    }

    /// Create a sprite with a rectangle
    pub fn rectangle(rectangle: Rectangle) -> Sprite {
        rectangle.to_sprite()
    }

    /// Create a sprite with a circle
    pub fn circle(circle: Circle) -> Sprite {
        circle.to_sprite()
    }

    /// Change the position of a sprite
    pub fn with_position(self, position: Vector) -> Sprite {
        let mut vertices = self.vertices.clone();
        for vertex in vertices.iter_mut() {
            vertex.pos = position;
        }
        Sprite {
            vertices,
            ..self
        }
    }

    /// Change the color of a sprite
    pub fn with_color(self, color: Color) -> Sprite {
        let mut vertices = self.vertices.clone();
        for vertex in vertices.iter_mut() {
            vertex.col = color;
        }
        Sprite {
            vertices,
            ..self
        }
    }

    /// Change the transform of a sprite
    pub fn with_transform(self, transform: Transform) -> Sprite {
        Sprite {
            transform,
            ..self
        }
    }

    /// Change the depth of a sprite
    pub fn with_z<T: Scalar>(self, z: T) -> Sprite {
        let mut triangles = self.triangles.clone();
        for triangle in triangles.iter_mut() {
            triangle.z = z.float();
        }
        Sprite {
            triangles,
            ..self
        }
    }

    ///Draws the sprite to the given window
    pub fn draw(&self, window: &mut Window) {
        let mut vertices = self.vertices.clone();
        for vertex in vertices.iter_mut() {
            vertex.pos = self.transform * vertex.pos;
        }
        window.add_vertices(self.vertices.iter().cloned(), self.triangles.iter().cloned());
    }
}