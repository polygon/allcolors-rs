use glium;

pub trait DisplayProgram {
    fn update(&mut self) -> Vec<Vec<(u8, u8, u8)>>;
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub fn run<T: 'static + DisplayProgram>(mut display_program: T) {
    use glium::{glutin, Surface};

    let events_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let shape = vec![
        Vertex { position: [-1.0, 1.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [1.0, 1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [1.0, -1.0], tex_coords: [0.0, 1.0] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    events_loop.run(move |event, _, control_flow| {
        match event {
          glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
              *control_flow = glutin::event_loop::ControlFlow::Exit;
              return;
            },
            _ => (),
          },
          _ => (),
        }

        let texture = glium::texture::Texture2d::new(&display, display_program.update()).unwrap();    
        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 1.0);
        let uniforms = uniform!{
            texture: &texture,
        };
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
