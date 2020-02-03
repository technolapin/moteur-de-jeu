use crate::misc::*;
use nalgebra_glm::look_at;
use nalgebra_glm::make_vec3;
use nalgebra_glm::rotate_x_vec3;
use nalgebra_glm::rotate_y_vec3;
use nalgebra_glm::rotate_z_vec3;
use nalgebra::Matrix3;

/**
A simple camera
*/
#[derive(Default)]
pub struct Camera {
    position: (f32, f32, f32),
    up: (f32, f32, f32),
    orientation: (f32, f32, f32),
    aspect_ratio: f32,
}

impl Camera {
    /** Constructor of Frame  */
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: (0., 0., 0.),
            orientation: (0., 0., -1.),
            up: (0., 1., 0.),
            aspect_ratio: aspect_ratio, // Ratio for the printing on the screen
        }
    }

    pub fn set_aspect_ratio(&mut self, width: f32, height: f32) {
        self.aspect_ratio = height / width;
    }

    pub fn set_position(&mut self, position: (f32, f32, f32)) {
        self.position = position;
    }

    /// Displace the camera
    pub fn relative_move(&mut self, displacement: (f32, f32, f32)) {

        let side = v_prod(self.orientation, self.up);

	// To get the Vec3 type for each tuple that we have --> Should be removed
        let orientation_array = [self.orientation.0, self.orientation.1, self.orientation.2] ;
        let up_array = [self.up.0, self.up.1, self.up.2] ;
	let side_array = [side.0, side.1, side.2];

        let orientation_vec = make_vec3(&orientation_array);
        let up_vec = make_vec3(&up_array);
	let side_vec = make_vec3(&side_array);

	let displacement_array = [displacement.0, displacement.1, displacement.2];
	let displacement_vec = make_vec3(&displacement_array);

	// Create the transfer_matrix from the base "Scene" to the base "Camera"
	let transfer_matrix = Matrix3::from_columns(&[orientation_vec, up_vec, side_vec]);

	// Calculate the new position of the camera in the base "Scene"
	let position_vec = transfer_matrix*displacement_vec;
	self.position = (position_vec[0]+self.position.0, position_vec[1]+self.position.1, position_vec[2]+self.position.2);
    }


    pub fn set_direction(&mut self, orientation: (f32, f32, f32)) {
        self.orientation = normalize_vec(orientation);
    }

    /// Rotation of the Camera around its axises
    pub fn rotation_relative(&mut self, (rx, ry, rz): (f32, f32, f32)) {
        //on tourne de rx rad autour de l'axe 0x
        //on tourne de ry rad autour de l'axe 0y
        //on tourne de rz rad autour de l'axe 0z

	// To get the Vec3 type for each tuple that we have --> Should be removed
        let orientation = [self.orientation.0, self.orientation.1, self.orientation.2] ;
        let up = [self.up.0, self.up.1, self.up.2] ;

        let f = make_vec3(&orientation);
        let u = make_vec3(&up);

        let rotate_x_orientation = rotate_x_vec3(&f, rx) ;
	let rotate_xy_orientation = rotate_y_vec3(&rotate_x_orientation, ry) ;
	let rotate_xyz_orientation = rotate_z_vec3(&rotate_xy_orientation, rz) ;

        let rotate_x_up = rotate_x_vec3(&u, rx) ;
	let rotate_xy_up = rotate_y_vec3(&rotate_x_up, ry) ;
	let rotate_xyz_up = rotate_z_vec3(&rotate_xy_up, rz) ;
	
        self.orientation = normalize_vec((rotate_xyz_orientation[0],rotate_xyz_orientation[1],rotate_xyz_orientation[2]));
        self.up = (rotate_xyz_up[0],rotate_xyz_up[1],rotate_xyz_up[2]);
    }

    /// Rotation of the Camera around axis x y and z of the Scene
    pub fn rotation(&mut self, (rx, ry, rz): (f32, f32, f32)) {
        //on tourne de rx rad autour de l'axe 0x
        //on tourne de ry rad autour de l'axe 0y
        //on tourne de rz rad autour de l'axe 0z

	// To get the Vec3 type for each tuple that we have --> Should be removed
        let orientation_scene = [0.0, 0.0, -1.];
        let up_scene = [0., 1., 0.];

        let f = make_vec3(&orientation_scene);
        let u = make_vec3(&up_scene);

        let rotate_x_orientation = rotate_x_vec3(&f, rx) ;
	let rotate_xy_orientation = rotate_y_vec3(&rotate_x_orientation, ry) ;
	let rotate_xyz_orientation = rotate_z_vec3(&rotate_xy_orientation, rz) ;

        let rotate_x_up = rotate_x_vec3(&u, rx) ;
	let rotate_xy_up = rotate_y_vec3(&rotate_x_up, ry) ;
	let rotate_xyz_up = rotate_z_vec3(&rotate_xy_up, rz) ;
	
        self.orientation = normalize_vec((rotate_xyz_orientation[0],rotate_xyz_orientation[1],rotate_xyz_orientation[2]));
        self.up = (rotate_xyz_up[0],rotate_xyz_up[1],rotate_xyz_up[2]);
    }


    /** Move the Objects of the scene according to the rotation of the Camera, Return the matrix of the view */
    /**
    returns the view matrix of the camera
    It will be used by the shaders to displace the objects of the scene to put them at the right place, with the right orientation and size.
     */
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {

	let position = [self.position.0, self.position.1, self.position.2];
        let center = [self.orientation.0 + self.position.0, self.orientation.1 + self.position.1, self.orientation.2 + self.position.2];
        let up = [self.up.0, self.up.1, self.up.2];

        let f = make_vec3(&center);
        let u = make_vec3(&up);
	let o = make_vec3(&position);

	let look_at = look_at(&o, &f, &u);
	let view_matrix = look_at.as_ref();
	
	*view_matrix 

    }

    /// Return the matrix of the perspective
    pub fn get_perspective_matrix(&self) -> [[f32; 4]; 4] {
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
