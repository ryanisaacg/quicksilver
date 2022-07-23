use crate::{Dimension, Position};
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;

#[derive(Clone)]
/// An input to a shader program stage
///
/// Attributes are composed of a name and a type, which are generated to form an OpenGL shader
/// attribute declaration. They indicate the input to the vertex and fragment shader steps.
pub struct Attribute {
    name: &'static str,
    value: AttributeType,
}

#[derive(Clone)]
/// The data type of a given attribute
pub enum AttributeType {
    /// A single, scalar f32 value
    Scalar,
    /// A vector value, ranging from 2 f32s to 4
    Vector(Dimension),
    /// A 2D array of f32 values, ranging from 2x2 to 4x4
    Matrix(Dimension, Dimension),
}

impl Position {
    #[cfg(target_arch = "wasm32")]
    fn glsl_string(self) -> &'static str {
        use Position::*;

        match self {
            Input => "attribute ",
            Output => "varying ",
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn glsl_string(self) -> &'static str {
        use Position::*;

        match self {
            Input => "in ",
            Output => "out ",
        }
    }
}

impl Attribute {
    pub fn new(name: &'static str, value: AttributeType) -> Attribute {
        Attribute { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn size(&self) -> i32 {
        use AttributeType::*;

        match self.value {
            Scalar => 1,
            Vector(n) => n as i32,
            Matrix(m, n) => (m as i32) * (n as i32),
        }
    }

    pub(crate) fn as_glsl(&self, _is_vertex: bool, pos: Position, shader: &mut String) {
        use AttributeType::*;

        #[cfg(target_arch = "wasm32")]
        let pos = if _is_vertex { pos } else { Position::Output };

        shader.push_str(pos.glsl_string());
        let gl_type = match self.value {
            Scalar => "float ".to_owned(),
            Vector(n) => format!("vec{} ", n as i32),
            Matrix(m, n) => format!("mat{}x{} ", m as i32, n as i32),
        };
        shader.push_str(&gl_type);
        shader.push_str(self.name());
        shader.push(';');
    }
}
