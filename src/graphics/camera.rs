use geom::{Rectangle, Transform, Vector};

#[derive(Clone, Copy)]
pub struct Camera {
    pub project: Transform,
    pub unproject: Transform,
    pub opengl: Transform
}

impl Camera {
    pub fn new(window: Rectangle, world: Rectangle) -> Camera {
        Camera::new_transformed(window, world, Transform::identity())
    }

    pub fn new_transformed(window: Rectangle, world: Rectangle, transform: Transform) -> Camera {
        let unproject = Transform::translate(window.top_left())
            * Transform::scale(window.size().times(world.size().recip()))
            * Transform::translate(-world.top_left())
            * transform;
        Camera {
            unproject: unproject,
            project: unproject.inverse(),
            opengl: Transform::scale(Vector::x() - Vector::y())
                * Transform::translate(-Vector::one())
                * Transform::scale(world.size().recip() * 2)
                * Transform::translate(-world.top_left())
                * transform
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection() {
        let camera = Camera::new(
            Rectangle::newi_sized(100, 100), 
            Rectangle::newi_sized(50, 50));
        let screen_bottom = Vector::y() * 100;
        let world_bottom = Vector::y() * 50;
        assert_eq!(camera.project * screen_bottom, world_bottom);
        assert_eq!(camera.unproject * world_bottom, screen_bottom);
        assert_eq!(camera.opengl * world_bottom, -Vector::one());
    }

    #[test]
    fn opengl_projection() {
        let camera = Camera::new(
            Rectangle::newi_sized(100, 100), 
            Rectangle::newi(50, 50, 50, 50));
        let world_top = Vector::one() * 50;
        let expected = -Vector::x() + Vector::y();
        assert_eq!(camera.opengl * world_top, expected);
    }

    #[test]
    fn custom_transform() {
        let rect = Rectangle::newi_sized(10, 10);
        let camera = Camera::new_transformed(rect, rect, Transform::rotate(-90f32));
        let point = Vector::x() * 5;
        let expected = Vector::y() * 5;
        assert_eq!(camera.project * point, expected);
    }
}

