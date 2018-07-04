use geom::{Rectangle, Transform, Vector};

#[derive(Clone, Copy, Debug)]
///A view into the world, used as a camera and a viewport
pub struct View {
    pub(crate) normalize: Transform,
    pub(crate) opengl: Transform,
}

impl View {
    ///Create a new view that looks at a given area
    pub fn new(world: Rectangle) -> View { View::new_transformed(world, Transform::identity()) }

    ///Create a new view that looks at a given area with a transform
    ///
    ///The transform is relative to the center of the region
    pub fn new_transformed(world: Rectangle, transform: Transform) -> View {
        let normalize = Transform::scale(world.size().recip())
                        * Transform::translate(world.size() / 2)
                        * transform
                        * Transform::translate(-world.top_left() - world.size() / 2);
        let opengl = Transform::scale(Vector::new(2, -2))
                     * Transform::translate(-Vector::one() / 2)
                     * normalize;
        View { normalize, opengl }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opengl_projection() {
        let view = View::new(Rectangle::new_sized(50, 50));
        let world_bottom = Vector::y() * 50;
        assert_eq!(view.opengl * world_bottom, -Vector::one());
        let view = View::new(Rectangle::new(50, 50, 50, 50));
        let world_top = Vector::one() * 50;
        let expected = -Vector::x() + Vector::y();
        assert_eq!(view.opengl * world_top, expected);
    }

    #[test]
    fn projection() {
        let view = View::new(Rectangle::new_sized(50, 50));
        let screen_size = Vector::new(100, 100);
        let unproject = Transform::scale(screen_size) * view.normalize;
        let project = unproject.inverse();
        let screen_bottom = Vector::y() * 100;
        let world_bottom = Vector::y() * 50;
        assert_eq!(project * screen_bottom, world_bottom);
        assert_eq!(unproject * world_bottom, screen_bottom);
    }

    //    #[test]
    //    fn custom_transform() {
    //        let rect = Rectangle::new(-10, -10, 10, 10);
    //        let view = View::new_transformed(rect, Transform::rotate(-90f32));
    //        let unproject = Transform::scale(rect.size()) * view.normalize;
    //        let project = unproject.inverse();
    //        let point = Vector::x() * 5;
    //        let expected = Vector::y() * 5;
    //        assert_eq!(project * point, expected);
    //    }
}
