use nalgebra_glm::Vec3;
use crate::color::Color;
use image::GenericImageView;

pub fn load_texture(path: &str) -> (Vec<u8>, u32, u32) {
    let img = image::open(path).expect("Failed to load texture");
    let img = img.flipv();  // Opcional: voltear la imagen verticalmente si es necesario
    let (width, height) = img.dimensions();  // Obtenemos las dimensiones de la textura
    (img.to_rgba8().into_raw(), width, height)  // Retornamos los bytes de la textura y las dimensiones
}

pub fn get_texture_color(texture: &Vec<u8>, texture_width: u32, texture_height: u32, uv: [f32; 2]) -> Color {
    let tex_x = (uv[0] * texture_width as f32) as usize;
    let tex_y = (uv[1] * texture_height as f32) as usize;
    let pixel_index = (tex_y * texture_width as usize + tex_x) * 4;  // Suponiendo formato RGBA

    Color {
        r: texture[pixel_index],
        g: texture[pixel_index + 1],
        b: texture[pixel_index + 2],
    }
}

pub fn calculate_uv(point_on_cube: Vec3) -> [f32; 2] {
    let u = point_on_cube.x.fract();  // Coordenada U basada en X
    let v = point_on_cube.y.fract();  // Coordenada V basada en Y
    [u, v]
}
