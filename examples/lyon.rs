// Draw the classic triangle to the screen
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Transform, Vector},
    graphics::{Color, Mesh, ShapeRenderer},
    lifecycle::{Settings, State, Window, run},
    lyon::{
        extra::rust_logo::build_logo_path,
        tessellation::{FillTessellator, FillOptions},
<<<<<<< Updated upstream
        path::{Path, builder::*},
=======
        path::{builder::*, Path}
>>>>>>> Stashed changes
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

        let mut logo_shape = ShapeRenderer::new(&mut logo, Color::BLACK);
        logo_shape.set_transform(Transform::scale((3, 3)));
        tessellator.tessellate_path(&path, &FillOptions::tolerance(0.01), &mut logo_shape).unwrap();

        Ok(LyonExample { logo })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        window.mesh().extend(&self.logo);
        Ok(())
    }
}

fn main() {
    run::<LyonExample>("Lyon Demo", Vector::new(800, 600), Settings::default());
}
