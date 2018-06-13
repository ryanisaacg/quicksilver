use geom::Vector;
use graphics::{Color, Image};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
/// A vertex for drawing items to the GPU
pub struct Vertex {
    /// The position of the vertex in space
    pub pos: Vector,
    /// If there is a texture attached to this vertex, where to get the texture data from
    ///
    /// It is normalized from 0 to 1
    pub tex_pos: Option<Vector>,
    /// The color to blend this vertex with
    pub col: Color,
}

#[derive(Clone)]
/// A triangle to draw to the GPU
pub struct GpuTriangle {
    /// The plane the triangle falls on
    pub z: f32,
    /// The indexes in the vertex list that the GpuTriangle uses
    pub indices: [u32; 3],
    /// The (optional) image used by the GpuTriangle
    ///
    /// All of the vertices used by the triangle should agree on whether it uses an image,
    /// it is up to you to maintain this
    pub image: Option<Image>
}

#[doc(hidden)]
impl PartialEq for GpuTriangle {
    fn eq(&self, other: &GpuTriangle) -> bool {
        match (&self.image, &other.image) {
            (&Some(ref a), &Some(ref b)) => a.get_id() == b.get_id(),
            (&None, &None) => true,
            _ => false
        }
    }
}

#[doc(hidden)]
impl Eq for GpuTriangle {}

#[doc(hidden)]
impl PartialOrd for GpuTriangle {
    fn partial_cmp(&self, other: &GpuTriangle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[doc(hidden)]
impl Ord for GpuTriangle {
    fn cmp(&self, other: &GpuTriangle) -> Ordering {
        match self.z.partial_cmp(&other.z) {
            None | Some(Ordering::Equal) => 
                match (&self.image, &other.image) {
                    (&Some(ref a), &Some(ref b)) => a.get_id().cmp(&b.get_id()),
                    (&Some(_), &None) => Ordering::Greater,
                    (&None, &Some(_)) => Ordering::Less,
                    (&None, &None) => Ordering::Equal,
                },
            Some(result) => result
        }
    }
}


