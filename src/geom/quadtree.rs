use geom::{Bounded, Rectangle, Shape};

/// A QuadTree that stores bounded objects and allows for collision checking
pub struct QuadTree<T: Bounded> {
    areas: Vec<Rectangle>,
    nodes: Vec<Vec<T>>
}

impl<T: Bounded> QuadTree<T> {
    /// Create a quadtree covering a given area with a given 'depth'
    ///
    /// Depth is more of an implementation detail: it is the number of levels in the tree,
    /// which determines how objects are distributed in the structure.
    pub fn new(area: Rectangle, depth: u32) -> QuadTree<T> {
        // The formula for determing length is (1 - 4 ^ depth) / (1 - 4), for the sum of the sequence
        // 1 + 4 + 16 + 64 + ... 4 ^ depth
        // which is the number of nodes in a quadtree with the given depth
        // The number of branches is the number of nodes in a tree with 1 less depth
        let branches = (1 - i32::pow(4, depth - 1)) / -3;
        let mut areas = Vec::new();
        let mut nodes = Vec::new();
        areas.push(area);
        nodes.push(Vec::new());
        for i in 0..(branches as usize) {
            let area = areas[i];
            let size = area.size() / 2;
            areas.push(Rectangle::newv(area.top_left(), size));
            areas.push(Rectangle::newv(area.top_left() + size.x_comp(), size));
            areas.push(Rectangle::newv(area.top_left() + size, size));
            areas.push(Rectangle::newv(area.top_left() + size.y_comp(), size));
            for _ in 0..4 {
                nodes.push(Vec::new());
            }
        }
        QuadTree { areas, nodes }
    }

    fn get_index(&self, bounds: Rectangle) -> i32 {
        let mut index = 0;
        while self.areas[index].contains_rect(bounds) && index * 4 + 5 < self.areas.len() {
            for i in 0..4 {
                let child = index * 4 + i;
                if self.areas[child].contains_rect(bounds) {
                    index = child;
                    continue;
                }
            }
            break;
        }
        index as i32
    }

    /// Add an object to the structure
    pub fn insert(&mut self, object: T) {
        let index = self.get_index(object.bounding_box());
        self.nodes[index as usize].push(object);
    }

    fn recurse_up<F>(&mut self, index: i32, bounds: Shape, f: &mut F) where F: FnMut(&mut T) {
        for object in self.nodes[index as usize].iter_mut().filter(|item| item.overlaps(&bounds)) {
            f(object);
        }
        if index > 0 {
            self.recurse_up((index - 1) / 4, bounds, f);
        }
    }

    fn recurse_down<F>(&mut self, index: i32, bounds: Shape, f: &mut F) where F: FnMut(&mut T) {
        if index * 4 + 5 < self.areas.len() as i32 {
            for object in self.nodes[index as usize].iter_mut().filter(|item| item.overlaps(&bounds)) {
                f(object);
            }
            for i in 0..4 {
                let child = index * 4 + i;
                self.recurse_down(child, bounds, f);
            }
        }
    }

    /// Apply a function to all items that intersect with the shape
    pub fn query<F>(&mut self, bounds: Shape, mut f: F) where F: FnMut(&mut T) {
        
        let index = self.get_index(bounds.bounding_box());
        self.recurse_up(index, bounds, &mut f);
        self.recurse_down(index, bounds, &mut f);
    }
}

#[cfg(test)]
mod tests {
    use geom::Vector;
    use super::*;

    #[test]
    fn query() {
        let mut quadtree = QuadTree::new(Rectangle::new_sized(100, 100), 10);
        quadtree.insert(Rectangle::new(20, 20, 50, 50));
        let mut found = false;
        quadtree.query(Shape::Vect(Vector::new(35, 35)), |_| found = true);
        assert!(found);
    }    
}