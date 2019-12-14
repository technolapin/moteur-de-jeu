use glium::Surface;

use super::graphical::*;

use crate::processing::material::*;

#[derive(Copy, Clone)]
pub struct Attr {
    pub world_position: (f32, f32, f32),
}
implement_vertex!(Attr, world_position);



pub struct Frame
{
    frame: glium::Frame
}

impl Frame
{
    pub fn new(gr: &Graphical) -> Self
    {
        Self
        {
            frame: gr.display.draw()
        }
    }
    
    pub fn draw(&mut self,
            gr: &Graphical,
            vertex_buffer: &glium::vertex::VertexBufferAny,
            per_instance: &glium::VertexBuffer<Attr>,
            material: &Material
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

        match material
        {
            Material::Textured{texture: texture} =>
            {
                self.frame.draw( (vertex_buffer, per_instance.per_instance().unwrap()),
                                  indices,
                                  &gr.program_textured,
                                  &uniform! {
                                      texture: texture,
                                      view_matrix: gr.camera.get_view_matrix(),
                                      perspective_matrix: gr.camera.get_perspective_matrix(),
                                  },
                                  &params).unwrap();
            },
            Material::NonTextured{
                ambiant_color: ambiant,
                diffuse_color: diffuse,
                specular_color: specular,
                specular_exponent: specular_exponent,
                emission_color: emission,
                opacity: opacity
            } =>
            {
                self.frame.draw( (vertex_buffer, per_instance.per_instance().unwrap()),
                                  indices,
                                  &gr.program_nontextured,
                                  &uniform! {view_matrix: gr.camera.get_view_matrix(),
                                             perspective_matrix: gr.camera.get_perspective_matrix(),
                                             ambiant: *ambiant,
                                             diffuse: *diffuse,
                                             specular: *specular,
                                             specular_exponent: *specular_exponent,
                                             emission: *emission,
                                             opacity: *opacity
                                  },
                                  &params).unwrap();
            }
            _ =>
            {
                self.frame.draw( (vertex_buffer, per_instance.per_instance().unwrap()),
                                  indices,
                                  &gr.program_default,
                                  &uniform! {view_matrix: gr.camera.get_view_matrix() },
                                  &params).unwrap();
            }
            
        }
        

        

        
    }
    
    pub fn clear(&mut self)
    {
        self.frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
    }
    
    pub fn show(self)
    {
        self.frame.finish().unwrap();
    }
}

