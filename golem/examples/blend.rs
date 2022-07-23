use blinds::*;
use golem::blend::*;
use golem::{
    Attribute, AttributeType, Context,
    Dimension::{D2, D4},
    ElementBuffer, GeometryMode, GolemError, NumberType, ShaderDescription, ShaderProgram, Uniform,
    UniformType, UniformValue, VertexBuffer,
};

async fn app(
    window: Window,
    ctx: golem::glow::Context,
    mut events: EventStream,
) -> Result<(), GolemError> {
    let ctx = &Context::from_glow(ctx)?;

    #[rustfmt::skip]
    let vertices = [
        // Position
        -0.5, -0.5,
        0.5, -0.5,
        0.5, 0.5,
        -0.5, 0.5,
    ];
    let indices = [0, 1, 2, 2, 3, 0];

    let mut shader = ShaderProgram::new(
        ctx,
        ShaderDescription {
            vertex_input: &[Attribute::new("vert_position", AttributeType::Vector(D2))],
            fragment_input: &[],
            uniforms: &[Uniform::new(
                "color",
                UniformType::Vector(NumberType::Float, D4),
            )],
            vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
        }"#,
            fragment_shader: r#" void main() {
            gl_FragColor = color;
        }"#,
        },
    )?;

    let mut vb = VertexBuffer::new(ctx)?;
    let mut eb = ElementBuffer::new(ctx)?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    ctx.clear();
    ctx.set_blend_mode(Some(BlendMode {
        equation: BlendEquation::default(),
        function: BlendFunction::default(),
        global_color: [0.0; 4],
    }));

    shader.bind();
    shader.prepare_draw(&vb, &eb)?;
    shader.set_uniform("color", UniformValue::Vector4([1.0, 0.0, 0.0, 0.5]))?;
    unsafe {
        shader.draw_prepared(0..indices.len(), GeometryMode::Triangles);
    }
    shader.set_uniform("color", UniformValue::Vector4([0.0, 0.0, 1.0, 0.5]))?;
    unsafe {
        shader.draw_prepared(0..indices.len(), GeometryMode::Triangles);
    }
    window.present();

    loop {
        events.next_event().await;
    }
}

fn main() {
    run_gl(Settings::default(), |window, gfx, events| async move {
        app(window, gfx, events).await.unwrap()
    });
}
