use glium::Surface;

use super::graphical::*;
use crate::processing::material::*;
use crate::processing::objects::*;

#[derive(Copy, Clone)]
pub struct Attr {
    pub world_transformation: [[f32; 4]; 4],
}
implement_vertex!(Attr, world_transformation);

pub struct Frame {
    frame: glium::Frame,
}

impl Frame {
    /** Constructor of Frame  */
    pub fn new(gr: &Graphical) -> Self {
        Self {
            frame: gr.display.display.draw(),
        }
    }

    /** Draw an Object in the Frame at positions contained by per_instance, call fn draw_group for each group of Object */
    pub fn draw(
        &mut self,
        gr: &Graphical,
        obj: &Object,
        per_instance: &glium::VertexBuffer<Attr>, // position
    ) {
        obj.groups
            .iter()
            .for_each(|(vertexes, material)|
                      self.draw_group(gr, vertexes, per_instance, material)
            );
    }
    
    /** Draw a group of Object (part of the Object) in the Frame, called by fn draw */
    pub fn draw_group(
        &mut self,
        gr: &Graphical,
        vertex_buffer: &glium::vertex::VertexBufferAny,
        per_instance: &glium::VertexBuffer<Attr>,
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
                        &gr.program.program_textured,
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
                        &gr.program.program_nontextured,
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
                        &gr.program.program_default,
                        &uniform! {view_matrix: gr.camera.get_view_matrix() },
                        &gr.parameters.parameters,
                    )
                    .unwrap();
            }
        }
    }

    /** Reset the Frame */
    pub fn clear(&mut self) {
        self.frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
    }

    /** Send the Frame to the Graphical Card */
    pub fn show(self) {
        self.frame.finish().unwrap();
    }
}
