use glium::Surface;
use glium::uniform;

use super::graphical::*;
use crate::processing::material::*;
use crate::processing::objects::*;
use crate::misc::Similarity;


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


    pub fn draw_2D(
	&mut self,
	gr: &Graphical,
	(x, y, w, h): (f32, f32, f32, f32),
	depth: f32
	    
    )
    {
	use crate::processing::vertex::Vertex;
	use glium::vertex::VertexBuffer;

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let mesh = vec![
	    Vertex{position: [x, y, depth], .. Default::default()},
	    Vertex{position: [x+w, y, depth], .. Default::default()},
	    Vertex{position: [x+w, y+h, depth], .. Default::default()},
	    Vertex{position: [x, y, depth], .. Default::default()},
	    Vertex{position: [x, y+h, depth], .. Default::default()},
	    Vertex{position: [x+w, y+h, depth], .. Default::default()},
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


	let view_matrix: [[f32; 4]; 4] =  [[1., 0., 0., 0.],
			    [0., 1., 0., 0.],
			    [0., 0., 1., 0.],
			    [0., 0., 0., 1.]];
        self.frame
            .draw(
                (&vbo, instance.per_instance().unwrap()),
                &indices,
                &gr.program.program_default,
                &uniform! {
		    view_matrix: view_matrix
                },
                &gr.parameters.parameters,
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
        obj.groups
            .iter()
            .for_each(|(vertexes, material)|
                      self.draw_group(gr, vertexes, per_instance, material)
            );
    }
    
    /// Draws a group of Object (part of the Object) in the Frame, called by fn draw
    pub fn draw_group(
        &mut self,
        gr: &Graphical,
        vertex_buffer: &glium::vertex::VertexBufferAny,
        per_instance: &glium::VertexBuffer<Similarity>,
        material: &Material,
    ) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        match material {
            Material::Textured {
                texture,
                specular_color,
                specular_exponent,
                opacity,
            } => {
                self.frame
                    .draw(
                        (vertex_buffer, per_instance.per_instance().unwrap()),
                        indices,
                        gr.program.programs.get("program_textured").unwrap(),
                        &uniform! {
                            texture: texture,
                            view_matrix: gr.camera.get_view_matrix(),
                            perspective_matrix: gr.camera.get_perspective_matrix(),

                            specular_color: *specular_color,
                            specular_exponent: *specular_exponent,
                            opacity: *opacity

                        },
                        &gr.parameters.parameters,
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
                        gr.program.programs.get("program_nontextured").unwrap(),
                        &uniform! {view_matrix: gr.camera.get_view_matrix(),
                                   perspective_matrix: gr.camera.get_perspective_matrix(),
                                   ambiant: *ambiant_color,
                                   diffuse: *diffuse_color,
                                   specular: *specular_color,
                                   specular_exponent: *specular_exponent,
                                   emission: *emission_color,
                                   opacity: *opacity
                        },
                        &gr.parameters.parameters,
                    )
                    .unwrap();
            }
            _ => {
                self.frame
                    .draw(
                        (vertex_buffer, per_instance.per_instance().unwrap()),
                        indices,
                        gr.program.programs.get("program_default").unwrap(),
                        &uniform! {view_matrix: gr.camera.get_view_matrix() },
                        &gr.parameters.parameters,
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
