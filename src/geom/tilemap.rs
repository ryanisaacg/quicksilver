use geom::{Rectangle, Vector};

#[derive(Clone)]
pub struct Tile<T: Clone> {
    pub value: Option<T>,
    pub empty: bool
}

impl<T: Clone> Tile<T> {
    pub fn solid(value: Option<T>) -> Tile<T> {
        Tile { value: value, empty: false }
    }

    pub fn empty(value: Option<T>) -> Tile<T> {
        Tile { value: value, empty: true }
    }
}


pub struct Tilemap<T: Clone> {
    data: Vec<Tile<T>>,
    width: f32,
    height: f32,
    tile_width: f32,
    tile_height: f32
}

impl<T: Clone> Tilemap<T> {

    pub fn new(map_width: f32, map_height: f32, tile_width: f32, tile_height: f32) -> Tilemap<T> {
        Tilemap {
            data: vec![Tile::empty(None); (map_width / tile_width * map_height / tile_height) as usize],
            width: map_width,
            height: map_height,
            tile_width: tile_width,
            tile_height: tile_height
        }
    }

    pub fn valid(&self, index: Vector) -> bool {
        index.x >= 0f32 && index.y >= 0f32 && index.x < self.width && index.y < self.height
    }

    fn array_index(&self, index: Vector) -> usize {
        ((index.x / self.tile_width).trunc() * (self.height / self.tile_height).trunc() 
                    + (index.y / self.tile_height).trunc()) as usize
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
            None => ()
        }
    }

    pub fn point_empty(&self, index: Vector) -> bool {
        match self.get(index) {
            Some(tile) => tile.empty,
            None => false
        }
    }

    pub fn region_empty(&self, area: Rectangle) -> bool {
        let mut x = area.x;
        let mut y = area.y;
        while x < area.x + area.width {
            while y < area.y + area.height {
                if !self.point_empty(Vector::new(x, y)) {
                    return false;
                }
                y += self.tile_height;
            }
            x += self.tile_width;
        }
        self.point_empty(area.top_left() + area.size().x_comp()) &&
            self.point_empty(area.top_left() + area.size().y_comp()) &&
            self.point_empty(area.top_left() + area.size())
    }

    pub fn move_until_contact(&self, bounds: Rectangle, speed: Vector) -> (Rectangle, Vector) {
        if !self.region_empty(bounds) {
            (bounds, Vector::zero())
        } else {
            let mut bounds = bounds;
            let mut attempt = speed.x_comp();
            //Move tile-wise along the x component
            while attempt.x > self.tile_width && self.region_empty(bounds.translate(attempt)) {
                attempt.x -= self.tile_width;
            }
            //If the remainder cannot be moved
            if !self.region_empty(bounds.translate(attempt)) {
                if attempt.x > 0f32 {
                    println!("{}", bounds.x + bounds.width);
                    bounds.x = ((bounds.x + bounds.width) / self.tile_width).ceil() * self.tile_width - bounds.width - 1f32;
                } else {
                    bounds.x = (bounds.x / self.tile_width).floor() * self.tile_width;
                }
                attempt.x = 0f32;
            }
            attempt += speed.y_comp();
            //Move tile-wise along the y component
            while attempt.y > self.tile_height && self.region_empty(bounds.translate(attempt)) {
                attempt.y -= self.tile_height;
            }
            //If the remainder cannot be moved
            if !self.region_empty(bounds.translate(attempt)) {
                if attempt.y > 0f32 {
                    bounds.y = ((bounds.y + bounds.height) / self.tile_height).ceil() * self.tile_height - bounds.height - 1f32;
                } else {
                    bounds.y = (bounds.y / self.tile_height).floor() * self.tile_height;
                }
                attempt.y = 0f32;
            }
            (bounds.translate(attempt), attempt)
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
        assert!(match map.get(Vector::x() * -1) { None => true, _ => false });
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
        let test_cases = [(Rectangle::newi(300, 5, 32, 32), Vector::newi(0, -10),
                Vector::newi(300, 0), Vector::zero()),
            (Rectangle::newi(80, 10, 16, 16), Vector::newi(1, -20),
                Vector::newi(81, 0), Vector::newi(1, 0)),
            (Rectangle::newi(600, 10, 30, 10), Vector::newi(15, 10),
                Vector::newi(609, 20), Vector::newi(0, 10)),
            (Rectangle::newi(10, 5, 5, 5), Vector::newi(2, 2),
                Vector::newi(12, 7), Vector::newi(2, 2)),
            (Rectangle::newi(5, 5, 10, 10), Vector::x() * -7.2,
                Vector::newi(0, 5), Vector::zero())];
        for case in test_cases.iter() {
            let (region, speed, expected_tl, expected_speed) = *case;
            let (region_new, speed_new) = map.move_until_contact(region, speed);
            assert_eq!(region_new.top_left(), expected_tl);
            assert_eq!(speed_new, expected_speed);
        }
    }
}
