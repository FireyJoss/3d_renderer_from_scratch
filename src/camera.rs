use nalgebra::{Matrix4, Vector4};

use crate::mesh::{Mesh2D, Mesh3D};

pub struct Camera {
    Mproj: Matrix4<f32>,
    w: f32,
    h: f32,
    world_position: Vector4<f32>,
    y_rotation: f32
}

impl Camera {
    pub fn new(world_position: Vector4<f32>, y_rotation: f32, w: f32, h: f32, fov_radians: f32, near_plane: f32, far_plane: f32) -> Self {
        let aspect_ratio = w / h;
        let tan_half_fov = (fov_radians / 2.0).tan();

        // Construct the perspective projection matrix
        let Mproj = Matrix4::new(
            1.0 / (aspect_ratio * tan_half_fov), 0.0, 0.0, 0.0,
            0.0, 1.0 / tan_half_fov, 0.0, 0.0,
            0.0, 0.0, (far_plane + near_plane) / (near_plane - far_plane), 
                  (2.0 * far_plane * near_plane) / (near_plane - far_plane),
            0.0, 0.0, -1.0, 0.0,
        );

        Self { Mproj, w, h , world_position, y_rotation}
    }

    pub fn move_cam(&mut self, delta_position: Vector4<f32>, delta_y_rotation: f32) {
        self.world_position += delta_position;
        self.y_rotation += delta_y_rotation;
    }

    pub fn get_2d_mesh<'a>(&self, mesh3d: &'a Mesh3D) -> Mesh2D<'a>
    {  
        //construct movement matrix
        let Mmov = Matrix4::new(
            self.y_rotation.cos(), 0.0, self.y_rotation.sin(), -self.world_position.x,
            0.0,1.0,0.0, -self.world_position.y,
            -self.y_rotation.sin(),0.0,self.y_rotation.cos(), -self.world_position.z,
            0.0, 0.0, 0.0, 1.0,
        );

        let Mtransform = self.Mproj * Mmov;
        let mut positions_screen: Vec<Vector4<f32>> = Vec::new();

        for point_3d in &mesh3d.positions {
            let point_clip = Mtransform * point_3d;
        
            // Directly calculate the screen coordinates
            positions_screen.push(Vector4::new(
                (point_clip.x / point_clip.w) * 0.5 * self.w,
                1.0 - (point_clip.y / point_clip.w) * 0.5 * self.h,
                point_clip.z / point_clip.w,
                1.0,
            ));
        }

        Mesh2D::new(positions_screen, &mesh3d.triangles)
    }
}
