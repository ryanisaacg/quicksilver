use super::{Rectangle, Vector, Shape};

#[derive(Clone)]
pub struct Tile<T: Clone> {
    pub value: Option<T>,
    pub empty: bool,
}

impl<T: Clone> Tile<T> {
    pub fn solid(value: Option<T>) -> Tile<T> {
        Tile {
            value: value,
            empty: false,
        }
    }

    pub fn empty(value: Option<T>) -> Tile<T> {
        Tile {
            value: value,
            empty: true,
        }
    }
}


pub struct Tilemap<T: Clone> {
    data: Vec<Tile<T>>,
    width: f32,
    height: f32,
    tile_width: f32,
    tile_height: f32,
}

impl<T: Clone> Tilemap<T> {
    pub fn new(map_width: f32, map_height: f32, tile_width: f32, tile_height: f32) -> Tilemap<T> {
        Tilemap::with_data(vec![Tile::empty(None);(map_width / tile_width * map_height / tile_height) as usize],
            map_width, map_height, tile_width, tile_height)
    }

    pub fn with_data(data: Vec<Tile<T>>, width: f32, height: f32, tile_width: f32, tile_height: f32) -> Tilemap<T> {
        Tilemap { data, width, height, tile_width, tile_height }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn size(&self) -> Vector {
        Vector::new(self.width, self.height)
    }

    pub fn region(&self) -> Rectangle {
        Rectangle::new_sized(self.width, self.height)
    }

    pub fn tile_width(&self) -> f32 {
        self.tile_width
    }

    pub fn tile_height(&self) -> f32 {
        self.tile_height
    }

    pub fn tile_size(&self) -> Vector {
        Vector::new(self.tile_width, self.tile_height)
    }

    pub fn valid(&self, index: Vector) -> bool {
        index.x >= 0f32 && index.y >= 0f32 && index.x < self.width && index.y < self.height
    }

    pub fn shape_valid(&self, shape: Shape) -> bool {
        let bbox = shape.bounding_box();
        self.valid(bbox.top_left()) && self.valid(bbox.top_left() + bbox.size())
    }

    fn array_index(&self, index: Vector) -> usize {
        ((index.x / self.tile_width).trunc() * (self.height / self.tile_height).trunc() +
             (index.y / self.tile_height).trunc()) as usize
    }

    pub fn get(&self, index: Vector) -> Option<&Tile<T>> {
        if self.valid(index) {
            Some(&self.data[self.array_index(index)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: Vector) -> Option<&mut Tile<T>> {
        if self.valid(index) {
            let index = self.array_index(index);
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: Vector, value: Tile<T>) {
        match self.get_mut(index) {
            Some(tile) => *tile = value,
            None => (),
        }
    }

    pub fn point_empty(&self, index: Vector) -> bool {
        match self.get(index) {
            Some(tile) => tile.empty,
            None => false,
        }
    }
    
    pub fn shape_empty(&self, shape: Shape) -> bool {
        let bounds = shape.bounding_box(); 
        match shape {
            //Rectangles and vectors are perfectly represented by their bounds
            Shape::Vect(_) => self.point_empty(shape.center()),
            Shape::Rect(_) | Shape::Circ(_) | Shape::Line(_) => {
                let x_start = (self.align_left(bounds.x) / self.tile_width) as i32;
                let y_start = (self.align_top(bounds.y) / self.tile_height) as i32;
                let x_end = (self.align_right(bounds.x + bounds.width) / self.tile_width) as i32;
                let y_end = (self.align_right(bounds.y + bounds.height) / self.tile_height) as i32;
                for x in x_start..x_end {
                    for y in y_start..y_end {
                        let point = Vector::newi(x, y).times(self.tile_size());
                        if !self.point_empty(point) && shape.overlaps_rect(Rectangle::newv(point, self.tile_size())) {
                            return false;
                        }
                    }
                }
                true
            }
        }
    }

    pub fn align_left(&self, x: f32) -> f32 {
        (x / self.tile_width).floor() * self.tile_width
    }

    pub fn align_right(&self, x: f32) -> f32 {
        (x / self.tile_width).ceil() * self.tile_width
    }

    pub fn align_top(&self, y: f32) -> f32 {
        (y / self.tile_height).floor() * self.tile_height
    }

    pub fn align_bottom(&self, y: f32) -> f32 {
        (y / self.tile_height).ceil() * self.tile_height
    }

    pub fn move_until_contact(&self, bounds: Shape, speed: Vector) -> (Shape, Vector) {
        if !self.shape_empty(bounds) {
            (bounds, Vector::zero())
        } else {
            //  If it is less than the total speed, mod the total speed by the tile size
            // Find how far can be moved beyond the last tile
            //  If it less than the remainder of the speed, set the total speed to zero
            let mut speed = speed;
            let mut bounds = bounds;
            let mut i = 1;
            // Find the largest number of tiles that can be moved
            let chunk = Vector::x() * speed.x.signum() * self.tile_width;
            while (chunk * (i + 1)).x.abs() < speed.x.abs() &&
                self.shape_empty(bounds.translate(chunk * (i + 1)))
            {
                i += 1;
            }
            let incomplete = if (chunk * (i + 1)).x.abs() <= speed.x.abs() {
                bounds = bounds.translate(chunk * i);
                speed.x %= self.tile_width;
                true
            } else {
                false
            };
            //If the remainder cannot be moved
            if incomplete || !self.shape_empty(bounds.translate(speed.x_comp())) {
                let mut bbox = bounds.bounding_box();
                if speed.x > 0f32 {
                    bbox.x = self.align_right(bbox.x + bbox.width) - bbox.width;
                } else {
                    bbox.x = self.align_left(bbox.x);
                }
                bounds = bounds.with_center(bbox.center());
                speed.x = 0f32;
            }
            i = 1;
            // Find the largest number of tiles that can be moved
            let chunk = Vector::y() * speed.y.signum() * self.tile_height;
            while (chunk * (i + 1)).y.abs() < speed.y.abs() &&
                self.shape_empty(bounds.translate(chunk * (i + 1)))
            {
                i += 1;
            }
            let incomplete = if (chunk * (i + 1)).y.abs() <= speed.y.abs() {
                bounds = bounds.translate(chunk * i);
                speed.y %= self.tile_height;
                true
            } else {
                false
            };
            //If the remainder cannot be moved
            if incomplete || !self.shape_empty(bounds.translate(speed)) {
                let mut bbox = bounds.bounding_box();
                if speed.y > 0f32 {
                    bbox.y = self.align_bottom(bbox.y + bbox.height) - bbox.height;
                } else {
                    bbox.y = self.align_top(bbox.y);
                }
                bounds = bounds.with_center(bbox.center());
                speed.y = 0f32;
            }
            (bounds.translate(speed), speed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Tilemap<i32> {
        let mut map = Tilemap::new(640f32, 480f32, 32f32, 32f32);
        map.set(Vector::newi(35, 35), Tile::solid(Some(5)));
        map
    }

    #[test]
    fn tile_values() {
        let map = setup();
        assert!(match map.get(Vector::x() * -1) {
            None => true,
            _ => false,
        });
        assert!(map.get(Vector::newi(35, 0)).unwrap().empty);
        assert!(!map.get(Vector::newi(35, 35)).unwrap().empty);
        assert!(!map.get(Vector::newi(35, 35)).unwrap().empty);
        assert_eq!(map.get(Vector::newi(35, 35)).unwrap().value.unwrap(), 5);
    }

    #[test]
    fn move_until_contact() {
        let map = setup();
        //Each test case has a starting rectangle, starting speed, expected top-left and expected
        //speed
        let test_cases = [
            (
                Rectangle::newi(300, 5, 32, 32),
                Vector::newi(0, -10),
                Vector::newi(300, 0),
                Vector::zero(),
            ),
            (
                Rectangle::newi(80, 10, 16, 16),
                Vector::newi(1, -20),
                Vector::newi(81, 0),
                Vector::newi(1, 0),
            ),
            (
                Rectangle::newi(600, 10, 30, 10),
                Vector::newi(15, 10),
                Vector::newi(610, 20),
                Vector::newi(0, 10),
            ),
            (
                Rectangle::newi(10, 5, 5, 5),
                Vector::newi(2, 2),
                Vector::newi(12, 7),
                Vector::newi(2, 2),
            ),
            (
                Rectangle::newi(5, 5, 10, 10),
                Vector::x() * -7.2,
                Vector::newi(0, 5),
                Vector::zero(),
            ),
            (
                Rectangle::newi(0, 0, 30, 30),
                Vector::x() * 2,
                Vector::newi(2, 0),
                Vector::x() * 2,
            ),
            (
                Rectangle::newi(0, 0, 30, 30),
                Vector::x() * 100,
                Vector::x() * 100,
                Vector::x() * 100,
            ),
            (
                Rectangle::newi(0, 0, 30, 30),
                Vector::y() * 100,
                Vector::y() * 100,
                Vector::y() * 100,
            ),
            (
                Rectangle::newi(150, 0, 30, 30),
                Vector::x() * -100,
                Vector::x() * 50,
                Vector::x() * -100,
            ),
            (
                Rectangle::newi(0, 150, 30, 30),
                Vector::y() * -200,
                Vector::zero(),
                Vector::zero(),
            ),
        ];
        for case in test_cases.iter() {
            let (region, speed, expected_tl, expected_speed) = *case;
            let (region_new, speed_new) = map.move_until_contact(Shape::Rect(region), speed);
            match region_new {
                Shape::Rect(region_new) => {
                    assert_eq!(region_new.top_left(), expected_tl);
                    assert_eq!(speed_new, expected_speed);
                },
                _ => ()
            }
        }
    }
}
