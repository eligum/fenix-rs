use fenix_core::logging;
use fenix_renderer::{
    buffer::{IndexBuffer, VertexBuffer},
    math,
    shader::ShaderProgram, texture::Texture2D,
};
use glam::{Mat4, Vec3};
use glfw::{Action, Context, CursorMode, Key, OpenGlProfileHint, WindowHint};
use log::{error, info, trace, warn, LevelFilter};
use std::mem::size_of;
use std::{ffi::c_void, ptr};

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

    // Set v-sync
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    // Load OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const c_void);

    // #[rustfmt::skip]
    // const VERTICES: [f32; 24] = [
    //     0.0, 0.0, 0.0,
    //     0.0, 0.0, 1.0,
    //     0.0, 1.0, 0.0,
    //     0.0, 1.0, 1.0,
    //     1.0, 0.0, 0.0,
    //     1.0, 0.0, 1.0,
    //     1.0, 1.0, 0.0,
    //     1.0, 1.0, 1.0,
    // ];

    // #[rustfmt::skip]
    // const INDICES: [u32; 36] = [
    //     1, 2, 3,
    //     2, 4, 3,
    //     2, 6, 4,
    //     6, 8, 4,
    //     6, 5, 8,
    //     5, 7, 8,
    //     5, 1, 7,
    //     1, 3, 7,
    //     4, 8, 3,
    //     8, 7, 3,
    //     1, 5, 2,
    //     5, 6, 2,
    // ];

    let container_tex = Texture2D::from_file("assets/image/container.jpg").unwrap();
    let awesome_tex = Texture2D::from_file("assets/image/awesomeface.png").unwrap();

    #[rustfmt::skip]
    const VERTICES: [f32; 5 * 4] = [
        0.0, 0.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 1.0, 0.0,
        1.0, 1.0, 0.0, 1.0, 1.0,
        0.0, 1.0, 0.0, 0.0, 1.0,
    ];

    #[rustfmt::skip]
    const INDICES: [u32; 3 * 2] = [
        0, 1, 2,
        0, 2, 3,
    ];

    let mut vao = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    let vertex_buff = VertexBuffer::from(&VERTICES);
    vertex_buff.bind();

    let index_buff = IndexBuffer::from(&INDICES);
    index_buff.bind();

    unsafe {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            0 as *const c_void,
        );

        gl::EnableVertexAttribArray(3);
        gl::VertexAttribPointer(
            3,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const c_void,
        );
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

        let model = Mat4::from_translation(Vec3::new(-0.5, -0.5, -0.5));
        let view = Mat4::from_translation(Vec3::new(0.0, 0.0, -3.0))
            * Mat4::from_axis_angle(
                Vec3::new(-1.0, 1.0, -1.0).try_normalize().unwrap(),
                math::radians(30.0),
            );
        let projection = Mat4::perspective_rh(45.0, 16.0 / 9.0, 0.1, 10.0);

        program.set_uniform_mat4("u_model", model);
        program.set_uniform_mat4("u_view", view);
        program.set_uniform_mat4("u_projection", projection);

        container_tex.bind(0);
        awesome_tex.bind(1);

        program.set_uniform_1i("color_map0", 0);
        program.set_uniform_1i("color_map1", 1);

        // Draw frame to buffer
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::TRIANGLES,
                index_buff.get_count() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
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
