use jpeg_decoder::Decoder;
use palette::Srgb;
use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use std::fs::File;
use std::io::BufReader;

use crate::vec3d::Vec3D;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;


fn refract(ray: Vec3D, normal: Vec3D, ratio: f64) -> Vec3D {
    // ray: incoming ray
    // normal: normal vector of the surface
    // ratio: the ratio of the refractive indices of the two materials

    // Snell's Law: n1 * sin(theta1) = n2 * sin(theta2)
    // cosine of the angle between the ray and the normal
    let cos_theta = (-ray).dot(normal).min(1.0);
    let perp = ratio * (ray + cos_theta * normal); // perpendicular to the surface
    let out = -(1.0 - perp.length_squared()).abs().sqrt() * normal;
    let refracted = perp + out;

    refracted
}

fn reflectance(cos: f64, ratio: f64) -> f64 {
    // Schlick's approximation
    let r0 = (1.0 - ratio) / (1.0 + ratio);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refract() {
        let ray = Vec3D::new(1.0, 1.0, 1.0).normalize();
        let normal = Vec3D::new(0.0, 0.0, 1.0);
        let ratio = 1.5;
        let refracted = refract(ray, normal, ratio);
        assert_approx_eq!(refracted.length(), 1.0);
    }

    #[test]
    fn test_reflectance() {
        let cos = 0.5;
        let ratio = 1.5;
        let reflectance_value = reflectance(cos, ratio);
        assert_approx_eq!(reflectance_value, 0.04, 1e-2);
    }

    #[test]
    fn test_refract_perpendicular() {
        let ray = Vec3D::new(0.0, -1.0, 0.0).normalize();
        let normal = Vec3D::new(0.0, 1.0, 0.0);
        let ratio = 1.0;
        let refracted = refract(ray, normal, ratio);
        assert_approx_eq!(refracted.x, 0.0);
        assert_approx_eq!(refracted.y, -1.0);
        assert_approx_eq!(refracted.z, 0.0);
    }

    #[test]
    fn test_reflectance_edge_case() {
        let cos = 1.0;
        let ratio = 1.5;
        let reflectance_value = reflectance(cos, ratio);
        assert_approx_eq!(reflectance_value, 0.04, 1e-2);
    }
}
