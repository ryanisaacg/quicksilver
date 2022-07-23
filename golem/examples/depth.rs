use blinds::*;
use golem::depth::*;
use golem::{
    Attribute, AttributeType, Context,
    Dimension::{D3, D4},
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
    let triangle_1 = [
        0.5, -0.5, 0.1,
        -0.5, 0.0, 0.1,
        0.5, 0.5, 0.1,
    ];
    #[rustfmt::skip]
    let triangle_2 = [
        -0.5, -0.5, 0.2,
        0.5, 0.0, 0.2,
        -0.5, 0.5, 0.2,
    ];
    let indices = [0, 1, 2];

    let mut shader = ShaderProgram::new(
        ctx,
        ShaderDescription {
            vertex_input: &[Attribute::new("vert_position", AttributeType::Vector(D3))],
            fragment_input: &[],
            uniforms: &[Uniform::new(
                "color",
                UniformType::Vector(NumberType::Float, D4),
            )],
            vertex_shader: r#" void main() {
                gl_Position = vec4(vert_position, 1);
            }"#,
            fragment_shader: r#" void main() {
                gl_FragColor = color;
            }"#,
        },
    )?;

    let mut vb = VertexBuffer::new(ctx)?;
    let mut eb = ElementBuffer::new(ctx)?;
    eb.set_data(&indices);
    ctx.clear();
    ctx.set_depth_test_mode(Some(DepthTestMode {
        function: DepthTestFunction::Less,
        range_near: 0.0,
        range_far: 1.0,
        depth_mask: true,
    }));

    shader.bind();
    vb.set_data(&triangle_1);
    shader.set_uniform("color", UniformValue::Vector4([1.0, 0.0, 0.0, 1.0]))?;
    unsafe {
        shader.draw(&vb, &eb, 0..indices.len(), GeometryMode::Triangles)?;
    }
    // Second (green) triangle is drawn after the first (red), but is occluded due to depth test.
    // Try disabling the depth test or setting it to different functions.
    vb.set_data(&triangle_2);
    shader.set_uniform("color", UniformValue::Vector4([0.0, 1.0, 0.0, 1.0]))?;
    unsafe {
        shader.draw(&vb, &eb, 0..indices.len(), GeometryMode::Triangles)?;
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
