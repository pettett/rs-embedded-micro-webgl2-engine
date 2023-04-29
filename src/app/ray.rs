use nalgebra::{Matrix4, Point3, Vector3};

/// Half line ray representation
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}
impl Ray {
    ///create a new ray firing in the z direction
    pub fn forward() -> Ray {
        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        }
    }
    ///Transform a ray by a matrix
    pub fn transform(&self, mat: &Matrix4<f32>) -> Ray {
        Ray {
            origin: mat.transform_point(&self.origin),
            direction: mat.transform_vector(&self.direction),
        }
    }
}
