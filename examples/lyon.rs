// Draw the classic triangle to the screen
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::Transform,
    graphics::{Color, Mesh, ShapeRenderer},
    lifecycle::{Settings, State, Window, run},
    lyon::{
        extra::rust_logo::build_logo_path,
        tessellation::{FillTessellator, FillOptions},
        path::{
            builder::*,
            default::Path
        }
    }
};


struct LyonExample {
    logo: Mesh
}

impl State for LyonExample {
    fn new() -> Result<LyonExample> {
        // Build a Path for the rust logo.
        let mut builder = SvgPathBuilder::new(Path::builder());
        build_logo_path(&mut builder);
        let path = builder.build();

        let mut tessellator = FillTessellator::new();

        let mut logo = Mesh::new();

        {
            let mut logo = ShapeRenderer::new(&mut logo, Color::BLACK);
            logo.set_transform(Transform::scale((3, 3)));
            tessellator.tessellate_path(path.path_iter(),
                &FillOptions::tolerance(0.01), &mut logo).unwrap();
        }

        Ok(LyonExample { logo })
    }

    fn draw(&mut self, window: &mut Window, _delta_time: f64) -> Result<()> {
        window.clear(Color::WHITE)?;
        window.mesh().apply(&self.logo);
        Ok(())
    }
}

fn main() {
    run::<LyonExample>("Lyon Demo", Vector::new(800, 600), Settings::default());
}
