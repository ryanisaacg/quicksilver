use blinds::*;
use golem::{
    Attribute, AttributeType, Context,
    Dimension::{D2, D4},
    ElementBuffer, GeometryMode, GolemError, ShaderDescription, ShaderProgram, VertexBuffer,
};

// The application loop, powered by the blinds crate
async fn app(
    window: Window,
    ctx: golem::glow::Context,
    mut events: EventStream,
) -> Result<(), GolemError> {
    // Create a context from 'glow', GL On Whatever
    let ctx = &Context::from_glow(ctx)?;

    #[rustfmt::skip]
    // This is the data that represents the triangle
    // It's arranged how it will be passed to the GPU: each position represented as two f32 values,
    // followed by each color represented as 4 f32 values. The positions are on a scale from -1.0
    // to 1.0, which represents the viewport in OpenGL. The colors are represented as R, G, B, A,
    // on a scale from 0.0 to 1.0
    let vertices = [
        // Position         Color
        -0.5, -0.5,         1.0, 0.0, 0.0, 1.0,
        0.5, -0.5,          0.0, 1.0, 0.0, 1.0,
        0.0, 0.5,           0.0, 0.0, 1.0, 1.0
    ];
    // This is the data that indicates how to draw the vertices
    // For a simple example of one triangle, we don't gain much from this. Any order of these three
    // points will give us the same triangle. However, if we add more points (to draw a square, for
    // example), then we can write each point once while using it in multiple triangles.
    let indices = [0, 1, 2];

    // Here we create the ShaderProgram, which is some code that runs on the GPU. It determines how
    // to turn our vertex data into an actual vertex that GL understands, and how to color each
    // 'fragment' (essentially a pixel). These are each their own little program, where the
    // information from the vertex shader is fed into the fragment shader.
    // For the purposes of making sure the shaders match, and for ensuring compatibility on desktop
    // and web, the inputs are represented as data structures and then converted to shader
    // declarations at runtime.
    // The input to the shader program is fed to the vertex_input, so your vertex data's format
    // needs to match what you define in vertex_input
    let mut shader = ShaderProgram::new(
        ctx,
        ShaderDescription {
            // Take in to the shader a position (as a vector with 2 components) and a color (as a
            // vector with 4 components). This is the same format as 'vertices' above
            vertex_input: &[
                Attribute::new("vert_position", AttributeType::Vector(D2)),
                Attribute::new("vert_color", AttributeType::Vector(D4)),
            ],
            // Pass to the fragment shader the color
            // OpenGL will actually smoothly interpolate between different vertex values for us, so
            // a red vertex and a blue vertex will have a gradient between them
            fragment_input: &[Attribute::new("frag_color", AttributeType::Vector(D4))],
            // Uniforms represent a value that's the same for the entire shader; we don't need any
            // here. If you're rendering images or applying transformations to your entire draw
            // call, use uniforms!
            uniforms: &[],
            // A program written in GLSL that uses the inputs and outputs defined above
            // There's also a hard-coded output called gl_Position
            vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
            frag_color = vert_color;
        }"#,
            // The fragment shader has a hard-coded output: gl_FragColor
            fragment_shader: r#" void main() {
            gl_FragColor = frag_color;
        }"#,
        },
    )?;

    // Create buffer objects, which we use to transfer data from the CPU to the GPU
    let mut vb = VertexBuffer::new(ctx)?;
    let mut eb = ElementBuffer::new(ctx)?;
    // Set the data of the buffer to be our vertices and indices from earlier
    vb.set_data(&vertices);
    eb.set_data(&indices);
    // Prepare the shader for operations: shaders will raise errors if you forget to bind them
    shader.bind();
    // Clear the screen
    ctx.clear();
    unsafe {
        // Using our buffers, draw our triangle
        // We could also interpert our indices as Lines or a variety of other shape options:
        // nothing binds us to necessarily using Triangles, even though they're the most common
        // shape in graphics
        shader.draw(&vb, &eb, 0..indices.len(), GeometryMode::Triangles)?;
    }
    // Show our data to the window
    window.present();
    // Keep the window open and responsive until the user exits
    loop {
        events.next_event().await;
    }
}

// Run our application!
fn main() {
    run_gl(Settings::default(), |window, gfx, events| async move {
        app(window, gfx, events).await.unwrap()
    });
}
