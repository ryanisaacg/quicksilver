use geom::{Positioned, Rectangle, Vector, Shape};
use std::ops::Fn;

#[derive(Clone, Deserialize, Serialize)]
///An individual tile
pub struct Tile<T: Clone> {
    ///The value stored in this tile
    pub value: Option<T>,
    ///If the tile is empty from a movement perspective
    pub empty: bool,
}

impl<T: Clone> Tile<T> {
    ///Create a solid version of a tile
    pub fn solid(value: Option<T>) -> Tile<T> {
        Tile {
            value: value,
            empty: false,
        }
    }

    ///Create a non-solid version of a tile
    pub fn empty(value: Option<T>) -> Tile<T> {
        Tile {
            value: value,
            empty: true,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
///A grid of Tile values
pub struct Tilemap<T: Clone> {
    data: Vec<Tile<T>>,
    map_size: Vector,
    tile_size: Vector
}

impl<T: Clone> Tilemap<T> {
    ///Create a map full of empty, non-solid tiles of a given size
    pub fn new(map_size: Vector, tile_size: Vector ) -> Tilemap<T> {
        let data = vec![Tile::empty(None);(map_size.x / tile_size.x * map_size.y / tile_size.y) as usize];
        Tilemap { data, map_size, tile_size }
    }

    ///Create a map with pre-filled data
    pub fn with_data(data: Vec<Tile<T>>, map_size: Vector, tile_size: Vector) -> Tilemap<T> {
        Tilemap { data, map_size, tile_size }
    }

    ///Get the width of the map
    pub fn width(&self) -> f32 {
        self.map_size.x
    }

    ///Get the height of the map
    pub fn height(&self) -> f32 {
        self.map_size.y
    }

    ///Get the size of the map
    pub fn size(&self) -> Vector {
        self.map_size
    }

    ///Get the region the map takes up
    pub fn region(&self) -> Rectangle {
        Rectangle::newv_sized(self.map_size)
    }

    ///Get the width of an individual tile
    pub fn tile_width(&self) -> f32 {
        self.tile_size.x
    }

    ///Get the height of an individual tile
    pub fn tile_height(&self) -> f32 {
        self.tile_size.y
    }

    ///Get the size of a tile
    pub fn tile_size(&self) -> Vector {
        self.tile_size
    }

    ///Check if a point is within the map bounds
    pub fn valid(&self, index: Vector) -> bool {
        self.region().contains(index)
    }

    ///Checks if a shape is valid in its entirety
    pub fn shape_valid(&self, shape: Shape) -> bool {
        let bbox = shape.bounding_box();
        self.valid(bbox.top_left()) && self.valid(bbox.top_left() + bbox.size())
    }

    fn array_index(&self, index: Vector) -> usize {
        ((index.x / self.tile_width()).trunc() * (self.height() / self.tile_height()).trunc() +
             (index.y / self.tile_height()).trunc()) as usize
    }

    ///Get the tile found at a given point, if it is valid
    pub fn get(&self, index: Vector) -> Option<&Tile<T>> {
        if self.valid(index) {
            Some(&self.data[self.array_index(index)])
        } else {
            None
        }
    }

    ///Get a mutable reference to a tile at a given point, if it is valid
    pub fn get_mut(&mut self, index: Vector) -> Option<&mut Tile<T>> {
        if self.valid(index) {
            let index = self.array_index(index);
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    ///Set the value at a given point
    pub fn set(&mut self, index: Vector, value: Tile<T>) {
        match self.get_mut(index) {
            Some(tile) => *tile = value,
            None => (),
        }
    }

    ///Find if a point's tile is empty
    pub fn point_empty(&self, index: Vector) -> bool {
        match self.get(index) {
            Some(tile) => tile.empty,
            None => false,
        }
    }
   
    ///Finds if the area taken by a shape is empty
    pub fn shape_empty(&self, shape: Shape) -> bool {
        let bounds = shape.bounding_box(); 
        match shape {
            Shape::Vect(_) => self.point_empty(shape.center()),
            Shape::Rect(_) | Shape::Circ(_) | Shape::Line(_) => {
                let x_start = (self.align_left(bounds.x) / self.tile_width()) as i32;
                let y_start = (self.align_top(bounds.y) / self.tile_height()) as i32;
                let x_end = (self.align_right(bounds.x + bounds.width) / self.tile_width()) as i32;
                let y_end = (self.align_right(bounds.y + bounds.height) / self.tile_height()) as i32;
                for x in x_start..x_end {
                    for y in y_start..y_end {
                        let point = Vector::new(x, y).times(self.tile_size());
                        if !self.point_empty(point) && shape.overlaps_rect(Rectangle::newv(point, self.tile_size())) {
                            return false;
                        }
                    }
                }
                true
            }
        }
    }

    ///Align a given X value to the leftmost edge of a tile
    pub fn align_left(&self, x: f32) -> f32 {
        (x / self.tile_width()).floor() * self.tile_width()
    }

    ///Align a given X value to the rightmost edge of a tile
    pub fn align_right(&self, x: f32) -> f32 {
        (x / self.tile_width()).ceil() * self.tile_width()
    }

    ///Align a given Y value to the topmost edge of a tile
    pub fn align_top(&self, y: f32) -> f32 {
        (y / self.tile_height()).floor() * self.tile_height()
    }

    ///Align a given Y value to the bottommost edge of a tile
    pub fn align_bottom(&self, y: f32) -> f32 {
        (y / self.tile_height()).ceil() * self.tile_height()
    }

    ///Find the furthest a shape can move along a vector, and what its future speed should be
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
            let chunk = Vector::x() * speed.x.signum() * self.tile_width();
            while (chunk * (i + 1)).x.abs() < speed.x.abs() &&
                self.shape_empty(bounds.translate(chunk * (i + 1)))
            {
                i += 1;
            }
            let incomplete = if (chunk * (i + 1)).x.abs() <= speed.x.abs() {
                bounds = bounds.translate(chunk * i);
                speed.x %= self.tile_width();
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
            let chunk = Vector::y() * speed.y.signum() * self.tile_height();
            while (chunk * (i + 1)).y.abs() < speed.y.abs() &&
                self.shape_empty(bounds.translate(chunk * (i + 1)))
            {
                i += 1;
            }
            let incomplete = if (chunk * (i + 1)).y.abs() <= speed.y.abs() {
                bounds = bounds.translate(chunk * i);
                speed.y %= self.tile_height();
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

    ///Convert a Tilemap into a map of a different type
    pub fn convert<U, F>(&self, conversion: F) -> Tilemap<U> 
        where U: Clone, F: Fn(&T) -> U {
        Tilemap {
            data: self.data.iter()
                .map(|tile| Tile {
                    value: match tile.value {
                        Some(ref x) => Some(conversion(x)),
                        None => None
                    },
                    empty: tile.empty
                }).collect(),
            map_size: self.map_size,
            tile_size: self.tile_size
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Tilemap<i32> {
        let mut map = Tilemap::new(Vector::new(640, 480), Vector::new(32, 32));
        map.set(Vector::new(35, 35), Tile::solid(Some(5)));
        map
    }

    #[test]
    fn tile_values() {
        let map = setup();
        assert!(match map.get(Vector::x() * -1) {
            None => true,
            _ => false,
        });
        assert!(map.get(Vector::new(35, 0)).unwrap().empty);
        assert!(!map.get(Vector::new(35, 35)).unwrap().empty);
        assert!(!map.get(Vector::new(35, 35)).unwrap().empty);
        assert_eq!(map.get(Vector::new(35, 35)).unwrap().value.unwrap(), 5);
    }

    #[test]
    fn move_until_contact() {
        let map = setup();
        //Each test case has a starting rectangle, starting speed, expected top-left and expected
        //speed
        let test_cases = [
            (
                Rectangle::new(300, 5, 32, 32),
                Vector::new(0, -10),
                Vector::new(300, 0),
                Vector::zero(),
            ),
            (
                Rectangle::new(80, 10, 16, 16),
                Vector::new(1, -20),
                Vector::new(81, 0),
                Vector::new(1, 0),
            ),
            (
                Rectangle::new(600, 10, 30, 10),
                Vector::new(15, 10),
                Vector::new(610, 20),
                Vector::new(0, 10),
            ),
            (
                Rectangle::new(10, 5, 5, 5),
                Vector::new(2, 2),
                Vector::new(12, 7),
                Vector::new(2, 2),
            ),
            (
                Rectangle::new(5, 5, 10, 10),
                Vector::x() * -7.2,
                Vector::new(0, 5),
                Vector::zero(),
            ),
            (
                Rectangle::new(0, 0, 30, 30),
                Vector::x() * 2,
                Vector::new(2, 0),
                Vector::x() * 2,
            ),
            (
                Rectangle::new(0, 0, 30, 30),
                Vector::x() * 100,
                Vector::x() * 100,
                Vector::x() * 100,
            ),
            (
                Rectangle::new(0, 0, 30, 30),
                Vector::y() * 100,
                Vector::y() * 100,
                Vector::y() * 100,
            ),
            (
                Rectangle::new(150, 0, 30, 30),
                Vector::x() * -100,
                Vector::x() * 50,
                Vector::x() * -100,
            ),
            (
                Rectangle::new(0, 150, 30, 30),
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
