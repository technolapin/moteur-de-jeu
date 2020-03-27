use glium::Surface;
use glium::uniform;

use super::graphical::*;
use super::params::*;
use crate::ressource_handling::{Object, Material};
use crate::misc::Similarity;
use super::programs::ProgramId;

/**
Where the scene is being constructed.
Destroyed uppon displaying.
*/
pub struct Frame {
    frame: glium::Frame,
}

impl Frame {
    /// Constructor of Frame
    pub fn new(gr: &Graphical) -> Self {
        Self {
            frame: gr.display.display.draw(),
        }
    }


    /// draws a simple image on top of the render
    pub fn draw_image_2d(
	&mut self,
	gr: &Graphical,
	(x, y, w, h): (f32, f32, f32, f32),
	depth: f32,
	texture: &glium::Texture2d,
	program: ProgramId
    )
    {
	
	unsafe {texture.generate_mipmaps();} // necessary to bind the texture
	use crate::ressource_handling::vertex::Vertex;
	use glium::vertex::VertexBuffer;

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
	
	let mesh = vec![
	    Vertex{position: [x, y, depth], texture: [0., 0.], .. Default::default()},
	    Vertex{position: [x+w, y, depth], texture: [1., 0.], .. Default::default()},
	    Vertex{position: [x+w, y+h, depth], texture: [1., 1.], .. Default::default()},
	    Vertex{position: [x, y, depth], texture: [0., 0.], .. Default::default()},
	    Vertex{position: [x, y+h, depth], texture: [0., 1.], .. Default::default()},
	    Vertex{position: [x+w, y+h, depth], texture: [1., 1.], .. Default::default()},
	];
	let instance = glium::vertex::VertexBuffer::dynamic(
            &gr.display.display,
            &vec![Similarity {
		world_transformation: [[1., 0., 0., 0.],
				       [0., 1., 0., 0.],
				       [0., 0., 1., 0.],
				       [0., 0., 0., 1.]]
            }],
	).unwrap();
	
	let vbo = VertexBuffer::new(&gr.display.display, &mesh).unwrap();


	let params = Params::new()
	    .always_top();
	    
	

        self.frame
            .draw(
                (&vbo, instance.per_instance().unwrap()),
                &indices,
                &gr.program.get(program).unwrap(),
                &uniform! {
                    texture: texture,
                },
                &params.parameters,
            )
            .unwrap();
    }


    /** Draws several instances of an Object in the Frame using the similarities contained by the VBO per_instance.
    Calls fn draw_group for each group of Object.
     */
    pub fn draw(
        &mut self,
        gr: &Graphical,
        obj: &Object,
        per_instance: &glium::VertexBuffer<Similarity>, // position
    ) {
        obj.data
            .iter()
            .for_each(|(group, program)|
                      self.draw_group(gr, &group.vertexes, per_instance, &group.material, gr.program.get(*program).unwrap(), &obj.params)
            );
    }
    
    /// Draws a group of Object (part of the Object) in the Frame, called by fn draw, with a specific material and program
    pub fn draw_group(
        &mut self,
        gr: &Graphical,
        vertex_buffer: &glium::vertex::VertexBufferAny,
        per_instance: &glium::VertexBuffer<Similarity>,
        material: &Material,
	program: &glium::Program,
        params: &Params
    ) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        match material {
            Material::Textured {
                texture,
                specular_color,
                specular_exponent,
                opacity,
            } => {
		unsafe {texture.generate_mipmaps();} // binds the texture
                self.frame
                    .draw(
                        (vertex_buffer, per_instance.per_instance().unwrap()),
                        indices,
                        program,
                        &uniform! {
                            texture: texture,
                            view_matrix: gr.camera.get_view_matrix(),
                            perspective_matrix: gr.camera.get_perspective_matrix(),

                            specular_color: *specular_color,
                            specular_exponent: *specular_exponent,
                            opacity: *opacity

                        },
                        &params.parameters,
                    )
                    .unwrap();
            }
	    
            Material::NonTextured {
                ambiant_color,
                diffuse_color,
                specular_color,
                specular_exponent,
                emission_color,
                opacity,
            } => {
                self.frame
                    .draw(
                        (vertex_buffer, per_instance.per_instance().unwrap()),
                        indices,
                        program,
                        &uniform! {view_matrix: gr.camera.get_view_matrix(),
                                   perspective_matrix: gr.camera.get_perspective_matrix(),
                                   ambiant: *ambiant_color,
                                   diffuse: *diffuse_color,
                                   specular: *specular_color,
                                   specular_exponent: *specular_exponent,
                                   emission: *emission_color,
                                   opacity: *opacity
                        },
                        &params.parameters,
                    )
                    .unwrap();
            }
            _ => {
                self.frame
                    .draw(
                        (vertex_buffer, per_instance.per_instance().unwrap()),
                        indices,
                        program,
                        &uniform! {view_matrix: gr.camera.get_view_matrix() },
                        &params.parameters,
                    )
                    .unwrap();
            }
        }
    }

    
    /// Resets the Frame
    pub fn clear(&mut self) {
        self.frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
    }

    /**
    Sends the Frame to the Graphical Card.
    The frame then cannot be used anymore
     */
    pub fn show(self) {
        self.frame.finish().unwrap();
    }
}
