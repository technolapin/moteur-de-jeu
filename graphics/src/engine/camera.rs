use crate::misc::*;

#[derive(Default)]
pub struct Camera
{
    position: (f32, f32, f32),
    up: (f32, f32, f32),
    orientation: (f32, f32, f32),
    aspect_ratio: f32,
}

impl Camera
{
    /** Constructor of Frame  */
    pub fn new(aspect_ratio: f32) -> Self
    {
        Self {
            position: (0., 0., 0.),
            orientation: (0., 0., -1.),
            up: (0., 1., 0.),
            aspect_ratio: aspect_ratio, // Ratio for the printing on the screen
        }
    }

    pub fn set_aspect_ratio(&mut self, width: f32, height: f32)
    {
        self.aspect_ratio = height / width;
    }

    pub fn set_position(&mut self, position: (f32, f32, f32))
    {
        self.position = position;
    }

    /** Update the position of the Camera */
    pub fn relative_move(&mut self, displacement: (f32, f32, f32))
    {
        let side = v_prod(self.orientation, self.up);
        self.position = (
            self.position.0
                + self.orientation.0 * displacement.0
                + self.up.0 * displacement.1
                + side.0 * displacement.2,
            self.position.1
                + self.orientation.1 * displacement.0
                + self.up.1 * displacement.1
                + side.1 * displacement.2,
            self.position.2
                + self.orientation.2 * displacement.0
                + self.up.2 * displacement.1
                + side.2 * displacement.2,
        );
    }

    pub fn set_direction(&mut self, orientation: (f32, f32, f32))
    {
        self.orientation = normalize_vec(orientation);
    }

    /** Rotation of the Camera around its axis */
    pub fn rotation_relative(&mut self, (rx, ry, rz): (f32, f32, f32))
    {
        //on tourne de rx rad autour de l'axe 0x
        //on tourne de ry rad autour de l'axe 0y
        //on tourne de rz rad autour de l'axe 0z

        let (x, y, z) = self.orientation;
        let (ux, uy, uz) = self.up;

        let (x, y, z) = (x, y * rx.cos() + z * rx.sin(), -y * rx.sin() + z * rx.cos());
        let (ux, uy, uz) = (
            ux,
            uy * rx.cos() + uz * rx.sin(),
            -uy * rx.sin() + uz * rx.cos(),
        );

        let (x, y, z) = (x * ry.cos() - z * ry.sin(), y, x * ry.sin() + z * ry.cos());
        let (ux, uy, uz) = (
            ux * ry.cos() - uz * ry.sin(),
            uy,
            ux * ry.sin() + uz * ry.cos(),
        );

        let (x, y, z) = (x * rz.cos() + y * rz.sin(), -x * rz.sin() + y * rz.cos(), z);
        let (ux, uy, uz) = (
            ux * rz.cos() + uy * rz.sin(),
            -ux * rz.sin() + uy * rz.cos(),
            uz,
        );

        self.orientation = normalize_vec((x, y, z));
        self.up = (ux, uy, uz);
    }

    /** Rotation of the Camera around axis x y and z of the Scene */
    pub fn rotation(&mut self, (rx, ry, rz): (f32, f32, f32))
    {
        //on tourne de rx rad autour de l'axe 0x
        //on tourne de ry rad autour de l'axe 0y
        //on tourne de rz rad autour de l'axe 0z

        let (x, y, z) = (0.0, 0.0, -1.);
        let (ux, uy, uz) = (0., 1., 0.);

        let (x, y, z) = (x, y * rx.cos() + z * rx.sin(), -y * rx.sin() + z * rx.cos());
        let (ux, uy, uz) = (
            ux,
            uy * rx.cos() + uz * rx.sin(),
            -uy * rx.sin() + uz * rx.cos(),
        );

        let (x, y, z) = (x * ry.cos() - z * ry.sin(), y, x * ry.sin() + z * ry.cos());
        let (ux, uy, uz) = (
            ux * ry.cos() - uz * ry.sin(),
            uy,
            ux * ry.sin() + uz * ry.cos(),
        );

        let (x, y, z) = (x * rz.cos() + y * rz.sin(), -x * rz.sin() + y * rz.cos(), z);
        let (ux, uy, uz) = (
            ux * rz.cos() + uy * rz.sin(),
            -ux * rz.sin() + uy * rz.cos(),
            uz,
        );

        self.orientation = normalize_vec((x, y, z));
        self.up = (ux, uy, uz);
    }

    /** Move the Objects of the scene according to the rotation of the Camera, Return the matrix of the view */
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4]
    {
        let f = self.orientation;
        //let u = (0., 1., 0.);
        //let u = normalize_vec((-f.1, -f.2, f.0));
        let u = self.up;

        let s = normalize_vec(v_prod(f, u));
        let p = (
            -self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
            -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
            -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2,
        );

        [
            [s.0, u.0, f.0, 0.0],
            [s.1, u.1, f.1, 0.0],
            [s.2, u.2, f.2, 0.0],
            [p.0, p.1, p.2, 1.0],
        ]
    }

    /** Return the matrix of the perspective */
    pub fn get_perspective_matrix(&self) -> [[f32; 4]; 4]
    {
        //let (width, height) = target.get_dimensions();

        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f * self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    }
}
