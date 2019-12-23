// Draw the classic triangle to the screen
extern crate quicksilver;

use quicksilver::{
    geom::{Transform, Vector},
    graphics::{Color, Mesh, ShapeRenderer},
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    lyon::{
        extra::rust_logo::build_logo_path,
        path::{builder::*, Path},
        tessellation::{FillOptions, FillTessellator, StrokeOptions, StrokeTessellator},
    },
    Result,
};

struct LyonExample {
    filled_logo: Mesh,
    stroked_logo: Mesh,
    draw_filled: bool,
}

impl State for LyonExample {
    fn new() -> Result<LyonExample> {
        // Build a Path for the rust logo.
        let mut builder = SvgPathBuilder::new(Path::builder());
        build_logo_path(&mut builder);
        let path = builder.build();

        let filled_logo = {
            let mut logo = Mesh::new();
            let mut logo_shape = ShapeRenderer::new(&mut logo, Color::BLACK);
            logo_shape.set_transform(Transform::scale((3, 3)));
            let mut tessellator = FillTessellator::new();
            tessellator
                .tessellate_path(&path, &FillOptions::tolerance(0.01), &mut logo_shape)
                .unwrap();
            logo
        };

        let stroked_logo = {
            let mut logo = Mesh::new();
            let mut logo_shape = ShapeRenderer::new(&mut logo, Color::BLACK);
            logo_shape.set_transform(Transform::scale((3, 3)));
            let mut tessellator = StrokeTessellator::new();
            tessellator
                .tessellate_path(
                    &path,
                    &StrokeOptions::tolerance(0.01).with_line_width(0.4),
                    &mut logo_shape,
                )
                .unwrap();
            logo
        };

        Ok(LyonExample {
            filled_logo,
            stroked_logo,
            draw_filled: true,
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match *event {
            Event::Key(Key::Space, ButtonState::Pressed) => {
                self.draw_filled = !self.draw_filled;
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        window.mesh().extend(if self.draw_filled {
            &self.filled_logo
        } else {
            &self.stroked_logo
        });
        Ok(())
    }
}

fn main() {
    run::<LyonExample>(
        "Lyon Demo - press Space to switch between tessellation methods",
        Vector::new(800, 600),
        Settings {
            multisampling: Some(4),
            ..Settings::default()
        },
    );
}
