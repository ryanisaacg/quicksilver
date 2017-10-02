use geom::{Rectangle, Transform, Vector};

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
            * Transform::scale(Vector::new(window.width / world.width, window.height / world.height))
            * Transform::translate(-world.top_left())
            * transform;
        Camera {
            unproject: unproject,
            project: unproject.inverse(),
            opengl: Transform::scale(Vector::new(1f32, -1f32))
                * Transform::translate(Vector::new(-1f32, -1f32))
                * Transform::scale(world.size().recip() * 2f32)
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
            Rectangle::new_sized(100f32, 100f32), 
            Rectangle::new_sized(50f32, 50f32));
        let screen_bottom = Vector::new(0f32, 100f32);
        let world_bottom = Vector::new(0f32, 50f32);
        assert_eq!(camera.project * screen_bottom, world_bottom);
        assert_eq!(camera.unproject * world_bottom, screen_bottom);
        assert_eq!(camera.opengl * world_bottom, Vector::new(-1f32, -1f32));
    }

    #[test]
    fn opengl_projection() {
        let camera = Camera::new(
            Rectangle::new_sized(100f32, 100f32), 
            Rectangle::new(50f32, 50f32, 50f32, 50f32));
        let world_top = Vector::new(50f32, 50f32);
        let expected = Vector::new(-1f32, 1f32);
        assert_eq!(camera.opengl * world_top, expected);
    }

    #[test]
    fn custom_transform() {
        let rect = Rectangle::new_sized(10f32, 10f32);
        let camera = Camera::new_transformed(rect, rect, Transform::rotate(-90f32));
        let point = Vector::new(5f32, 0f32);
        let expected = Vector::new(0f32, 5f32);
        assert_eq!(camera.project * point, expected);
    }
}

