use blinds::*;
use golem::{
    Attribute, AttributeType, ColorFormat, Context,
    Dimension::{D2, D4},
    ElementBuffer, GeometryMode, GolemError, NumberType, ShaderDescription, ShaderProgram, Surface,
    Texture, Uniform, UniformType, UniformValue, VertexBuffer,
};

async fn app(
    window: Window,
    ctx: golem::glow::Context,
    mut events: EventStream,
) -> Result<(), GolemError> {
    let ctx = &Context::from_glow(ctx)?;

    // Step 1: Draw a triangle to the surface
    #[rustfmt::skip]
    let vertices = [
        // Position         Color
        -0.5, -0.5,         1.0, 0.0, 0.0, 1.0,
        0.5, -0.5,          0.0, 1.0, 0.0, 1.0,
        0.0, 0.5,           0.0, 0.0, 1.0, 1.0
    ];
    let indices = [0, 1, 2];

    let mut shader = ShaderProgram::new(
        ctx,
        ShaderDescription {
            vertex_input: &[
                Attribute::new("vert_position", AttributeType::Vector(D2)),
                Attribute::new("vert_color", AttributeType::Vector(D4)),
            ],
            fragment_input: &[Attribute::new("frag_color", AttributeType::Vector(D4))],
            uniforms: &[],
            vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
            frag_color = vert_color;
        }"#,
            fragment_shader: r#" void main() {
            gl_FragColor = frag_color;
        }"#,
        },
    )?;

    let mut vb = VertexBuffer::new(ctx)?;
    let mut eb = ElementBuffer::new(ctx)?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind();
    let mut backing_texture = Texture::new(ctx)?;
    backing_texture.set_image(None, 100, 100, ColorFormat::RGBA);
    ctx.set_viewport(0, 0, backing_texture.width(), backing_texture.height());
    let surface = Surface::new(ctx, backing_texture)?;

    surface.bind();
    ctx.clear();
    unsafe {
        shader.draw(&vb, &eb, 0..indices.len(), GeometryMode::Triangles)?;
    }
    Surface::unbind(ctx);

    let size = window.size();
    let scale = window.scale_factor();
    ctx.set_viewport(0, 0, (size.x * scale) as u32, (size.y * scale) as u32);

    // Step 2: Draw a few copies of this triangle to the screen
    // Also, for fun, let's rotate them dynamically
    #[rustfmt::skip]
    let vertices = [
        // Position         UV
        -0.2, -0.2,         0.0, 0.0,
        0.2, -0.2,          1.0, 0.0,
        0.2, 0.2,           1.0, 1.0,
        -0.2, 0.2,          0.0, 1.0,
    ];
    let indices = [0, 1, 2, 2, 3, 0];
    let mut shader = ShaderProgram::new(
        ctx,
        ShaderDescription {
            vertex_input: &[
                Attribute::new("vert_position", AttributeType::Vector(D2)),
                Attribute::new("vert_uv", AttributeType::Vector(D2)),
            ],
            fragment_input: &[Attribute::new("frag_uv", AttributeType::Vector(D2))],
            uniforms: &[
                Uniform::new("image", UniformType::Sampler2D),
                Uniform::new("rotate", UniformType::Matrix(D2)),
                Uniform::new("translate", UniformType::Vector(NumberType::Float, D2)),
            ],
            vertex_shader: r#" void main() {
            gl_Position = vec4(translate + (rotate * vert_position), 0, 1);
            frag_uv = vert_uv;
        }"#,
            fragment_shader: r#" void main() {
            gl_FragColor = texture(image, frag_uv);
        }"#,
        },
    )?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind();
    shader.prepare_draw(&vb, &eb)?;
    shader.set_uniform("image", UniformValue::Int(1))?;

    let bind_point = std::num::NonZeroU32::new(1).unwrap();
    unsafe {
        let texture = surface.borrow_texture().unwrap();
        texture.set_active(bind_point);
    }

    let mut angle = 0f32;
    loop {
        while let Some(_) = events.next_event().await {}
        ctx.clear();
        let draw = |angle: f32, translate| -> Result<(), GolemError> {
            let c = angle.cos();
            let s = angle.sin();
            let rotate = [c, -s, s, c];
            shader.set_uniform("rotate", UniformValue::Matrix2(rotate))?;
            shader.set_uniform("translate", UniformValue::Vector2(translate))?;
            unsafe {
                shader.draw_prepared(0..indices.len(), GeometryMode::Triangles);
            }

            Ok(())
        };
        draw(0.0, [0.0, 0.0])?;
        draw(angle, [-0.5, -0.5])?;
        draw(angle / 2.0, [-0.5, 0.5])?;
        draw(angle / 4.0, [0.5, 0.5])?;
        draw(angle / 8.0, [0.5, -0.5])?;

        angle += 0.005;

        window.present();
    }
}

fn main() {
    run_gl(Settings::default(), |window, gfx, events| async move {
        app(window, gfx, events).await.unwrap()
    });
}
