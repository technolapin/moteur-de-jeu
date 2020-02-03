use crate::misc::*;
use nalgebra_glm::look_at;
use nalgebra_glm::make_vec3;
use nalgebra_glm::rotate_x_vec3;
use nalgebra_glm::rotate_y_vec3;
use nalgebra_glm::rotate_z_vec3;
use nalgebra::Matrix3;
use nalgebra::Vector3;

/**
A simple camera
*/

pub struct Camera {
    position: Vector3<f32>,
    up: Vector3<f32>,
    orientation: Vector3<f32>,
    aspect_ratio: f32,
}

impl Camera {
    /** Constructor of Frame  */
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: Vector3::new(0., 0., 0.), 
            orientation: Vector3::new(0., 0., -1.), 
            up: Vector3::new(0., 1., 0.),
            aspect_ratio: aspect_ratio, // Ratio for the printing on the screen
        }
    }

    pub fn set_aspect_ratio(&mut self, width: f32, height: f32) {
        self.aspect_ratio = height / width;
    }

    pub fn set_position(&mut self, position: (f32, f32, f32)) {
        self.position = Vector3::new(position.0, position.1, position.2);
    }

    /// Displace the camera
    pub fn relative_move(&mut self, displacement: (f32, f32, f32)) {

        let side = self.orientation.cross(&self.up);
	let displacement_vec = Vector3::new(displacement.0, displacement.1, displacement.2);

	// Create the transfer_matrix from the base "Scene" to the base "Camera"
	let transfer_matrix = Matrix3::from_columns(&[self.orientation, self.up, side]);

	// Calculate the new position of the camera in the base "Scene"
	let position_vec = transfer_matrix*displacement_vec;
	self.position = Vector3::new(position_vec[0]+self.position[0], position_vec[1]+self.position[1], position_vec[2]+self.position[2]);
    }


    pub fn set_direction(&mut self, orientation: (f32, f32, f32)) {
	let temporaire = normalize_vec(orientation);
        self.orientation = Vector3::new(temporaire.0, temporaire.1, temporaire.2);
    }

    /// Rotation of the Camera around its axises
    pub fn rotation_relative(&mut self, (rx, ry, rz): (f32, f32, f32)) {
        //on tourne de rx rad autour de l'axe 0x
        //on tourne de ry rad autour de l'axe 0y
        //on tourne de rz rad autour de l'axe 0z

        self.orientation = rotate_x_vec3(&self.orientation, rx) ;
	self.orientation = rotate_y_vec3(&self.orientation, ry) ;
	self.orientation = rotate_z_vec3(&self.orientation, rz).normalize() ;

        self.up = rotate_x_vec3(&self.up, rx) ;
	self.up = rotate_y_vec3(&self.up, ry) ;
	self.up = rotate_z_vec3(&self.up, rz) ;

    }

    /// Rotation of the Camera around axis x y and z of the Scene
    pub fn rotation(&mut self, (rx, ry, rz): (f32, f32, f32)) {
        //on tourne de rx rad autour de l'axe 0x
        //on tourne de ry rad autour de l'axe 0y
        //on tourne de rz rad autour de l'axe 0z

	let orientation_scene = Vector3::new(0., 0., -1.);
	let up_scene = Vector3::new(0., 1., 0.);

	self.orientation = rotate_x_vec3(&orientation_scene, rx);
	self.orientation = rotate_y_vec3(&self.orientation, ry);
	self.orientation = rotate_z_vec3(&self.orientation, rz).normalize();

	self.up = rotate_x_vec3(&up_scene, rx);
	self.up = rotate_y_vec3(&self.up, ry);
	self.up = rotate_z_vec3(&self.up, rz);

    }


    /** Move the Objects of the scene according to the rotation of the Camera, Return the matrix of the view */
    /**
    returns the view matrix of the camera
    It will be used by the shaders to displace the objects of the scene to put them at the right place, with the right orientation and size.
     */
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {

	let center = Vector3::new(self.orientation[0] + self.position[0], self.orientation[1] + self.position[1], self.orientation[2] + self.position[2]);
	let look_at = look_at(&self.position, &center, &self.up);
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
