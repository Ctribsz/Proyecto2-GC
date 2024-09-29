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
}

