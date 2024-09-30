use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Option<Color>,   // Color difuso (puede ser None si usamos textura)
    pub texture: Option<Vec<u8>>, // Textura (opcional)
    pub texture_width: u32,       // Ancho de la textura
    pub texture_height: u32,      // Alto de la textura
    pub specular: f32,            // Brillo especular
    pub albedo: [f32; 2],         // Factores de reflectancia
}

impl Material {
    pub fn new(diffuse: Option<Color>, texture: Option<Vec<u8>>, texture_width: u32, texture_height: u32,specular: f32, albedo: [f32; 2]) -> Self {
        Material {
            diffuse,
            texture,
            texture_height,
            texture_width,
            specular,
            albedo,
        }
    }

    pub fn default() -> Self {
        Material {
            diffuse: Some(Color::new(50, 50, 50)),  // Un color gris oscuro
            texture: None,  // Sin textura
            texture_width: 0,
            texture_height: 0,
            specular: 0.0,  // Sin brillo especular
            albedo: [0.0, 0.0],  // Ninguna reflectancia
        }
    }

    pub fn grass() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/grass_carried.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 50.0,
            albedo: [0.9, 0.1],
        }
    }

    // Método para crear material de tierra
    pub fn dirt() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/dirt.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 30.0,
            albedo: [0.8, 0.2],
        }
    }

    // Método para crear material de piedra
    pub fn stone() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/stone.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 10.0,
            albedo: [0.7, 0.3],
        }
    }

    pub fn diamond() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/diamond_ore.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 10.0,
            albedo: [0.7, 0.3],
        }
    }

    pub fn azalea() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/azalea.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 10.0,
            albedo: [0.7, 0.3],
        }
    }

    pub fn ama() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/amatista.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 10.0,
            albedo: [0.7, 0.3],
        }
    }

    pub fn bamboo() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/bamboo.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 10.0,
            albedo: [0.7, 0.3],
        }
    }

    pub fn sand() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/sand.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 10.0,
            albedo: [0.7, 0.3],
        }
    }
}

