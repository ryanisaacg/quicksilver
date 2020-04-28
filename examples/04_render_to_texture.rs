// Example 4: Render to Texture
// Render some data to an image, and draw that image to the screen
use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Color, Graphics, Image, PixelFormat, Surface},
    input::{Input, Window},
    run, Result, Settings,
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
    // Set the render area to the surface size
    gfx.fit_to_surface(&surface)?;
    // Draw a circle inside a rectangle
    gfx.fill_rect(
        &Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0)),
        Color::RED,
    );
    gfx.fill_circle(&Circle::new(Vector::new(400.0, 150.0), 50.0), Color::BLACK);
    // Flush to the surface, which draws to the image
    gfx.flush(Some(&surface))?;

    gfx.clear(Color::BLACK);
    // Reset the viewport to the window size
    gfx.fit_to_window(&window);
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
