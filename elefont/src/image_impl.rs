use super::*;

use image::{ImageBuffer, Rgba};

impl Texture for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn width(&self) -> u32 {
        ImageBuffer::width(self)
    }

    fn height(&self) -> u32 {
        ImageBuffer::height(self)
    }

    fn put_rect(&mut self, pixel: PixelType, data: &[u8], gpu: &TextureGlyph) {
        use PixelType::*;

        assert!(gpu.bounds.x >= 0);
        assert!(gpu.bounds.y >= 0);
        let bx = gpu.bounds.x as u32;
        let by = gpu.bounds.y as u32;

        match pixel {
            Alpha => {
                for x in 0..gpu.bounds.width {
                    for y in 0..gpu.bounds.height {
                        for i in 0..3 {
                            self.get_pixel_mut(bx + x, by + y).0[i] = 255;
                        }
                        self.get_pixel_mut(bx + x, by + y).0[3] =
                            data[(x + y * gpu.bounds.width) as usize];
                    }
                }
            }
            RGBA => {
                for x in bx..gpu.bounds.width {
                    for y in by..gpu.bounds.height {
                        let index = ((x + y * gpu.bounds.height) * 4) as usize;
                        let pixel = &data[index..(4 + index)];
                        self.get_pixel_mut(x, y).0.clone_from_slice(pixel);
                    }
                }
            }
        }
    }
}
