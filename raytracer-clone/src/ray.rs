use crate::vec3d::Vec3D;
use crate::materials::Material;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3D,
    direction: Vec3D,
}

impl Ray {
    pub fn new(origin: Vec3D, direction: Vec3D) -> Ray {
        Ray { origin, direction }
    }
    
    pub fn origin(&self) -> Vec3D {
        self.origin
    }

    pub fn direction(&self) -> Vec3D {
        self.direction
    }

    // Returns the point at a distance t along the ray
    pub fn at(&self, t: f64) -> Vec3D {
        self.origin + t * self.direction
    }
}

pub struct HitRecord<'material> {
    pub t: f64,
    pub point: Point3D,
    pub normal: Point3D,
    pub front_face: bool,
    pub material: &'material Material,
    pub u: f64,
    pub v: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}