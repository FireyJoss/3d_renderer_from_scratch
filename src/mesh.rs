use nalgebra::{Vector3, Vector4};

pub struct Mesh3D {
    pub positions: Vec<Vector4<f32>>,
    pub triangles: Vec<Vector3<usize>>,
}

impl Mesh3D {
    pub fn new(positions: Vec<Vector4<f32>>, triangles: Vec<Vector3<usize>>,) -> Self {
        Self { positions, triangles }
    }
}

pub struct Mesh2D<'a> {
    pub positions: Vec<Vector4<f32>>,
    pub triangles: &'a [Vector3<usize>],
}

impl<'a> Mesh2D<'a> {
    pub fn new(positions: Vec<Vector4<f32>>, triangles: &'a [Vector3<usize>]) -> Self {
        Self { positions, triangles }
    }

    pub fn get_vertex(&self, index: usize) -> Option<Vector4<f32>> {
        self.positions.get(index).copied()
    }
}
