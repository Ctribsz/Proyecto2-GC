use nalgebra_glm::Vec3;
use crate::ray_intersect::{RayIntersect, Intersect};
use crate::material::Material;

pub struct Cube {
    pub min: Vec3, // Coordenada mínima (esquina inferior del cubo)
    pub max: Vec3, // Coordenada máxima (esquina superior del cubo)
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let inv_dir = Vec3::new(1.0 / ray_direction.x, 1.0 / ray_direction.y, 1.0 / ray_direction.z);

        // Calculamos t1, t2, t3, t4, t5, t6 para cada plano del cubo
        let mut t1 = (self.min.x - ray_origin.x) * inv_dir.x;
        let mut t2 = (self.max.x - ray_origin.x) * inv_dir.x;
        let mut t3 = (self.min.y - ray_origin.y) * inv_dir.y;
        let mut t4 = (self.max.y - ray_origin.y) * inv_dir.y;
        let mut t5 = (self.min.z - ray_origin.z) * inv_dir.z;
        let mut t6 = (self.max.z - ray_origin.z) * inv_dir.z;

        // Intercambiamos los valores si el rayo está apuntando hacia atrás en cualquier eje
        if t1 > t2 { std::mem::swap(&mut t1, &mut t2); }
        if t3 > t4 { std::mem::swap(&mut t3, &mut t4); }
        if t5 > t6 { std::mem::swap(&mut t5, &mut t6); }

        // Calculamos tmin (la entrada más lejana) y tmax (la salida más cercana)
        let tmin = t1.max(t3).max(t5);
        let tmax = t2.min(t4).min(t6);

        // Verificamos si hay una intersección válida
        if tmax >= tmin && tmin > 0.0 {
            let distance = tmin;
            let point = ray_origin + ray_direction * distance;

            // Calculamos la normal del cubo
            let mut normal = Vec3::zeros();
            if tmin == t1 || tmin == t2 {
                normal.x = if tmin == t1 { -1.0 } else { 1.0 };
            } else if tmin == t3 || tmin == t4 {
                normal.y = if tmin == t3 { -1.0 } else { 1.0 };
            } else if tmin == t5 || tmin == t6 {
                normal.z = if tmin == t5 { -1.0 } else { 1.0 };
            }
            return Intersect::new(point, normal, distance, self.material.clone());
        }

        Intersect::empty()
    }
}
