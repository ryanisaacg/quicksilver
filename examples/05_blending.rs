// Example 2: The Image
// Draw an image to the screen
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::blend::{BlendChannel, BlendFactor, BlendFunction, BlendInput, BlendMode},
    graphics::{Color, Graphics, Image},
    input::{Input, Window},
    run, Result, Settings,
};

fn main() {
    run(
        Settings {
            title: "Blend Example",
            ..Settings::default()
        },
        app,
    );
}

// This time we might return an error, so we use a Result
async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let image = Image::load(&gfx, "image.png").await?;
    gfx.clear(Color::WHITE);
    // Set the blend pipeline
    // Use the default blend equation (which is adding the two colors)
    // Blend via the alpha of the source pixels, which means transparent objects show what's
    // underneath
    gfx.set_blend_mode(Some(BlendMode {
        equation: Default::default(),
        function: BlendFunction::Same {
            source: BlendFactor::Color {
                input: BlendInput::Source,
                channel: BlendChannel::Alpha,
                is_inverse: false,
            },
            destination: BlendFactor::Color {
                input: BlendInput::Source,
                channel: BlendChannel::Alpha,
                is_inverse: true,
            },
        },
        global_color: [0.0; 4],
    }));
    // Draw the example image with a red rectangle over it
    let region = Rectangle::new(Vector::new(200.0, 200.0), image.size());
    gfx.draw_image(&image, region);
    gfx.fill_rect(&region, Color::RED.with_alpha(0.5));
    // Set the blend pipeline again
    // Now, just use the source pixel and ignore the destination pixel, regardless of alpha
    gfx.set_blend_mode(Some(BlendMode {
        equation: Default::default(),
        function: BlendFunction::Same {
            source: BlendFactor::One,
            destination: BlendFactor::Zero,
        },
        global_color: [0.0; 4],
    }));
    let region = Rectangle::new(Vector::new(400.0, 300.0), image.size());
    gfx.draw_image(&image, region);
    gfx.fill_rect(&region, Color::RED.with_alpha(0.5));
    gfx.present(&window)?;

    loop {
        while let Some(_) = input.next_event().await {}
    }
}
