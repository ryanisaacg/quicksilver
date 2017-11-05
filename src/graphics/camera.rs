use geom::{Rectangle, Transform, Vector};

#[derive(Clone, Copy)]
pub struct Camera {
    opengl: Transform,
}

impl Camera {
    pub fn new(world: Rectangle) -> Camera {
        Camera::new_transformed(world, Transform::identity())
    }

    pub fn new_transformed(world: Rectangle, transform: Transform) -> Camera {
        Camera {
            opengl: Transform::scale(Vector::x() - Vector::y()) *
                Transform::translate(-Vector::one()) *
                Transform::scale(world.size().recip() * 2) *
                Transform::translate(-world.top_left()) * transform,
        }
    }

    pub fn transform(&self) -> Transform {
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
