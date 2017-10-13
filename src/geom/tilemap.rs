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
        map.set(Vector::new(35f32, 35f32), Tile::solid(Some(5)));
        map
    }

    #[test]
    fn tile_values() {
        let map = setup();
        assert!(match map.get(Vector::new(-1f32, 0f32)) { None => true, _ => false });
        assert!(map.get(Vector::new(35f32, 0f32)).unwrap().empty);
        assert!(!map.get(Vector::new(35f32, 35f32)).unwrap().empty);
        assert!(!map.get(Vector::new(35f32, 35f32)).unwrap().empty);
        assert_eq!(map.get(Vector::new(35f32, 35f32)).unwrap().value.unwrap(), 5);
    }

    #[test]
    fn move_until_contact() {
        let map = setup();
        //Each test case has a starting rectangle, starting speed, expected top-left and expected
        //speed
        let test_cases = [(Rectangle::new(300f32, 5f32, 32f32, 32f32), Vector::new(0f32, -10f32),
                Vector::new(300f32, 0f32), Vector::zero()),
            (Rectangle::new(80f32, 10f32, 16f32, 16f32), Vector::new(1f32, -20f32),
                Vector::new(81f32, 0f32), Vector::new(1f32, 0f32)),
            (Rectangle::new(600f32, 10f32, 30f32, 10f32), Vector::new(15f32, 10f32),
                Vector::new(609f32, 20f32), Vector::new(0f32, 10f32)),
            (Rectangle::new(10f32, 5f32, 5f32, 5f32), Vector::new(2f32, 2f32),
                Vector::new(12f32, 7f32), Vector::new(2f32, 2f32)),
            (Rectangle::new(5f32, 5f32, 10f32, 10f32), Vector::new(-7.2f32, 0f32),
                Vector::new(0f32, 5f32), Vector::zero())];
        for case in test_cases.iter() {
            let (region, speed, expected_tl, expected_speed) = *case;
            let (region_new, speed_new) = map.move_until_contact(region, speed);
            assert_eq!(region_new.top_left(), expected_tl);
            assert_eq!(speed_new, expected_speed);
        }
    }
}
