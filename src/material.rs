use crate::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub diffuse: Option<Color>,   // Color difuso (puede ser None si usamos textura)
    pub texture: Option<Vec<u8>>, // Textura (opcional)
    pub texture_width: u32,       // Ancho de la textura
    pub texture_height: u32,      // Alto de la textura
    pub specular: f32,            // Brillo especular
    pub albedo: [f32; 2],         // Factores de reflectancia
    pub emissive: Option<Color>,  // Color emisivo (si el material emite luz)
    pub emission_intensity: f32,  // Intensidad de emisión de luz
}


impl Material {
    pub fn new(diffuse: Option<Color>, texture: Option<Vec<u8>>, texture_width: u32, texture_height: u32, specular: f32, albedo: [f32; 2], emissive: Option<Color>, emission_intensity: f32) -> Self {
        Material {
            diffuse,
            texture,
            texture_width,
            texture_height,
            specular,
            albedo,
            emissive,
            emission_intensity,
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
            emissive: None,
            emission_intensity: 0.0,
        }
    }

    pub fn grass() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/grass_carried.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 20.0,      // Relativamente bajo, ya que el pasto no es reflectante
            albedo: [0.8, 0.1],
            emissive: None,  // Más luz difusa, menos especular
            emission_intensity: 0.0,
        }
    }
    
    pub fn dirt() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/dirt.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 10.0,      // Muy bajo, tierra es casi mate
            albedo: [0.9, 0.05], // Alta luz difusa, casi nada de especular
            emissive: None,
            emission_intensity: 0.0,
        }
    }
    
    pub fn stone() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/stone.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 15.0,      // Piedra puede tener un poco de brillo pero muy bajo
            albedo: [0.7, 0.1],  // Principalmente luz difusa
            emissive: None,
            emission_intensity: 0.0,
        }
    }
    
    pub fn diamond() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/diamond_ore.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 80.0,      // Diamante es muy reflectante
            albedo: [0.6, 0.4],  // Más especular que difuso
            emissive: None,
            emission_intensity: 0.0,
        }
    }
    
    pub fn azalea() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/azalea.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 15.0,      // Hojas pueden tener algo de brillo
            albedo: [0.8, 0.1],  // Principalmente difuso
            emissive: None,
            emission_intensity: 0.0,
        }
    }
    
    pub fn bamboo() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/bamboo.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 20.0,      // La caña de bambú tiene algo de brillo
            albedo: [0.75, 0.1], // Más difuso, poco especular
            emissive: None,
            emission_intensity: 0.0,
        }
    }
    
    pub fn sand() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/sand.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 5.0,       // Arena es casi mate
            albedo: [0.9, 0.05], // Muy poco especular, casi todo difuso
            emissive: None,
            emission_intensity: 0.0,
        }
    }
    
    pub fn moosy() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/mossy.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 10.0,      // Musgo es mate
            albedo: [0.8, 0.1],  // Luz difusa
            emissive: None,
            emission_intensity: 0.0,
        }
    }
    
    pub fn moosy_block() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/stonebrick.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 15.0,      // Ladrillos de piedra tienen algo de brillo
            albedo: [0.7, 0.2],  // Un poco de especularidad
            emissive: None,
            emission_intensity: 0.0,
        }
    }
    
    pub fn lamp() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/lamp.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 50.0,      // Las lámparas son reflectantes
            albedo: [0.4, 0.6],  // Más luz especular, menos difusa
            emissive: Some(Color::new(255, 215, 0)), 
            emission_intensity: 10.0,
        }
    }
    
    pub fn emerald() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/emerald.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 60.0,      // Esmeralda es bastante reflectante
            albedo: [0.5, 0.5],  // Equilibrio entre difuso y especular
            emissive: None,
            emission_intensity: 0.0,
        }
    }    


    pub fn concrete() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/concrete.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 10.0,      // Esmeralda es bastante reflectante
            albedo: [0.8, 0.1],  // Equilibrio entre difuso y especular
            emissive: None,
            emission_intensity: 0.0,
        }
    } 


    pub fn glass() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/glass.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 60.0,  // Especularidad alta, pero no extrema para ver más color
            albedo: [0.4, 0.6],
            emissive: None,
            emission_intensity: 0.0,
        }
    }

    pub fn terracota() -> Self {
        Material {
            diffuse: None,
            texture: Some(crate::texture::load_texture("assets/terracota.png").0),
            texture_width: 16,
            texture_height: 16,
            specular: 20.0,      // Relativamente bajo, ya que el pasto no es reflectante
            albedo: [0.8, 0.1],
            emissive: None,  // Más luz difusa, menos especular
            emission_intensity: 0.0,
        }
    } 
}

