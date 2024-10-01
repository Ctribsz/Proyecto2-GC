use nalgebra_glm::Vec3;
use crate::cube::Cube;
use crate::material::Material;

pub fn generate_diorama() -> Vec<Cube> {
    let mut objects = Vec::new();

    let cube_size = 0.5;

    //Cubos de diamantes de primero
    for x in (0..(1.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for z in (0..(1.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(x, 0.0, z),  // Coordenada mínima del cubo
                max: Vec3::new(x + cube_size, cube_size, z + cube_size),  // Coordenada máxima del cubo
                material: Material::diamond(),  // Usamos referencia al material
            });
        }
    }

    //Cubos bloque de stone 
    let mut x = 1.0;
    while x < 4.0 {
        let mut z = 0.0;
        while z < 4.0 {
            objects.push(Cube {
                min: Vec3::new(x, 0.0, z),  // Coordenada mínima del cubo
                max: Vec3::new(x + cube_size, cube_size, z + cube_size),  // Coordenada máxima (cubos 0.5x0.5)
                material: Material::stone(),  // Material
            });
            z += cube_size;  // Incremento manual para el eje Z
        }
        x += cube_size;  // Incremento manual para el eje X
    }
    
    //Linea de cubos
    for x in (0..(1.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for z in (0..(4.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(x, 0.0, z),  // Coordenada mínima del cubo
                max: Vec3::new(x + cube_size, cube_size, z + cube_size),  // Coordenada máxima del cubo
                material: Material::stone(),  // Usamos referencia al material
            });
        }
    }

    //Pared trasera 1
    for y in (0..(4.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for z in (0..(4.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(4.0, y, z),  // Coordenada mínima del cubo
                max: Vec3::new(4.0 + cube_size, y + cube_size, z + cube_size),  // Coordenada máxima del cubo
                material: Material::stone(),  // Usamos referencia al material
            });
        }
    }

    //Pared trasera 2
    for y in (0..(4.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for x in (0..(5.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(x, y, 4.0),  // Coordenada mínima del cubo
                max: Vec3::new(cube_size, y + cube_size, 4.0 + cube_size),  // Coordenada máxima del cubo
                material: Material::stone(),  // Usamos referencia al material
            });
        }
    }

    //Techo grass
    for x in (4..(5.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for z in (0..(2.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(x, 4.0, z),  // Coordenada mínima del cubo
                max: Vec3::new(x + cube_size, 4.0 + cube_size, z + cube_size),  // Coordenada máxima del cubo
                material: Material::grass(),  // Usamos referencia al material
            });
        }
    }

    //Techo tierra
    for x in (5..(5.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for z in (2..(5.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(x, 4.0, z),  // Coordenada mínima del cubo
                max: Vec3::new(x + cube_size, 4.0 + cube_size, z + cube_size),  // Coordenada máxima del cubo
                material: Material::dirt(),  // Usamos referencia al material
            });
        }
    }

    //Techo arena 
    for x in (0..(3.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
        for z in (4..(5.0 / cube_size) as usize).map(|i| i as f32 * cube_size) {
            objects.push(Cube {
                min: Vec3::new(x, 4.0, z),  // Coordenada mínima del cubo
                max: Vec3::new(x + cube_size, 4.0 + cube_size, z + cube_size),  // Coordenada máxima del cubo
                material: Material::sand(),  // Usamos referencia al material
            });
        }
    }

    //Elementos unicos dentro del bioma
    //Azalea fila 1
    objects.push(Cube {
        min: Vec3::new(3.5, 3.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 3.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.0, 3.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.0 + cube_size, 3.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(2.5, 3.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(2.5 + cube_size, 3.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });
    
    //Azalea fila 2
    objects.push(Cube {
        min: Vec3::new(3.5, 3.0, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 3.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.0, 3.0, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.0 + cube_size, 3.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(2.5, 3.0, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(2.5 + cube_size, 3.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Azalea fila 3
    objects.push(Cube {
        min: Vec3::new(3.5, 2.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 2.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.0, 2.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.0 + cube_size, 2.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 2.0, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 2.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.0, 2.0, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.0 + cube_size, 2.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 1.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 1.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.0, 2.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.0 + cube_size, 2.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::azalea(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Bamboo
    //Bamboo fila 1
    objects.push(Cube {
        min: Vec3::new(3.5, 3.5, 0.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 3.5 + cube_size, cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::bamboo(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 3.5, 0.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 3.5 + cube_size, 0.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::bamboo(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 3.5, 1.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 3.5 + cube_size, 1.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::bamboo(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 3.5, 1.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 3.5 + cube_size, 1.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::bamboo(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Bamboo fila 2
    objects.push(Cube {
        min: Vec3::new(3.5, 3.0, 0.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 3.0 + cube_size, cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::bamboo(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 3.0, 0.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 3.0 + cube_size, 0.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::bamboo(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 3.0, 1.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 3.0 + cube_size, 1.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::bamboo(),  // Material del cubo (o cualquier otro material que prefieras)
    });
    

    //Bamboo fila 3
    objects.push(Cube {
        min: Vec3::new(3.5, 2.5, 0.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 2.5 + cube_size, cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::bamboo(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 2.5, 0.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 2.5 + cube_size, 0.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::bamboo(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Bamboo fila 4
    objects.push(Cube {
        min: Vec3::new(3.5, 2.0, 0.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 2.0 + cube_size, cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::bamboo(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Mossy blocks
    //Fila 1
    objects.push(Cube {
        min: Vec3::new(0.0, 2.0, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 2.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.0, 2.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 2.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.0, 3.0 ,3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 3.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.0, 3.5 ,3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 3.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Fila 2
    objects.push(Cube {
        min: Vec3::new(0.5, 2.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.5 + cube_size, 2.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.5, 3.0 ,3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.5 + cube_size, 3.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.5, 3.5 ,3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.5 + cube_size, 3.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Fila 3
    objects.push(Cube {
        min: Vec3::new(1.0, 3.0 ,3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(1.0 + cube_size, 3.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(1.0, 3.5 ,3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(1.0 + cube_size, 3.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(1.0, 3.0 ,3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(1.0 + cube_size, 3.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    // Fila 4
    objects.push(Cube {
        min: Vec3::new(1.5, 3.5 ,3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(1.5 + cube_size, 3.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Moggsy blocks 2
    objects.push(Cube {
        min: Vec3::new(3.5, 1.0 ,3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 1.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy_block(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 0.5 ,3.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 0.5 + cube_size, 3.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy_block(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 0.5 ,2.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 0.5 + cube_size, 2.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy_block(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.0, 0.5 ,3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.0 + cube_size, 0.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy_block(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Grass and dirt blocks
    objects.push(Cube {
        min: Vec3::new(3.5, 0.5 , 0.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 0.5 + cube_size, 0.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::grass(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.0, 0.5 , 0.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.0 + cube_size, 0.5 + cube_size, 0.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::grass(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 0.5 , 0.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 0.5 + cube_size, 0.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::dirt(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 0.5 , 1.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 0.5 + cube_size, 1.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::dirt(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 0.5 , 1.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 0.5 + cube_size, 1.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::moosy(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 1.0 , 0.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 1.0 + cube_size, 0.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::concrete(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Esmeraldas y diamantes
    objects.push(Cube {
        min: Vec3::new(3.5, 2.5 , 2.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 2.5 + cube_size, 2.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::emerald(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(3.5, 2.5 , 2.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(3.5 + cube_size, 2.5 + cube_size, 2.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::emerald(),  // Material del cubo (o cualquier otro material que prefieras)
    });
    
    //Diamantes
    objects.push(Cube {
        min: Vec3::new(2.5, 0.5 , 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(2.5 + cube_size, 0.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::diamond(),  // Material del cubo (o cualquier otro material que prefieras)
    });    

    objects.push(Cube {
        min: Vec3::new(2.0, 0.5 , 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(2.0 + cube_size, 0.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::diamond(),  // Material del cubo (o cualquier otro material que prefieras)
    });    


    objects.push(Cube {
        min: Vec3::new(2.0, 1.0, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(2.0 + cube_size, 1.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::diamond(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Concreto 
    objects.push(Cube {
        min: Vec3::new(0.0, 1.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 1.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::concrete(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.0, 1.0, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 1.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::concrete(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.0, 0.5, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 0.5 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::concrete(),  // Material del cubo (o cualquier otro material que prefieras)
    });
    
    //Fila 2
    objects.push(Cube {
        min: Vec3::new(0.0, 1.0, 3.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 1.0 + cube_size, 3.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::concrete(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.0, 0.5, 3.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 0.5 + cube_size, 3.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::concrete(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Fila 3
    objects.push(Cube {
        min: Vec3::new(0.0, 0.5, 2.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 0.5 + cube_size, 2.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::concrete(),  // Material del cubo (o cualquier otro material que prefieras)
    });


    //Glass
    objects.push(Cube {
        min: Vec3::new(2.0, 2.0, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(2.0 + cube_size, 2.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::glass(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(1.5, 2.0, 3.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(1.5 + cube_size, 2.0 + cube_size, 3.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::glass(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Lamp
    objects.push(Cube {
        min: Vec3::new(2.0, 3.5, 2.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(2.0 + cube_size, 3.5 + cube_size, 2.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::lamp(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    //Mesa
    objects.push(Cube {
        min: Vec3::new(0.0, 0.5, 0.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 0.5 + cube_size, 0.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::terracota(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.5, 0.5, 0.0),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.5 + cube_size, 0.5 + cube_size, 0.0 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::terracota(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.0, 0.5, 0.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.0 + cube_size, 0.5 + cube_size, 0.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::terracota(),  // Material del cubo (o cualquier otro material que prefieras)
    });

    objects.push(Cube {
        min: Vec3::new(0.5, 0.5, 0.5),  // Coordenada mínima (X, Y, Z) donde quieres que inicie el cubo
        max: Vec3::new(0.5 + cube_size, 0.5 + cube_size, 0.5 + cube_size),  // Coordenada máxima, sumando el tamaño del cubo
        material: Material::terracota(),  // Material del cubo (o cualquier otro material que prefieras)
    });


    
    objects
}