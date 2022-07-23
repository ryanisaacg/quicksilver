use elefont::rusttype_provider::SizedFont;
use elefont::FontCache;
use image::ImageBuffer;
use rusttype::Font;

fn main() {
    let font_data = include_bytes!("DejaVuSans.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
    let font = SizedFont::new(font, 24.0);

    let image = ImageBuffer::new(200, 200);

    let mut cache = FontCache::new(Box::new(font), image);
    cache.render_string("Hello, world!").for_each(|r| {
        r.unwrap();
    });
    cache.render_string("こんにちは世界！").for_each(|r| {
        r.unwrap();
    });
    cache.render_string("Привет, мир!").for_each(|r| {
        r.unwrap();
    });
    cache
        .texture()
        .save("result.png")
        .expect("Failed to save file");
}
