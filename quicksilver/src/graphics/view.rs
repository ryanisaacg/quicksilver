use crate::geom::{Vector, Rectangle, Transform};

#[derive(Clone, Copy, Debug)]
///A view into the world, used as a camera and a viewport
pub struct View {
    pub(crate) normalize: Transform,
    pub(crate) opengl: Transform
}

impl View {
    ///Create a new view that looks at a given area
    pub fn new(world: Rectangle) -> View {
        View::new_transformed(world, Transform::IDENTITY)
    }
   
    ///Create a new view that looks at a given area with a transform
    ///
    ///The transform is relative to the center of the region
    pub fn new_transformed(world: Rectangle, transform: Transform) -> View {
        let normalize = Transform::scale(world.size().recip())
                * Transform::translate(world.size() / 2)
                * transform
                * Transform::translate(-world.top_left() - world.size() / 2);
        let opengl = Transform::scale(Vector::new(2, -2))
            * Transform::translate(-Vector::ONE / 2)
            * normalize;
        View { normalize, opengl }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opengl_projection() {
        let view = View::new(Rectangle::new_sized((50, 50)));
        let world_bottom = Vector::Y * 50;
        assert_eq!(view.opengl * world_bottom, -Vector::ONE);
        let view = View::new(Rectangle::new((50, 50), (50, 50)));
        let world_top = Vector::ONE * 50;
        let expected = -Vector::X + Vector::Y;
        assert_eq!(view.opengl * world_top, expected);
    }
    
    #[test]
    fn projection() {
        let view = View::new(Rectangle::new_sized((50, 50)));
        let screen_size = Vector::new(100, 100);
        let unproject = Transform::scale(screen_size) * view.normalize;
        let project = unproject.inverse();
        let screen_bottom = Vector::Y * 100;
        let world_bottom = Vector::Y * 50;
        assert_eq!(project * screen_bottom, world_bottom);
        assert_eq!(unproject * world_bottom, screen_bottom);
    }
    
//    #[test]
//    fn custom_transform() {
//        let rect = Rectangle::new(-10, -10, 10, 10);
//        let view = View::new_transformed(rect, Transform::rotate(-90f32));
//        let unproject = Transform::scale(rect.size()) * view.normalize;
//        let project = unproject.inverse();
//        let point = Vector::X * 5;
//        let expected = Vector::Y * 5;
//        assert_eq!(project * point, expected);
//    }
}
