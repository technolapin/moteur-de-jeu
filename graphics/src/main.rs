#[macro_use]
extern crate glium;
extern crate rand;

#[allow(unused_imports)]
use glium::{glutin, Surface};


fn load_wavefront(display: &glium::Display,
                  data: &[u8]) -> glium::vertex::VertexBufferAny
{
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        texture: [f32; 2],
    }

    implement_vertex!(Vertex, position, normal, texture);

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::Obj::load_buf(&mut data).unwrap();

    let mut vertex_data = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
                    for v in [v1, v2, v3].iter() {
                        let position = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);

                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        vertex_data.push(Vertex {
                            position: position,
                            normal: normal,
                            texture: texture,
                        })
                    }
                },
                _ => unimplemented!()
            }
        }
    }


    

    glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap()
        .into_vertex_buffer_any()
}









fn normalize_vec(v: (f32, f32, f32)) -> (f32, f32, f32)
{
    let norm = (v.0*v.0 + v.1*v.1 * v.2*v.2).sqrt();
    (v.0/norm, v.1/norm, v.2/norm)
}

fn v_prod(u: (f32, f32, f32), v: (f32, f32, f32)) -> (f32, f32, f32)
{
    (
        u.1*v.2 - u.2*v.1,
        -u.0*v.2 + u.2*v.0,
        u.0*v.1 - u.1*v.0
    )
}

#[derive(Copy, Clone)]
struct Attr {
    world_position: (f32, f32, f32),
}
implement_vertex!(Attr, world_position);

#[derive(Default)]
struct Camera
{
    position: (f32, f32, f32),
    orientation: (f32, f32, f32),
    aspect_ratio: f32
}

impl Camera
{
    fn new(aspect_ratio: f32) -> Self
    {
        Self
        {
            position: (0., 0., 0.),
            orientation: (0., 0., -1.),
            aspect_ratio: aspect_ratio
        }
    }

    fn set_position(&mut self, position: (f32, f32, f32))
    {
        self.position = position;
    }
    fn set_direction(&mut self, orientation: (f32, f32, f32))
    {
        self.orientation = normalize_vec(orientation);
    }
    
    fn get_view_matrix(&self) -> [[f32; 4]; 4]
    {
        let f = self.orientation;
        let u = (0., 1., 0.);

        let s = normalize_vec(v_prod(f, u));
        let v =  v_prod(s, f);

        [
            [s.0, u.0, f.0, 0.],
            [s.1, u.1, f.1, 0.],
            [s.2, u.2, f.2, 0.],
            [0., 0., 0., 0.],
        ]
        
    }
    
}



fn main() {

    let mut graphics = Graphical::new();

    // lit le .obj
    // building the vertex and index buffers
    let vertex_buffer = load_wavefront(&graphics.display,
                                                include_bytes!("teapot.obj"));

    // list of teapots with position and direction
    let mut teapots = (0 .. 10000)
        .map(|_| {
            let pos: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
            let dir: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
            let pos = (pos.0 * 1.5 - 0.75, pos.1 * 1.5 - 0.75, pos.2 * 1.5 - 0.75);
            let dir = (dir.0 * 1.5 - 0.75, dir.1 * 1.5 - 0.75, dir.2 * 1.5 - 0.75);
            (pos, dir)
        })
        .collect::<Vec<_>>();

    // building the vertex buffer with the attributes per instance
    // contient les positions des objets instanciés
    let mut per_instance = {


        // créé un vecteur de 10000 vertex (un point par object)
        let data = teapots.iter().map(|_| {
            Attr
            {
                world_position: (0.0, 0.0, 0.0),
            }
        }).collect::<Vec<_>>();

        glium::vertex::VertexBuffer::dynamic(&graphics.display, &data).unwrap()
    };




    // the main loop
    loop
    {
        // updating the teapots
        {
            let mut mapping = per_instance.map();
            for (src, dest) in teapots.iter_mut().zip(mapping.iter_mut()) {
                (src.0).0 += (src.1).0 * 0.001;
                (src.0).1 += (src.1).1 * 0.001;
                (src.0).2 += (src.1).2 * 0.001;

                dest.world_position = src.0;
            }
        }
        

        graphics.draw(&vertex_buffer,
                      &per_instance);
        
    }
}


struct Graphical
{
    display: glium::Display,
    program: glium::Program,
    camera: Camera
}




impl Graphical
{
    fn new() -> Self
    {
        let event_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        // les shaders, toussa
        let program = glium::Program::from_source(
            &display,
            "
            #version 140

            in vec3 position;
            in vec3 normal;
            in vec3 world_position;
            out vec3 v_position;
            out vec3 v_normal;
            out vec3 v_color;

            void main() {
                v_position = position;
                v_normal = normal;
                v_color = vec3(float(gl_InstanceID) / 10000.0, 1.0, 1.0);
                gl_Position = vec4(position * 0.0005 + world_position, 1.0);
            }
        ",
            "
            #version 140

            in vec3 v_normal;
            in vec3 v_color;
            out vec4 f_color;

            const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

            void main() {
                float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                vec3 color = (0.3 + 0.7 * lum) * v_color;
                f_color = vec4(color, 1.0);
            }
        ",
            None)
            .unwrap();

        Self
        {
            display: display,
            program: program,
            camera: Camera::new(1.0)
        }
    }

    
    fn draw(&mut self,
            vertex_buffer: &glium::vertex::VertexBufferAny, // le buffer de l'objet
            per_instance: &glium::VertexBuffer<Attr>) // les positions des instanciations de l'objet
    {

        // NoIndices est le type d'indices qu'on utilise quand on a pas besoin d'indices
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // drawing a frame
        let params = glium::DrawParameters
        {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess, // si c'est devant
                write: true, // alors on dessine
                .. Default::default()
            },
            .. Default::default()
        };

        let mut target = self.display.draw();
        // remet tout en noir et la distance depth à 1.0
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        target.draw( (vertex_buffer, per_instance.per_instance().unwrap()),
                      indices,
                      &self.program,
                      &uniform! { matrix: self.camera.get_view_matrix() },
                      &params
        ).unwrap();
        
        target.finish().unwrap();
        
        
    }


    
}
    
