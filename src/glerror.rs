#![macro_use]

fn error_string(error: gl::types::GLuint) -> &'static str {
    match error {
        gl::NO_ERROR => "GL_NO_ERROR",
        gl::INVALID_ENUM => "GL_INVALID_ENUM",
        gl::INVALID_VALUE => "GL_INVALID_VALUE",
        gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
        gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
        gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
        gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
        gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
        _ => "Not a valid GLerror",
    }
}

fn print_one_error() -> bool {
    let error = unsafe { gl::GetError() };
    match error {
        gl::NO_ERROR => false,
        _ => {
            eprintln!("GL error: {}", error_string(error));
            true
        }
    }
}

pub fn print_gl_errors() -> bool {
    if print_one_error() {
        loop {
            if !print_one_error() {
                return true;
            }
        }
    }

    false
}

macro_rules! assert_no_gl_error {
    () => {
        if glerror::print_gl_errors() {
            panic!("expected no GL errors")
        }
    };
}
