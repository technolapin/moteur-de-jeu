use crate::misc::*;

#[derive(Default)]
pub struct Camera
{
    position: (f32, f32, f32),
    up: (f32, f32, f32),
    orientation: (f32, f32, f32),
    aspect_ratio: f32
}


impl Camera
{
    pub fn new(aspect_ratio: f32) -> Self
    {
        Self
        {
            position: (0., 0., 0.),
            orientation: (0., 0., -1.),
            up: (0., 1., 0.),
            aspect_ratio: aspect_ratio
        }
    }

    pub fn set_aspect_ratio(&mut self, width: f32, height: f32)
    {
        self.aspect_ratio = height/width;
    }
    
    pub fn set_position(&mut self, position: (f32, f32, f32))
    {
        self.position = position;
    }
    
    pub fn set_direction(&mut self, orientation: (f32, f32, f32))
    {
        self.orientation = normalize_vec(orientation);
    }

    pub fn rotation(&mut self, (rx, ry, rz): (f32, f32, f32))
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
    
    
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4]
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

    pub fn get_perspective_matrix(&self) -> [[f32; 4]; 4]
    {
        //let (width, height) = target.get_dimensions();

        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f *   self.aspect_ratio,    0.0,              0.0              ,   0.0],
            [         0.0           ,     f ,              0.0              ,   0.0],
            [         0.0           ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0           ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }
    
}


