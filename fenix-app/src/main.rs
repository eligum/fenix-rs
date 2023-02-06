use fenix_core::logging;
use fenix_renderer::{
    buffer::{IndexBuffer, VertexBuffer},
    shader::ShaderProgram,
};
use glam::{Mat4, Vec3};
use glfw::{Action, Context, CursorMode, Key, OpenGlProfileHint, WindowHint};
use log::{error, info, trace, warn, LevelFilter};
use std::ffi::c_void;
use std::mem::size_of;

fn main() {
    logging::setup("fenix.log", LevelFilter::Trace).expect("failed to initialize logging");

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    info!("GLFW version: {:?}", glfw::get_version_string());

    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::ContextVersion(4, 5));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::Resizable(true));

    #[cfg(debug_assertions)]
    {
        glfw.window_hint(WindowHint::OpenGlDebugContext(true));
        warn!("Using an OpenGL debug context, GL operations will be significantly slower.");
        warn!("If this is not intentional you should request a normal context.");
    }

    let (mut window, events) = glfw
        .create_window(1280, 720, "Fenix - Editor", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const c_void);

    #[rustfmt::skip]
    const VERTICES: [f32; 108] = [
        // back face
        0.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        0.0, 0.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 0.0, 0.0,
        // right face
        1.0, 0.0, 1.0,
        1.0, 0.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 0.0, 1.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 1.0,
        // front face
        0.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 1.0, 1.0,
        0.0, 0.0, 1.0,
        1.0, 1.0, 1.0,
        0.0, 1.0, 1.0,
        // left face
        0.0, 0.0, 0.0,
        0.0, 0.0, 1.0,
        0.0, 1.0, 1.0,
        0.0, 0.0, 0.0,
        0.0, 1.0, 1.0,
        0.0, 1.0, 0.0,
        // top face
        0.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 0.0,
        0.0, 1.0, 1.0,
        1.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        // bottom face
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 1.0,
        0.0, 0.0, 0.0,
        1.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
    ];

    let mut vao = 0;


    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    let vertex_buff = VertexBuffer::from(&VERTICES);
    vertex_buff.bind();

    unsafe {
        // gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // gl::BufferData(
        //     gl::ARRAY_BUFFER,
        //     size_of_val(VERTICES) as isize,
        //     VERTICES.as_ptr() as *const c_void,
        //     gl::STATIC_DRAW,
        // );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of::<f32>() as i32,
            0 as *const c_void,
        );

        gl::EnableVertexAttribArray(0);
    }

    let mut program = ShaderProgram::from_file("assets/shader.vert", "assets/shader.frag", None)
        .unwrap_or_else(|err| {
            error!("{}", err);
            warn!("Using empty ShaderProgram");
            ShaderProgram::new().unwrap()
        });
    program.bind();

    unsafe {
        gl::Viewport(0, 0, 1280, 720);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    while !window.should_close() {
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        let model = Mat4::from_translation(Vec3::new(-0.5, -0.5, -3.0));
        let view = Mat4::IDENTITY;
        let projection = Mat4::perspective_rh(45.0, 16.0 / 9.0, 0.1, 10.0);

        program.set_uniform_mat4("u_model", model);
        program.set_uniform_mat4("u_view", view);
        program.set_uniform_mat4("u_projection", projection);

        // Draw frame to buffer
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, VERTICES.len() as i32);
            gl::BindVertexArray(0);
        }

        // Swap front and back buffers
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
            trace!("Key SPACE pressed.");
            match window.get_cursor_mode() {
                CursorMode::Normal => window.set_cursor_mode(CursorMode::Disabled),
                CursorMode::Disabled => window.set_cursor_mode(CursorMode::Normal),
                _ => {},
            }
        },
        _ => {},
    }
}
