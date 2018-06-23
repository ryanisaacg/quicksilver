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

impl Vertex {
    /// Create a new untextured GPU vertex
    pub fn new_untextured(pos: Vector, col: Color) -> Vertex {
        Vertex {
            pos,
            tex_pos: None,
            col
        }
    }

    /// Create a new textured GPU vertex
    pub fn new_textured(pos: Vector, tex_pos: Vector, col: Color) -> Vertex {
        Vertex {
            pos,
            tex_pos: Some(tex_pos),
            col
        }
    }
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

impl GpuTriangle {
    /// Create a new untextured GPU Triangle
    pub fn new_untextured(indices: [u32; 3], z: f32) -> GpuTriangle {
        GpuTriangle {
            z,
            indices,
            image: None
        }
    }

    /// Create a new textured GPU triangle
    pub fn new_textured(indices: [u32; 3], z: f32, image: Image) -> GpuTriangle {
        GpuTriangle {
            z,
            indices,
            image: Some(image)
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
        match self.z.partial_cmp(&other.z) {
            None | Some(Ordering::Equal) => 
                Some(match (&self.image, &other.image) {
                    (&Some(ref a), &Some(ref b)) => a.get_id().cmp(&b.get_id()),
                    (&Some(_), &None) => Ordering::Greater,
                    (&None, &Some(_)) => Ordering::Less,
                    (&None, &None) => Ordering::Equal,
                }),
            result => result
        }
    }
}

#[doc(hidden)]
impl Ord for GpuTriangle {
    fn cmp(&self, other: &GpuTriangle) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


