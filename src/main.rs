mod shader;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

const TITLE : &str = "Project Bahamut";



fn main() {
    // SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // let display = video.current_display_mode(0).unwrap();

    let window = video
        .window(TITLE, 800, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    // OpenGL
    let gl = video.gl_attr();
    gl.set_context_profile(GLProfile::Core);
    gl.set_context_version(4, 6);

    let _gl_ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl.context_version(), (4, 6));

    let sdl_events = &mut sdl_context.event_pump().unwrap();

    'running: loop {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();

        for event in sdl_events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}