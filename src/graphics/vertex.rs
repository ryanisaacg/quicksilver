use mint::{ColumnMatrix3, Vector2};
use crate::graphics::{Color, Image};

#[derive(Clone, Copy, Debug, PartialEq)]
/// A vertex for drawing items to the GPU
pub struct Vertex {
    /// The position of the vertex in space
    pub pos: Vector2<f32>,
    /// If there is a texture attached to this vertex, where to get the texture data from
    ///
    /// It is normalized from 0 to 1
    pub uv: Option<Vector2<f32>>,
    /// The color to blend this vertex with
    pub color: Color,
}

pub struct DrawGroup {
    pub elements: Vec<Element>,
    pub image: Option<Image>,
    pub transform: ColumnMatrix3<f32>,
}

#[derive(Clone)]
pub enum Element {
    Point(u32),
    Line([u32; 2]),
    Triangle([u32; 3]),
}

/*
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

impl GpuTriangle {
    /// Create a new untextured GPU Triangle
    pub fn new(offset: u32, indices: [u32; 3], z: impl Scalar, bkg: Background) -> GpuTriangle {
        GpuTriangle {
            z: z.float(),
            indices: [indices[0] + offset, indices[1] + offset, indices[2] + offset],
            image: bkg.image().cloned()
        }
    }
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

*/
