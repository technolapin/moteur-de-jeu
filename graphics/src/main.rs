#[macro_use]
extern crate glium;
extern crate rand;

#[allow(unused_imports)]
use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2],
}

implement_vertex!(Vertex, position, normal, texture);



fn load_wavefront(display: &glium::Display,
                  data: &[u8]) -> glium::vertex::VertexBufferAny
{

    let mut data = ::std::io::BufReader::new(data); // on ouvre le fichier
    let mut data = obj::Obj::load_buf(&mut data).unwrap();
    data.load_mtls().unwrap(); // charge le mlt associé
   
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

use glium::vertex::{VertexBufferAny, VertexBuffer};
use glium::texture::{RawImage2d, Texture2d};


#[derive(Debug)]
struct Material
{
    texture: Texture2d
}

#[derive(Debug)]
struct Group
{
    voxels: VertexBufferAny,
    material: Option<String>
}

#[derive(Debug)]
struct Objects
{
//    meshes: Vec<(VertexBufferAny, Option<String>)>,
    objects: HashMap<String, Vec<Group>>,
    materials: HashMap<String, Material>
}

use std::fs::File;
use obj::{Obj, Mtl};
use std::io::Cursor;
use std::collections::HashMap;



impl Objects
{
    fn new(gr: &Graphical, path_to_wavefront: &str, path_to_mtl: &str) -> Self
    {
        use genmesh::{Polygon, Triangle, Quad};
        
        
        
        let file = File::open(path_to_wavefront).unwrap();
        let mut bufreader = ::std::io::BufReader::new(file);
        let mut obj = Obj::load_buf(&mut bufreader).unwrap();
        obj.load_mtls().unwrap();
        
        let file = File::open(path_to_mtl).unwrap();
        let mut bufreader = ::std::io::BufReader::new(file);
        
        let mtl = Mtl::load(&mut bufreader);
        
        //println!("{}", mtl.materials);
        /*
        let ambient_color = None; // Ka
        let diffuse_color = None; // Kd
        let specular_color = None; // Ks
        let emissive = None; // Ke
        // Km?
        // Tf?
        let specular_exponent = None; // Ns
        // Ni?
        let opacity = None; // d or Tr (d = 1-Tr)
        let illumination = None; // illum

        let map_ambiant_color = None;
        let map_diffuse_color = None;
        let map_specular_color = None;
        let map_emissive = None;
        let map_specular_exponent = None;
        let map_opacity = None;
        let map_bump = None;
        let map_reflexion = None;
         */
        let mut materials = HashMap::new();
        
        
        for material in mtl.materials.iter()
        {
            let texture = if let Some(texture_path) = &material.map_kd
            {
                let file = File::open(texture_path).unwrap();
                let mut bufreader = ::std::io::BufReader::new(file);
                let image = image::load(&mut bufreader,
                                    image::PNG).unwrap().to_rgba();
                let image_dimensions = image.dimensions();
                let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
                let texture = Texture2d::new(&gr.display, image).unwrap();
                texture
            }
            else
            {
                Texture2d::empty(&gr.display, 1, 1).unwrap()
            };
            println!("MATERIAL: {}", material.name.clone());
            materials.insert(material.name.clone(), Material{texture: texture});
        }
                
        let mut objects = Objects{
            objects: HashMap::new(),
            materials: materials
        };
        
        println!("{:?}", obj.objects);
        
        for object in obj.objects.iter()
        {
            let mut groups = Vec::new();
            for group in object.groups.iter()
            {
                let mut mesh = Vec::new();
                
                
                for polygon in group.polys.iter()
                {
                    match polygon
                    {
                        &Polygon::PolyTri(
                            Triangle{x: v1, y: v2, z: v3}) =>
                        {
                            for v in [v1, v2, v3].iter()
                            {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex
                                          {
                                              position: position,
                                              normal: normal,
                                              texture: texture,
                                          })
                            
                            }
                        },
                        &Polygon::PolyQuad(
                            Quad{x: v1, y: v2, z: v3, w: v4}) =>
                        {
                            for v in [v1, v2, v3].iter()
                            {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex
                                          {
                                              position: position,
                                              normal: normal,
                                              texture: texture,
                                          })
                                    
                            }
                            for v in [v3, v4, v1].iter()
                            {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex
                                          {
                                              position: position,
                                              normal: normal,
                                              texture: texture,
                                          })
                                    
                            }
                        }

                    }
                }

                groups.push(
                    Group
                    {
                        voxels: VertexBuffer::new(&gr.display, &mesh).unwrap()
                            .into_vertex_buffer_any(),
                        material: match &group.material
                        {
                            Some(mat) => Some(mat.name.clone()),
                            None => None
                        }
                    }
                );
                
            }
            objects.objects.insert(object.name.clone(), groups);
        }

        objects

    }

    fn get_object(&self, name: String) -> Vec<(&VertexBufferAny, Option<&Material>)>
    {
        let groups = self.objects.get(&name).unwrap();

        groups.iter().map(|group|
                          {
                              (
                                  &group.voxels,
                                  match &group.material
                                  {
                                      None => None,
                                      Some(string) => self.materials.get(string)
                                  }
                               )
                          }
        ).collect::<Vec<_>>()
        
        
    }

}








fn normalize_vec(v: (f32, f32, f32)) -> (f32, f32, f32)
{
    let norm = (v.0*v.0 + v.1*v.1 + v.2*v.2).sqrt();
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
    up: (f32, f32, f32),
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
            up: (0., 1., 0.),
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

    fn rotation(&mut self, (rx, ry, rz): (f32, f32, f32))
    {
        //on tourne de rx rad autour de l'axe 0x
        //on tourne de ry rad autour de l'axe 0y
        //on tourne de rz rad autour de l'axe 0z

        let (x, y, z) = self.orientation;
        let (ux, uy, uz) = self.up;

        let (x, y, z) = ( x,
                          y*rx.cos() + z*rx.sin(),
                          -y*rx.sin() + z*rx.cos());
        let (ux, uy, uz) = ( ux,
                          uy*rx.cos() + uz*rx.sin(),
                          -uy*rx.sin() + uz*rx.cos());
        

        let (x, y, z) = ( x*ry.cos() - z*ry.sin(),
                          y,
                          x*ry.sin() + z*ry.cos());
        let (ux, uy, uz) = ( ux*ry.cos() - uz*ry.sin(),
                          uy,
                          ux*ry.sin() + uz*ry.cos());

        
        let (x, y, z) = ( x*rz.cos() + y*rz.sin(),
                          -x*rz.sin() + y*rz.cos(),
                          z);
        let (ux, uy, uz) = ( ux*rz.cos() + uy*rz.sin(),
                          -ux*rz.sin() + uy*rz.cos(),
                          uz);
        
        self.orientation = normalize_vec((x, y, z));
        self.up = (ux, uy, uz);
        
        
    }
    
    
    fn get_view_matrix(&self) -> [[f32; 4]; 4]
    {
        let f = self.orientation;
        //let u = (0., 1., 0.);
        //let u = normalize_vec((-f.1, -f.2, f.0));
        let u = self.up;
        
        let s = normalize_vec(v_prod(f, u));
        let v =  v_prod(s, f);
        let p = (
            -self.position.0*s.0 -self.position.1*s.1 -self.position.1*s.2,
            -self.position.0*u.0 -self.position.1*u.1 -self.position.1*u.2,
            -self.position.0*f.0 -self.position.1*f.1 -self.position.1*f.2
        );

        [
            [s.0, u.0, f.0, 0.0],
            [s.1, u.1, f.1, 0.0],
            [s.2, u.2, f.2, 0.0],
            [p.0, p.1, p.2, 1.0],
        ]

        
    }
    
}



fn main() {

    let mut graphics = Graphical::new();

    // lit le .obj
    // building the vertex and index buffers
    let teapot_vertex_buffer = load_wavefront(
        &graphics.display,
        include_bytes!("teapot.obj")
    );
    let teto_vertex_buffer = load_wavefront(
        &graphics.display,
        include_bytes!("teto.obj")
    );
    let textured_cube = load_wavefront(
        &graphics.display,
        include_bytes!("textured_cube.obj")
    );

    /*
    use std::io::Cursor;
    let image = image::load(Cursor::new(&include_bytes!("zelda.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&graphics.display, image).unwrap();
*/

    let kube = Objects::new(&graphics, "textured_cube.obj", "textured_cube.mtl");
    // let kube = Objects::new(&graphics, "teto.obj", "teto.mtl");
    
    println!("\nKube: {:?}", kube);
    
    // list of teapots with position and direction
    let mut teapots = (0 .. 100)
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



    let to_display = kube.get_object(String::from("Cube.001"));
    
    
    // the main loop
    loop
    {
        graphics.camera.rotation((0., 0.01, 0.001));
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
        

        let mut frame = graphics.frame();
        frame.clear();
        /*
        frame.draw(&graphics,
                   &teto_vertex_buffer,
                   &per_instance);
        frame.draw(&graphics,
                   &teapot_vertex_buffer,
                   &per_instance);
         */
        /*
        frame.draw(&graphics,
                   &textured_cube,
                   &per_instance, &texture);
*/
        to_display.iter().for_each(
            |(vertexes, maybe_material)|
            {
                match maybe_material
                {
                    Some(material) =>
                    {
                        frame.draw(&graphics,
                                   vertexes,
                                   &per_instance, &material.texture);
                    },
                    None => unimplemented!()
                }
            }
        );

        
        frame.show();
        
    }   

}


struct Graphical
{
    display: glium::Display,
    program: glium::Program,
    camera: Camera
}


struct Frame
{
    frame: glium::Frame
}

impl Frame
{
    fn new(gr: &Graphical) -> Self
    {
        Self
        {
            frame: gr.display.draw()
        }
    }
    fn draw(&mut self,
            gr: &Graphical,
            vertex_buffer: &glium::vertex::VertexBufferAny,
            per_instance: &glium::VertexBuffer<Attr>,
            texture: &glium::texture::Texture2d
    )
    {
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


        self.frame.draw( (vertex_buffer, per_instance.per_instance().unwrap()),
                          indices,
                          &gr.program,
                          &uniform! { texture: texture, view_matrix: gr.camera.get_view_matrix() },
                          &params
        ).unwrap();

        
    }
    
    fn clear(&mut self)
    {
        self.frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
    }
    fn show(self)
    {
        self.frame.finish().unwrap();
    }
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
            in vec2 texture;
            out vec2 v_tex_coords;
            out vec3 v_position;
            out vec3 v_normal;
            out vec3 v_color;

            uniform mat4 view_matrix;


            void main() {
                v_tex_coords = texture;
                v_position = position;
                v_normal = normal;
                v_color = vec3(float(gl_InstanceID) / 10000.0, 1.0, 1.0);
                gl_Position = view_matrix*vec4(position * 0.0005 + world_position, 1.0);
            }
        ",
            "
            #version 140

            in vec3 v_normal;
            in vec3 v_color;
            in vec2 v_tex_coords;
            out vec4 f_color;


            uniform sampler2D tex;

            void main() {
              f_color = texture(tex, v_tex_coords);
            }
        ",/*
            const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

            void main() {
                float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                vec3 color = (0.3 + 0.7 * lum) * v_color;
                f_color = vec4(color, 1.0);
            }
        ",*/
            None).unwrap();

        
        Self
        {
            display: display,
            program: program,
            camera: Camera::new(2.0)
        }
    }
    fn frame(&mut self) -> Frame
    {
        Frame::new(self)
    }
}
