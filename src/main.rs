extern crate gl;
extern crate sdl2;

mod glerror;
mod shader;

const VERT_SRC: &'static str = include_str!("./shaders/mandel.vert");
const FRAG_SRC: &'static str = include_str!("./shaders/mandel.frag");

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_system = sdl_context.video().unwrap();
    let gl_attr = video_system.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_framebuffer_srgb_compatible(true);

    let window = video_system
        .window(env!("CARGO_PKG_NAME"), 800, 600)
        .resizable()
        .opengl()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_system.gl_get_proc_address(s) as *const _);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut should_quit = false;

    let shader = shader::Shader::from_vert_frag(VERT_SRC, FRAG_SRC);

    let positions: Vec<[f32; 2]> = vec![
        [-1., -1.],
        [1., -1.],
        [1., 1.],
        [1., 1.],
        [-1., 1.],
        [-1., -1.],
    ];

    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        use gl::types::*;
        use std::mem::{size_of, transmute};

        gl::CreateVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::CreateBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (positions.len() * size_of::<f32>() * 2) as GLsizeiptr,
            transmute(&positions[0]),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            2 * size_of::<f32>() as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    let mut pos = [0.0, 0.0];
    let mut scale = 1.0;

    let mut should_redraw = true;
    while !should_quit {
        for e in event_pump.poll_iter() {
            use sdl2::event::Event;
            match e {
                Event::Quit { .. } => {
                    should_quit = true;
                }
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        if key.name() == "Escape" {
                            should_quit = true;
                        }
                    }
                }
                Event::MouseMotion {
                    mousestate,
                    xrel,
                    yrel,
                    ..
                } => {
                    if mousestate.left() {
                        pos[0] += xrel as f32 * scale;
                        pos[1] -= yrel as f32 * scale;
                        should_redraw = true;
                    }
                }
                Event::MouseWheel { y, .. } => {
                    if y != 0 {
                        should_redraw = true;
                        if y > 0 {
                            scale *= 1.25;
                        } else {
                            scale /= 1.25;
                        }
                    }
                }
                Event::Window { win_event, .. } => {
                    use sdl2::event::WindowEvent;
                    match win_event {
                        WindowEvent::SizeChanged(..) => {
                            should_redraw = true;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if should_redraw {
            shader.activate();
            unsafe {
                let [x, y] = pos;
                let window_size = window.drawable_size();
                gl::Uniform2f(1, x, y);
                gl::Uniform1f(2, scale);
                gl::Uniform2i(3, window_size.0 as i32, window_size.1 as i32);
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }

            window.gl_swap_window();
            should_redraw = false;
            assert_no_gl_error!();
        }
    }
    assert_no_gl_error!();
}
