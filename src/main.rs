#![allow(non_upper_case_globals)]
extern crate sdl2_sys; use self::sdl2_sys::*;
use std::ptr; use std::ffi; use std::mem;
mod gl { include!(concat!(env!("OUT_DIR"), "/bindings.rs")); }

fn main() {
    unsafe {
        if SDL_Init(SDL_INIT_VIDEO) < 0 {
            panic!("Could not initialize video subsystem");
        }

        SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 3);
        SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 3);
        let window: *mut SDL_Window = SDL_CreateWindow(
            ffi::CString::new("My Game Window").unwrap().as_ptr(),
            SDL_WINDOWPOS_UNDEFINED,
            SDL_WINDOWPOS_UNDEFINED,
            640, 480,
            SDL_WindowFlags::SDL_WINDOW_SHOWN as u32 |
            SDL_WindowFlags::SDL_WINDOW_OPENGL as u32);

        let gl_context: SDL_GLContext = SDL_GL_CreateContext(window);
        if gl_context == ptr::null_mut() {
            panic!("Couldn't create opengl context!");
        }

        SDL_GL_MakeCurrent(window, gl_context);

        gl::load_with(|s| SDL_GL_GetProcAddress(s.as_ptr() as _) as _);

        let version = gl::GetString(gl::VERSION as _);
        if version == ptr::null() {
            panic!("There was an error creating the opengl context!");
        }

        loop {
            let mut e: SDL_Event = mem::uninitialized();
            if SDL_PollEvent(&mut e) != 0 {
                if *e.type_() == SDL_QUIT {
                    break
                }
            }

            gl::ClearColor(1.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            SDL_GL_SwapWindow(window);
            SDL_Delay(20);
        }

        SDL_GL_DeleteContext(gl_context);
        SDL_Quit();
    }
}













