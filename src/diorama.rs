use nalgebra_glm::Vec3;
use crate::cube::Cube;
use crate::material::Material;

pub fn create_diorama(grass_material: Material, dirt_material: Material) -> Vec<Cube> {
    let mut objects: Vec<Cube> = Vec::new();

    // Cubo de césped
    objects.push(Cube {
        min: Vec3::new(-0.5, -0.5, -1.0),
        max: Vec3::new(0.5, 0.5, -0.5),
        material: grass_material.clone(),  // Clonamos el material de césped
    });

    // Cubo de tierra (usamos .clone() para no mover el valor)
    objects.push(Cube {
        min: Vec3::new(1.0, -0.5, -1.5),
        max: Vec3::new(1.5, 0.0, -1.0),
        material: dirt_material.clone(),  // Clonamos el material de tierra
    });

    objects.push(Cube {
        min: Vec3::new(2.0, -0.5, -1.0),
        max: Vec3::new(2.5, 0.0, -0.5),
        material: dirt_material.clone(),  // Clonamos el material de tierra de nuevo
    });

    // Añadir más cubos
    objects.push(Cube {
        min: Vec3::new(3.0, -0.5, -2.0),
        max: Vec3::new(3.5, 0.5, -1.5),
        material: grass_material.clone(),  // Clonamos el material de césped
    });

    objects
}
