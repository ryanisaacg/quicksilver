use geom::{Rectangle, Transform, Vector};

#[derive(Clone)]
pub struct ViewportBuilder {
    screen_size: Vector,
    transform: Transform
}

impl ViewportBuilder {
    pub(crate) fn new(screen_size: Vector) -> ViewportBuilder {
        ViewportBuilder { 
            screen_size,
            transform: Transform::identity()
        }
    }

    pub fn transform(&self, transform: Transform) -> ViewportBuilder {
        ViewportBuilder {
            screen_size: self.screen_size,
            transform: transform * self.transform
        }
    }

    pub fn build(&self, world: Rectangle) -> Viewport {
        let unproject = Transform::scale(self.screen_size.times(world.size().recip())) *
            Transform::translate(-world.top_left()) * self.transform;
        let project = unproject.inverse();
        Viewport { project, unproject }
    }
}

pub struct Viewport {
    project: Transform,
    unproject: Transform
}

impl Viewport {
    pub fn project(&self) -> Transform {
        self.project
    }

    pub fn unproject(&self) -> Transform {
        self.unproject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection() {
        let viewport = ViewportBuilder::new(Vector::newi(100, 100))
                        .build(Rectangle::newi_sized(50, 50));
        let screen_bottom = Vector::y() * 100;
        let world_bottom = Vector::y() * 50;
        assert_eq!(viewport.project() * screen_bottom, world_bottom);
        assert_eq!(viewport.unproject() * world_bottom, screen_bottom);
    }
    
    #[test]
    fn custom_transform() {
        let rect = Rectangle::newi_sized(10, 10);
        let viewport = ViewportBuilder::new(rect.size())
                        .transform(Transform::rotate(-90f32))
                        .build(rect);
        let point = Vector::x() * 5;
        let expected = Vector::y() * 5;
        assert_eq!(viewport.project() * point, expected);
    }
}
