// Example 4: Render to Texture
// Render some data to an image, and draw that image to the screen
use quicksilver::{
    geom::{Circle, Rectangle, Transform, Vector},
    graphics::{Color, Image, PixelFormat, Surface},
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Square Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    gfx.clear(Color::WHITE);
    // Create a surface, which allows rendering to an image
    let mut surface = Surface::new(
        &gfx,
        Image::from_raw(&gfx, None, 512, 512, PixelFormat::RGBA)?,
    )?;
    // We can use projections to add letterboxing around the content, split-screen,
    // picture-in-picture, etc. For now we'll just change the size of our rendering area.
    // The projection controls how coordinates map to the drawing target
    // Here, we're mapping the coordinates one-to-one from our coordinates to the Surface
    gfx.set_projection(Transform::orthographic(Rectangle::new_sized(surface.size().unwrap())));
    // Draw a circle inside a rectangle
    gfx.fill_rect(
        &Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0)),
        Color::RED,
    );
    gfx.fill_circle(&Circle::new(Vector::new(400.0, 150.0), 50.0), Color::BLACK);
    // Flush to the surface, which draws to the image
    gfx.flush_surface(&surface)?;

    gfx.clear(Color::BLACK);
    // Now we're going to set the projection again, this time to the size of the window
    gfx.set_projection(Transform::orthographic(Rectangle::new_sized(window.size())));
    // Extract the image from the surface to use
    let image = surface.detach().expect("The image failed to detach");
    // Draw that image to the screen twice
    gfx.draw_image(&image, Rectangle::new_sized(Vector::new(400.0, 300.0)));
    gfx.draw_image(
        &image,
        Rectangle::new(Vector::new(400.0, 300.0), Vector::new(400.0, 300.0)),
    );
    gfx.present(&window)?;

    loop {
        while let Some(_) = input.next_event().await {}
    }
}
