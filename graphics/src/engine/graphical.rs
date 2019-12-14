use super::camera::*;
use super::frame::*;

pub struct Graphical
{
    pub display: glium::Display,
    pub program_textured: glium::Program,
    pub program_nontextured: glium::Program,
    pub program_default: glium::Program,
    pub camera: Camera
}

impl Graphical
{

    pub fn new() -> Self
    {
        let event_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        // les shaders, toussa
        let program_textured = glium::Program::from_source(
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
            uniform mat4 perspective_matrix;


            void main() {
                v_tex_coords = texture;
                v_position = position;
                v_normal = normal;
                v_color = vec3(float(gl_InstanceID) / 10000.0, 1.0, 1.0);
                gl_Position = perspective_matrix*view_matrix*vec4(position * 0.0005 + world_position, 1.0);
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

        let program_nontextured = glium::Program::from_source(
            &display,
            "
            #version 140

            in vec3 position;
            in vec3 normal;
            in vec3 world_position;

            out vec3 v_position;
            out vec3 v_normal;
            out vec4 v_color;

            uniform mat4 view_matrix;
            uniform mat4 perspective_matrix;

            uniform vec3 ambiant; // remarque, ce n'est pas le rôle de l'objet de savoir l'ambiance
            uniform vec3 diffuse;
            uniform vec3 specular;
            uniform float specular_exponent;
            uniform vec3 emission;
            uniform float opacity;

            const vec3 light_direction = normalize(vec3(0., 1., 1.));
            const vec3 light_color = vec3(1., 1., 0.9);

            void main() {
                float diffusion = max(dot(normalize(normal), light_direction), 0.);
                vec3 camera_dir = normalize(-position);
                vec3 half_direction = normalize(normalize(light_direction) + camera_dir);

                float spec = pow(max(dot(half_direction, normalize(normal)), 0.0), specular_exponent);
                v_position = position;
                v_normal = normalize(normal);
                v_color = vec4(ambiant*0.01 + diffuse*diffusion, opacity + specular*spec);
                gl_Position = perspective_matrix*view_matrix*vec4(position * 0.0005 + world_position, 1.0);
            }
        ",
            "
            #version 140
            uniform vec3 specular;

            in vec3 v_normal;
            in vec4 v_color;
            out vec4 f_color;


 
            void main() {
              f_color = v_color;
            }
        ",
            None).unwrap();



        let program_default = glium::Program::from_source(
            &display,
            "
            #version 140

            in vec3 position;
            in vec3 normal;
            in vec3 world_position;
            out vec3 v_position;

            uniform mat4 view_matrix;


            void main() {
                v_position = position;
                gl_Position = view_matrix*vec4(position * 0.0005 + world_position, 1.0);
            }
        ",
            "
            #version 140
            out vec4 f_color;


            void main() {
              f_color = vec4(255, 0, 255, 255);
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
            program_textured: program_textured,
            program_nontextured: program_nontextured,
            program_default: program_default,
            camera: Camera::new(2.0)
        }
    }

    
    pub fn frame(&mut self) -> Frame
    {
        Frame::new(self)
    }

    pub fn update_dimensions(&mut self)
    {
        let (w, h) = self.display.get_framebuffer_dimensions();
        println!("{} {} {}", w, h, (w as f32)/(h as f32));
        self.camera.set_aspect_ratio(w as f32, h as f32);
    }

}
