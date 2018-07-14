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
    pub fn new(x: impl Scalar, y: impl Scalar, radius: impl Scalar) -> Circle {
        Circle {
            x: x.float(),
            y: y.float(),
            radius: radius.float(),
        }
    }

    /// Create a circle with the center as a vector
    pub fn newv<V: Into<Vector>>(center: V, radius: impl Scalar) -> Circle {
        let center = center.into();
        Circle {
            x: center.x,
            y: center.y,
            radius: radius.float()
        }
    }

    ///Construct a circle from a center and a Ball
    #[cfg(feature="ncollide2d")]
    pub fn from_ball<V: Into<Vector>>(center: V, ball: Ball<f32>) -> Circle {
        let center = center.into();
        Circle::newv(center, ball.radius())
    }

    ///Convert the circle into an ncollide Ball
    #[cfg(feature="ncollide2d")]
    pub fn into_ball(self) -> Ball<f32> {
        Ball::new(self.radius)
    }

    /// Check to see if a circle contains a point
    pub fn contains<V: Into<Vector>>(self, v: V) -> bool {
        let v = v.into();
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
    pub fn translate<V: Into<Vector>>(self, v: V) -> Circle {
        let v = v.into();
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


impl Drawable for Circle {
    fn draw(&self, window: &mut Window, params: DrawAttributes) {
        let transform = Transform::translate(self.center())
            * params.transform
            * Transform::scale(Vector::ONE * self.radius);
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

// Until there's serious compile-time calculations in Rust,
// it's best to just pre-write the points on a rasterized circle

// Python script to generate the array:
/*
import math
from math import pi


def points_on_circumference(center=(0, 0), r=50, n=100):
    n -= 1
    return [
        (
            center[0]+(math.cos(2 * pi / n * x) * r),  # x
            center[1] + (math.sin(2 * pi / n * x) * r)  # y

        ) for x in range(0, n + 1)]


number_of_points = 64
points = points_on_circumference(center=(0,0),r=1, n=number_of_points)

print("const CIRCLE_POINTS: [Vector; %i] = [" % number_of_points)

for point in points:
    print("    Vector { x: %s, y: %s }," % (point[0], point[1]))

print("];")
*/

const CIRCLE_POINTS: [Vector; 64] = [
    Vector { x: 1.0, y: 0.0 },
    Vector { x: 0.9950307753654014, y: 0.09956784659581666 },
    Vector { x: 0.9801724878485438, y: 0.19814614319939758 },
    Vector { x: 0.9555728057861407, y: 0.2947551744109042 },
    Vector { x: 0.9214762118704076, y: 0.38843479627469474 },
    Vector { x: 0.8782215733702285, y: 0.47825397862131824 },
    Vector { x: 0.8262387743159949, y: 0.5633200580636221 },
    Vector { x: 0.766044443118978, y: 0.6427876096865394 },
    Vector { x: 0.6982368180860729, y: 0.7158668492597184 },
    Vector { x: 0.6234898018587336, y: 0.7818314824680298 },
    Vector { x: 0.5425462638657594, y: 0.8400259231507714 },
    Vector { x: 0.4562106573531629, y: 0.8898718088114687 },
    Vector { x: 0.365341024366395, y: 0.9308737486442042 },
    Vector { x: 0.27084046814300516, y: 0.962624246950012 },
    Vector { x: 0.17364817766693022, y: 0.9848077530122081 },
    Vector { x: 0.07473009358642417, y: 0.9972037971811801 },
    Vector { x: -0.024930691738072913, y: 0.9996891820008162 },
    Vector { x: -0.12434370464748516, y: 0.9922392066001721 },
    Vector { x: -0.22252093395631434, y: 0.9749279121818236 },
    Vector { x: -0.31848665025168454, y: 0.9479273461671317 },
    Vector { x: -0.41128710313061156, y: 0.9115058523116731 },
    Vector { x: -0.5000000000000002, y: 0.8660254037844385 },
    Vector { x: -0.58374367223479, y: 0.8119380057158564 },
    Vector { x: -0.6616858375968595, y: 0.7497812029677341 },
    Vector { x: -0.7330518718298263, y: 0.6801727377709194 },
    Vector { x: -0.7971325072229225, y: 0.6038044103254774 },
    Vector { x: -0.8532908816321556, y: 0.5214352033794981 },
    Vector { x: -0.900968867902419, y: 0.43388373911755823 },
    Vector { x: -0.9396926207859084, y: 0.3420201433256685 },
    Vector { x: -0.969077286229078, y: 0.24675739769029342 },
    Vector { x: -0.9888308262251285, y: 0.14904226617617428 },
    Vector { x: -0.9987569212189223, y: 0.04984588566069704 },
    Vector { x: -0.9987569212189223, y: -0.04984588566069723 },
    Vector { x: -0.9888308262251285, y: -0.14904226617617447 },
    Vector { x: -0.969077286229078, y: -0.24675739769029362 },
    Vector { x: -0.9396926207859084, y: -0.34202014332566866 },
    Vector { x: -0.9009688679024191, y: -0.433883739117558 },
    Vector { x: -0.8532908816321555, y: -0.5214352033794983 },
    Vector { x: -0.7971325072229224, y: -0.6038044103254775 },
    Vector { x: -0.7330518718298262, y: -0.6801727377709195 },
    Vector { x: -0.6616858375968594, y: -0.7497812029677342 },
    Vector { x: -0.5837436722347898, y: -0.8119380057158565 },
    Vector { x: -0.4999999999999996, y: -0.8660254037844388 },
    Vector { x: -0.4112871031306116, y: -0.9115058523116731 },
    Vector { x: -0.3184866502516841, y: -0.9479273461671318 },
    Vector { x: -0.2225209339563146, y: -0.9749279121818236 },
    Vector { x: -0.12434370464748495, y: -0.9922392066001721 },
    Vector { x: -0.024930691738073156, y: -0.9996891820008162 },
    Vector { x: 0.07473009358642436, y: -0.9972037971811801 },
    Vector { x: 0.17364817766693083, y: -0.984807753012208 },
    Vector { x: 0.2708404681430051, y: -0.962624246950012 },
    Vector { x: 0.3653410243663954, y: -0.9308737486442041 },
    Vector { x: 0.45621065735316285, y: -0.8898718088114687 },
    Vector { x: 0.5425462638657597, y: -0.8400259231507713 },
    Vector { x: 0.6234898018587334, y: -0.7818314824680299 },
    Vector { x: 0.698236818086073, y: -0.7158668492597183 },
    Vector { x: 0.7660444431189785, y: -0.6427876096865389 },
    Vector { x: 0.8262387743159949, y: -0.563320058063622 },
    Vector { x: 0.8782215733702288, y: -0.4782539786213178 },
    Vector { x: 0.9214762118704076, y: -0.38843479627469474 },
    Vector { x: 0.9555728057861408, y: -0.2947551744109039 },
    Vector { x: 0.9801724878485438, y: -0.19814614319939772 },
    Vector { x: 0.9950307753654014, y: -0.09956784659581641 },
    Vector { x: 1.0, y: 0.0 },
];

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
        let vec1 = (0, 0);
        let vec2 = (11, 11);
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