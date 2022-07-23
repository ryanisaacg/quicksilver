use blinds::*;
use core::convert::TryInto;
use golem::depth::*;
use golem::{
    Attribute, AttributeType, Context,
    Dimension::{D3, D4},
    ElementBuffer, GeometryMode, GolemError, NumberType, ShaderDescription, ShaderProgram, Uniform,
    UniformType, UniformValue, VertexBuffer,
};
use nalgebra_glm as glm;

async fn app(
    window: Window,
    ctx: golem::glow::Context,
    mut events: EventStream,
) -> Result<(), GolemError> {
    let ctx = &Context::from_glow(ctx)?;

    // A cube
    #[rustfmt::skip]
    let vertices = [
        -1.0, -1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, -1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
    ];
    #[rustfmt::skip]
    let indices = [
        0, 1, 2, 2, 1, 3,
        0, 4, 5, 0, 5, 1,
        1, 5, 3, 3, 5, 7,
        4, 6, 5, 5, 6, 7,
        2, 3, 6, 6, 3, 7,
        0, 2, 4, 4, 2, 6
    ];

    let mut shader = ShaderProgram::new(
        ctx,
        ShaderDescription {
            vertex_input: &[Attribute::new("vert_position", AttributeType::Vector(D3))],
            fragment_input: &[],
            uniforms: &[
                Uniform::new("color", UniformType::Vector(NumberType::Float, D4)),
                Uniform::new("projection", UniformType::Matrix(D4)),
                Uniform::new("view", UniformType::Matrix(D4)),
                Uniform::new("model", UniformType::Matrix(D4)),
            ],
            vertex_shader: r#" void main() {
                gl_Position = projection * view * model * vec4(vert_position, 1);
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
    ctx.set_depth_test_mode(Some(DepthTestMode {
        function: DepthTestFunction::Less,
        range_near: 0.1,
        range_far: 100.0,
        depth_mask: true,
    }));

    shader.bind();
    shader.prepare_draw(&vb, &eb)?;
    shader.set_uniform(
        "projection",
        UniformValue::Matrix4(
            glm::perspective(
                window.size().x / window.size().y,
                core::f32::consts::FRAC_PI_4,
                0.1,
                100.0,
            )
            .as_slice()
            .try_into()
            .unwrap(),
        ),
    )?;
    shader.set_uniform(
        "view",
        UniformValue::Matrix4(
            glm::look_at(
                &glm::vec3(10.0, 7.0, -10.0),
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 1.0, 0.0),
            )
            .as_slice()
            .try_into()
            .unwrap(),
        ),
    )?;

    // Red cube (front)
    shader.set_uniform("color", UniformValue::Vector4([1.0, 0.0, 0.0, 1.0]))?;
    shader.set_uniform(
        "model",
        UniformValue::Matrix4(
            glm::translate(&glm::identity(), &glm::vec3(1.25, 0.0, 0.0))
                .as_slice()
                .try_into()
                .unwrap(),
        ),
    )?;
    unsafe {
        shader.draw_prepared(0..indices.len(), GeometryMode::Triangles);
    }

    // Green cube (back)
    shader.set_uniform("color", UniformValue::Vector4([0.0, 1.0, 0.0, 1.0]))?;
    shader.set_uniform(
        "model",
        UniformValue::Matrix4(
            glm::translate(&glm::identity(), &glm::vec3(-1.25, 0.0, 1.25))
                .as_slice()
                .try_into()
                .unwrap(),
        ),
    )?;
    unsafe {
        shader.draw_prepared(0..indices.len(), GeometryMode::Triangles);
    }

    // Blue cube (right)
    shader.set_uniform("color", UniformValue::Vector4([0.0, 0.0, 1.0, 1.0]))?;
    shader.set_uniform(
        "model",
        UniformValue::Matrix4(
            glm::translate(&glm::identity(), &glm::vec3(-1.0, 0.0, -1.25))
                .as_slice()
                .try_into()
                .unwrap(),
        ),
    )?;
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
