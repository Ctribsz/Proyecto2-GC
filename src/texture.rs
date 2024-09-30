use nalgebra_glm::Vec3;
use crate::color::Color;

pub fn load_texture(file_path: &str) -> (Vec<u8>, u32, u32) {
    let img = image::open(file_path).expect(&format!("Error al cargar la textura: {}", file_path));
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();
        
    (img.into_raw(), width, height)  // Convertimos la imagen en un vector de bytes RGBA
}

pub fn get_texture_color(texture: &Vec<u8>, texture_width: u32, texture_height: u32, uv: [f32; 2]) -> Color {
    let tex_x = (uv[0] * texture_width as f32) as usize;
    let tex_y = (uv[1] * texture_height as f32) as usize;
    let pixel_index = (tex_y * texture_width as usize + tex_x) * 4;  // Formato RGBA, 4 bytes por píxel

    if pixel_index >= texture.len() {
        return Color::new(255, 0, 0);  // Devuelve color rojo para indicar error
    }

    Color {
        r: texture[pixel_index],
        g: texture[pixel_index + 1],
        b: texture[pixel_index + 2],
    }
}


pub fn calculate_uv(point_on_cube: Vec3, normal: Vec3, min: Vec3, max: Vec3) -> [f32; 2] {
    let (u, v) = if normal.z.abs() > 0.5 {
        // Cara frontal o trasera, usamos X e Y
        let u = (point_on_cube.x - min.x) / (max.x - min.x);
        let v = (point_on_cube.y - min.y) / (max.y - min.y);
        (u, v)
    } else if normal.x.abs() > 0.5 {
        // Caras laterales, usamos Z e Y
        let u = (point_on_cube.z - min.z) / (max.z - min.z);
        let v = (point_on_cube.y - min.y) / (max.y - min.y);
        (u, v)
    } else {
        // Cara superior o inferior, usamos X y Z
        let u = (point_on_cube.x - min.x) / (max.x - min.x);
        let v = (point_on_cube.z - min.z) / (max.z - min.z);
        (u, v)
    };

    // Nos aseguramos de que las coordenadas UV estén entre 0.0 y 1.0
    [u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)]
}
