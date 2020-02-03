use std::path::Path;

use crate::misc::{read_file, get_ressources_path};


use super::Display;


/**
The various registered shaders programs.
For now it has some shaders hard-coded but those will eventually be dissmissed.
*/
pub struct Programs
{
    pub programs: Vec<glium::Program>,
    pub program_textured: glium::Program,
    pub program_nontextured: glium::Program,
    pub program_default: glium::Program,
}

impl Programs
{
    pub fn new(display: &Display) -> Self
    {	
	let mut program = Self
	{ 	programs: Vec::new(),

		program_textured: glium::Program::from_source(              
		    	&display.display,
		    	"
		    	#version 140

		    	in vec3 position;
			    in vec3 normal;
			    in mat4 world_transformation;
			    in vec2 texture;

			    out vec2 v_tex_coords;
			    out vec3 v_position;
			    out vec3 v_normal;
			    out vec4 v_color;

			    uniform mat4 view_matrix;
			    uniform mat4 perspective_matrix;

			    uniform vec3 specular;
			    uniform float specular_exponent;
			    uniform float opacity;

			    const vec3 light_direction = normalize(vec3(0., 1., -1.));
			    const vec3 light_color = vec3(1., 1., 0.9);

			    void main() {
				vec3 norm = normalize((world_transformation*vec4(normal, 0.)).xyz);
				vec3 camera_dir = normalize((-world_transformation*vec4(position, 1.0)).xyz);
				vec3 half_direction = normalize(light_direction + camera_dir);
				float diffusion = max(dot(norm, light_direction), -dot(norm, light_direction)*(1.-opacity));

				float spec = pow(max(dot(half_direction, norm), 0.0), specular_exponent);

				v_tex_coords = texture;
				v_position = position;
				v_normal = norm;
				v_color = vec4(specular*spec, opacity);
				gl_Position = perspective_matrix*view_matrix*world_transformation*vec4(position, 1.0);
			    }
			",
			    "
			    #version 140

			    in vec3 v_normal;
			    in vec4 v_color;
			    in vec2 v_tex_coords;
			    out vec4 f_color;

			    const vec3 light_direction = normalize(vec3(0., 1., -1.));
			    const vec3 light_color = vec3(1., 1., 0.9);

			    uniform sampler2D tex;

			    void main() {
			      float diffusion = max(dot(normalize(v_normal), light_direction), 0.01);
			      f_color = texture(tex, v_tex_coords)*diffusion + v_color;
			    }
			",/*
			    const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

			    void main() {
				float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
				vec3 color = (0.3 + 0.7 * lum) * v_color;
				f_color = vec4(color, 1.0);
			    }
			",*/
			    None).unwrap(),

		program_nontextured: glium::Program::from_source(
			    &display.display,
			    "
			    #version 140

			    in vec3 position;
			    in vec3 normal;
			    in mat4 world_transformation;

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
			    const vec3 light_direction = normalize(vec3(0., 1., -1.));

			    const vec3 light_color = vec3(1., 1., 0.9);

			    void main()
			    {
				vec3 norm = normalize((world_transformation*vec4(normal, 0.)).xyz);

				float diffusion = max(dot(norm, light_direction), 0.);
				vec3 camera_dir = normalize((-view_matrix[3]-world_transformation*vec4(position, 1.0)).xyz);
				vec3 half_direction = normalize(normalize(light_direction) + camera_dir);

				float spec = pow(max(dot(half_direction, norm), 0.0), specular_exponent);
				v_position = position;
				v_normal = norm;
				v_color = vec4(diffuse*0.01 + diffuse*diffusion + specular*spec, opacity);
				gl_Position = perspective_matrix*view_matrix*world_transformation*vec4(position, 1.0);
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
			    None).unwrap(),



  	      program_default: glium::Program::from_source(
			    &display.display,
			    "
			    #version 140

			    in vec3 position;
			    in vec3 normal;
			    in mat4 world_transformation;
			    out vec3 v_position;

			    uniform mat4 view_matrix;


			    void main() {
				v_position = position;
				gl_Position = view_matrix*world_transformation*vec4(position, 1.0);
			    }
			",
			    "
			    #version 140
			    out vec4 f_color;


			    void main() {
			      f_color = vec4(255, 0, 255, 255);
			    }
			",
			    None).unwrap(),

	};
	program.add_program(display, "textured"); 
	program
    }

    /**
    Loads a program from source files.
    The name of the program is the name of the folder containing its various shaders.
    */
    pub fn add_program(&mut self, display: &Display, program_name: &str)
    {

	let ressources_path = get_ressources_path();
	let shaders_path = ressources_path.join(Path::new("shaders"));
	let program_path = shaders_path.join(Path::new(program_name));

	let path_vertex = program_path.join(Path::new("vertex.glsl"));
	let path_fragment = program_path.join(Path::new("fragment.glsl"));
	let path_geometry = program_path.join(Path::new("geometry.glsl"));


	let pgrm = 
	if path_geometry.is_file()
	{	glium::Program::from_source(
			    &display.display,
			    read_file(path_vertex).as_str(),
			    read_file(path_fragment).as_str(),
			    Some(read_file(path_geometry).as_str()) )
		   .unwrap()
        } 
	else
	{	glium::Program::from_source(
			    &display.display,
			    read_file(path_vertex).as_str(),
			    read_file(path_fragment).as_str(),
			    None)
		   .unwrap()
	};

	self.programs.push(pgrm);
    }
}

