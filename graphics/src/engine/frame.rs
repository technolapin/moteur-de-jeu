use glium::Surface;

use super::graphical::*;
use crate::processing::objects::*;
use crate::processing::material::*;


#[derive(Copy, Clone)]
pub struct Attr 
{
    pub world_transformation: [[f32; 4]; 4],
}
implement_vertex!(Attr, world_transformation);



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
                       obj: &Object,
                       per_instance: &glium::VertexBuffer<Attr>,
    )
    {
        obj.groups
            .iter()
            .for_each(
                |(vertexes, maybe_material)|
                {
                    match maybe_material
                    {
                        Some(material) =>
                        {
                            self.draw_group(gr,
                                            vertexes,
                                            per_instance,
                                            material);
                        },
                        None => unimplemented!()
                    }
                }
            );

        
    }

    
    pub fn draw_group(
        &mut self,
        gr: &Graphical,
        vertex_buffer: &glium::vertex::VertexBufferAny,
        per_instance: &glium::VertexBuffer<Attr>,
        material: &Material
    )
    {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


        match material
        {
            Material::Textured{
                texture: texture,
                specular_color: specular,
                specular_exponent: specular_exponent,
                opacity: opacity
            } =>
            {
                self.frame.draw( (vertex_buffer, per_instance.per_instance().unwrap()),
                                  indices,
                                  &gr.program_textured,
                                  &uniform! {
                                      texture: texture,
                                      view_matrix: gr.camera.get_view_matrix(),
                                      perspective_matrix: gr.camera.get_perspective_matrix(),
                                      
                                      specular_color: *specular,
                                      specular_exponent: *specular_exponent,
                                      opacity: *opacity
                                      
                                  },
                                  &gr.parameters).unwrap();
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
                                  &gr.parameters).unwrap();
            }
            _ =>
            {
                self.frame.draw( (vertex_buffer, per_instance.per_instance().unwrap()),
                                  indices,
                                  &gr.program_default,
                                  &uniform! {view_matrix: gr.camera.get_view_matrix() },
                                  &gr.parameters).unwrap();
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

