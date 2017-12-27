use geom::{Rectangle, Transform, Vector};

#[derive(Clone, Copy)]
///A Camera that projects drawn items onto the screen
pub struct Camera {
    opengl: Transform,
}

impl Camera {
    ///Create a camera that maps a given area to the screen
    pub fn new(world: Rectangle) -> Camera {
        Camera::new_transformed(world, Transform::identity())
    }

    ///Create a camera that maps a given area to the screen with the given transformation
    pub fn new_transformed(world: Rectangle, transform: Transform) -> Camera {
        Camera {
            opengl: Transform::scale(Vector::x() - Vector::y())
                * Transform::translate(-Vector::one())
                * Transform::scale(world.size().recip() * 2)
                * Transform::translate(world.size() / 2)
                * transform
                * Transform::translate(-world.top_left() - world.size() / 2)
        }
    }

    pub(crate) fn transform(&self) -> Transform {
        self.opengl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection() {
        let camera = Camera::new(Rectangle::newi_sized(50, 50));
        let world_bottom = Vector::y() * 50;
        assert_eq!(camera.transform() * world_bottom, -Vector::one());
    }

    #[test]
    fn opengl_projection() {
        let camera = Camera::new(Rectangle::newi(50, 50, 50, 50));
        let world_top = Vector::one() * 50;
        let expected = -Vector::x() + Vector::y();
        assert_eq!(camera.transform() * world_top, expected);
    }
}
