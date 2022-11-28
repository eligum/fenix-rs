// #![allow(unused_imports)]

use fenix_core::logging::setup_logging;
use glfw::{Action, Context, Key};
use log::{error, info, warn, LevelFilter};
use std::mem::{size_of_val, size_of};
use std::os::raw::c_void;

fn main() {
    setup_logging("fenix.log", LevelFilter::Info).expect("failed to initialize logging");

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    info!("GLFW version: {:?}", glfw::get_version_string());

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::Resizable(true));

    let (mut window, events) = glfw
        .create_window(1280, 720, "Fenix Editor", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const c_void);

    #[rustfmt::skip]
    const VERTICES: &[f32] = &[
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
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(VERTICES) as isize,
            VERTICES.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
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
    
    const VERT_SHADER: &str = r#"#version 450 core
        layout (location = 0) in vec3 a_pos;
        void main() {
            gl_Position = vec4(a_pos, 1.0);
        }
    "#;
    const FRAG_SHADER: &str = r#"#version 450 core
        out vec4 frag_color;
        void main() {
            frag_color = vec4(1.0, 0.5, 0.0, 1.0);
        }
    "#;
    
    unsafe {
        // Vertex shader
        let vert_id = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vert_id,
            1,
            &(VERT_SHADER.as_bytes().as_ptr() as *const i8),
            &(VERT_SHADER.len() as i32),
        );
        gl::CompileShader(vert_id);
        let mut success = 0;
        gl::GetShaderiv(vert_id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let buf_size = 1024;
            let mut log_info: Vec<u8> = Vec::with_capacity(buf_size);
            let mut log_len: i32 = 0;
            gl::GetShaderInfoLog(
                vert_id,
                buf_size as i32,
                &mut log_len,
                log_info.as_mut_ptr() as *mut i8,
            );
            log_info.set_len(log_len as usize);
            error!("Failed to compile VERTEX shader:\n{}", String::from_utf8_lossy(&log_info));
        }

        // Fragment shader
        let frag_id = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            frag_id,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr() as *const i8),
            &(FRAG_SHADER.len() as i32),
        );
        gl::CompileShader(frag_id);
        let mut success = 0;
        gl::GetShaderiv(frag_id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let buf_size = 1024;
            let mut log_info: Vec<u8> = Vec::with_capacity(buf_size);
            let mut log_len: i32 = 0;
            gl::GetShaderInfoLog(
                frag_id,
                buf_size as i32,
                &mut log_len,
                log_info.as_mut_ptr() as *mut i8,
            );
            log_info.set_len(log_len as usize);
            error!("Failed to compile FRAGMENT shader:\n{}", String::from_utf8_lossy(&log_info));
        }
    
        // Link shader program
        let program_id = gl::CreateProgram();
        gl::AttachShader(program_id, vert_id);
        gl::AttachShader(program_id, frag_id);
        gl::LinkProgram(program_id);
        let mut success = 0;
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let buf_size = 1024;
            let mut log_info: Vec<u8> = Vec::with_capacity(buf_size);
            let mut log_len: i32 = 0;
            gl::GetProgramInfoLog(
                program_id,
                buf_size as i32,
                &mut log_len,
                log_info.as_mut_ptr() as *mut i8,
            );
            log_info.set_len(log_len as usize);
            error!("Failed to link PROGRAM shader:\n{}", String::from_utf8_lossy(&log_info));
        }
        gl::UseProgram(program_id); // TODO: Move this closer to the draw call
        
        // Delete shader source from GPU memory
        gl::DeleteShader(vert_id);
        gl::DeleteShader(frag_id);
    }

    unsafe {
        gl::Viewport(0, 0, 1280, 720);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
        }

        // Swap buffers and poll IO events
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
