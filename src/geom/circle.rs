#[cfg(feature="ncollide2d")] use ncollide2d::shape::Ball;
use geom::{about_equal, Positioned, Rectangle, Scalar, Transform, Vector};
use graphics::{DrawAttributes, Drawable, GpuTriangle, Vertex, Window};
use std::{
    cmp::{Eq, PartialEq},
    iter
};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
///A circle with a center and a radius
pub struct Circle {
    /// The x coordinate of the center
    pub x: f32,
    /// The y coordinate of the center
    pub y: f32,
    /// The radius of the circle
    pub radius: f32,
}

impl Circle {
    /// Create a new circle with the given dimensions
    pub fn new<T: Scalar>(x: T, y: T, radius: T) -> Circle {
        Circle {
            x: x.float(),
            y: y.float(),
            radius: radius.float(),
        }
    }

    /// Create a circle with the center as a vector
    pub fn newv<T: Scalar>(center: Vector, radius: T) -> Circle {
        Circle {
            x: center.x,
            y: center.y,
            radius: radius.float()
        }
    }

    ///Construct a circle from a center and a Ball
    #[cfg(feature="ncollide2d")]
    pub fn from_ball(center: Vector, ball: Ball<f32>) -> Circle {
        Circle::newv(center, ball.radius())
    }

    ///Convert the circle into an ncollide Ball
    #[cfg(feature="ncollide2d")]
    pub fn into_ball(self) -> Ball<f32> {
        Ball::new(self.radius)
    }

    /// Check to see if a circle contains a point
    pub fn contains(self, v: Vector) -> bool {
        (v - self.center()).len2() < self.radius.powi(2)
    }

    ///Check if a circle overlaps a rectangle
    pub fn overlaps_rect(self, r: Rectangle) -> bool {
        r.overlaps_circ(self)
    }

    ///Check if two circles overlap
    pub fn overlaps_circ(self, c: Circle) -> bool {
        (self.center() - c.center()).len2() < (self.radius + c.radius).powi(2)
    }

    ///Translate a circle by a given vector
    pub fn translate(self, v: Vector) -> Circle {
        Circle::new(self.x + v.x, self.y + v.y, self.radius)
    }

    ///Move a circle so it is entirely contained within a Rectangle
    pub fn constrain(self, outer: Rectangle) -> Circle {
        Circle::newv(Rectangle::new(self.x - self.radius, self.y - self.radius, self.radius * 2.0, self.radius * 2.0).constrain(outer).center(), self.radius)
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        about_equal(self.x, other.x) && about_equal(self.y, other.y) && about_equal(self.radius, other.radius)
    }
}

impl Eq for Circle {}

impl Positioned for Circle {
    fn center(&self) -> Vector {
        Vector::new(self.x, self.y)
    }

    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.x - self.radius, self.y - self.radius, self.radius * 2.0, self.radius * 2.0)
    }
}

// Until there's serious compile-time calculations in Rust,
// it's best to just pre-write the points on a rasterized circle
const CIRCLE_POINTS: [Vector; 24] = [
    Vector { x: 1.0, y: 0.0 },
    Vector { x: -0.7596879128588213, y: 0.6502878401571168 },
    Vector { x: 0.15425144988758405, y: -0.9880316240928618 },
    Vector { x: 0.5253219888177297, y: 0.8509035245341184 },
    Vector { x: -0.9524129804151563, y: -0.3048106211022167 },
    Vector { x: 0.9217512697247493, y: -0.38778163540943045 },
    Vector { x: -0.4480736161291701, y: 0.8939966636005579 },
    Vector { x: -0.24095904923620143, y: -0.9705352835374847 },
    Vector { x: 0.8141809705265618, y: 0.5806111842123143 },
    Vector { x: -0.9960878351411849, y: 0.08836868610400143 },
    Vector { x: 0.6992508064783751, y: -0.7148764296291646 },
    Vector { x: -0.06633693633562374, y: 0.9977972794498907 },
    Vector { x: -0.5984600690578581, y: -0.8011526357338304 },
    Vector { x: 0.9756226979194443, y: 0.21945466799406363 },
    Vector { x: -0.8838774731823718, y: 0.46771851834275896 },
    Vector { x: 0.36731936773024515, y: -0.9300948780045254 },
    Vector { x: 0.32578130553514806, y: 0.9454451549211168 },
    Vector { x: -0.8623036078310824, y: -0.5063916349244909 },
    Vector { x: 0.9843819506325049, y: -0.1760459464712114 },
    Vector { x: -0.6333425312327234, y: 0.7738715902084317 },
    Vector { x: -0.022096619278683942, y: -0.9997558399011495 },
    Vector { x: 0.6669156003948422, y: 0.7451332645574127 },
    Vector { x: -0.9911988217552068, y: -0.13238162920545193 },
    Vector { x: 0.8390879278598296, y: -0.5439958173735323 }
];

impl Drawable for Circle {
    fn draw(&self, window: &mut Window, params: DrawAttributes) {
        let transform = Transform::translate(self.center())
            * params.transform
            * Transform::scale(Vector::one() * self.radius);
        let vertices = CIRCLE_POINTS
            .iter()
            .map(|point| Vertex::new_untextured(transform * point.clone(), params.color));
        let indices = iter::repeat(params.z)
            .take(CIRCLE_POINTS.len() - 1)
            .enumerate()
            .map(|(index, z)| GpuTriangle::new_untextured([0, index as u32, index as u32 + 1], z));
        window.add_vertices(vertices, indices);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let circ = Circle::new(0f32, 1f32, 2f32);
        assert_eq!(circ.x, 0f32);
        assert_eq!(circ.y, 1f32);
        assert_eq!(circ.radius, 2f32);
    }

    #[test]
    fn contains() {
        let circ = Circle::new(0, 0, 10);
        let vec1 = Vector::new(0, 0);
        let vec2 = Vector::new(11, 11);
        assert!(circ.contains(vec1));
        assert!(!circ.contains(vec2));
    }

    #[test]
    fn overlap() {
        let a = Circle::new(0, 0, 16);
        let b = Circle::new(5, 5, 4);
        let c = Circle::new(50, 50, 5);
        let d = Rectangle::new(10, 10, 10, 10);
        assert!(a.overlaps_circ(b));
        assert!(!a.overlaps_circ(c));
        assert!(a.overlaps_rect(d));
        assert!(!c.overlaps_rect(d));
    }

    #[test]
    fn rect_overlap() {
        let circ = Circle::new(0, 0, 5);
        let rec1 = Rectangle::new_sized(2, 2);
        let rec2 = Rectangle::new(5, 5, 4, 4);
        assert!(circ.overlaps_rect(rec1));
        assert!(rec1.overlaps_circ(circ));
        assert!(!circ.overlaps_rect(rec2));
        assert!(!rec2.overlaps_circ(circ));
    }

    #[test]
    fn translate() {
        let circ = Circle::new(0, 0, 16);
        let translate = Vector::new(4, 4);
        assert_eq!(circ.center() + translate, circ.translate(translate).center());
    }

}