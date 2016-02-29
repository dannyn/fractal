use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex: [f32; 2],
}


fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    implement_vertex!(Vertex, position, tex);

    let tris = vec![Vertex {position: [-1.0,1.0],  tex: [0.0,1.0]}, 
                    Vertex {position: [1.0,1.0],   tex: [1.0,1.0]},
                    Vertex {position: [1.0,-1.0],  tex: [1.0,0.0]},
                    Vertex {position: [-1.0,-1.0], tex: [0.0,0.0]}];
    let ilist:Vec<u16> = vec![0, 1, 2, 0, 2, 3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &tris).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &ilist).unwrap();

    let mut v_file = File::open("assests/shaders/mandelbrot.vert").unwrap();
    let mut vert_shader_src = String::new();
    v_file.read_to_string(&mut vert_shader_src).unwrap();

    let mut f_file = File::open("assests/shaders/mandelbrot.frag").unwrap();
    let mut frag_shader_src = String::new();
    f_file.read_to_string(&mut frag_shader_src).unwrap();

    let program = glium::Program::from_source(&display, &vert_shader_src, &frag_shader_src, None).unwrap();

    let mut x = 0.7f32; 
    let mut y = 0.0f32;
    let mut iter = 100;
    let mut scale = 2.0f32;

    loop {
        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform!{scale: scale, center: [x, y], iter: iter};
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Escape)) => return,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Up)) => y -= 0.1f32 * scale,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Down)) => y += 0.1f32 * scale,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Left)) => x += 0.1f32 * scale,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Right)) => x -=0.1f32 * scale,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::PageUp)) => iter += 5,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::PageDown)) => iter -= 5,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::X)) => scale = scale * 0.9f32,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Z)) => scale = scale / 0.9f32,
                _ => ()
            }
        }
    }
}
