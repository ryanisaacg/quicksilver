use geom::{Rectangle, Transform, Vector};

#[derive(Clone)]
/// Allows the creation of a viewport
///
/// Can only be created from an instance of a Window
pub struct ViewportBuilder {
    pub(crate) screen_size: Vector,
    pub(crate) transform: Transform
}

impl ViewportBuilder {
    ///Transform a viewport and by extension the points it projects
    pub fn transform(&self, transform: Transform) -> ViewportBuilder {
        ViewportBuilder {
            screen_size: self.screen_size,
            transform: transform * self.transform
        }
    }

    ///Create the viewport
    pub fn build(&self, world: Rectangle) -> Viewport {
        let unproject = Transform::scale(self.screen_size.times(world.size().recip())) *
            Transform::translate(-world.top_left()) * self.transform;
        let project = unproject.inverse();
        Viewport { project, unproject }
    }
}


///Allows projection back and forth from the world to the screen
pub struct Viewport {
    project: Transform,
    unproject: Transform
}

impl Viewport {
    ///A matrix to project from the screen to the world
    pub fn project(&self) -> Transform {
        self.project
    }

    ///A matrix to unproject from the world to the screen
    pub fn unproject(&self) -> Transform {
        self.unproject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection() {
        let viewport = ViewportBuilder {
            screen_size: Vector::newi(100, 100),
            transform: Transform::identity()
        }.build(Rectangle::newi_sized(50, 50));
        let screen_bottom = Vector::y() * 100;
        let world_bottom = Vector::y() * 50;
        assert_eq!(viewport.project() * screen_bottom, world_bottom);
        assert_eq!(viewport.unproject() * world_bottom, screen_bottom);
    }
    
    #[test]
    fn custom_transform() {
        let rect = Rectangle::newi_sized(10, 10);
        let viewport = ViewportBuilder { screen_size: rect.size(), transform: Transform::identity() }
                        .transform(Transform::rotate(-90f32))
                        .build(rect);
        let point = Vector::x() * 5;
        let expected = Vector::y() * 5;
        assert_eq!(viewport.project() * point, expected);
    }
}
