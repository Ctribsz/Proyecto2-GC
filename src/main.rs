mod framebuffer;
mod ray_intersect;
mod cube;
mod color;
mod camera;
mod light;
mod material;
mod texture;
mod diorama;

use minifb::{ Window, WindowOptions, Key };
use nalgebra_glm::Vec3;
use std::time::Duration;
use std::f32::consts::PI;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use crate::light::Light;
use std::time::Instant;
use crate::diorama::generate_diorama;
use rayon::prelude::*;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::texture::{calculate_uv, get_texture_color};
use crate::cube::Cube;

fn main() {
    let window_width = 800;
    let window_height = 600;
    
    // Reducimos el tamaño del framebuffer a la mitad de la ventana
    let framebuffer_scale_factor = 0.75;  // Renderizamos a la mitad de la resolución
    let framebuffer_width = (window_width as f32 * framebuffer_scale_factor) as usize;
    let framebuffer_height = (window_height as f32 * framebuffer_scale_factor) as usize;
    
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Diorama - Upscaled",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Llamar a la función que genera el diorama manualmente
    let objects = generate_diorama();

    let light = Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Color::new(255, 255, 255),
        3.0
    );    

    let rotation_speed = PI/5.0;
    let mut last_frame_time = Instant::now();

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );   

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Calcula delta time
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_time;
    
        let adjusted_rotation_speed = rotation_speed * delta_time;
    
        // Movimientos de la cámara usando el delta time
        if window.is_key_down(Key::Left) {
            camera.orbit(adjusted_rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(-adjusted_rotation_speed, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -adjusted_rotation_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, adjusted_rotation_speed);
        }
    
        framebuffer.clear();  // Limpiar el framebuffer reducido antes de renderizar
    
        // Renderizamos en el framebuffer reducido
        render(&mut framebuffer, &objects, &camera, &light);

        // Escalar el framebuffer reducido al tamaño completo de la ventana
        let scaled_buffer = upscale_framebuffer(&framebuffer, window_width, window_height);
    
        // Actualizar la ventana con el buffer escalado
        if let Err(e) = window.update_with_buffer(&scaled_buffer, window_width, window_height) {
            println!("Error al actualizar el buffer: {:?}", e);
        }
    
        std::thread::sleep(Duration::from_millis(10));  // Aproximadamente 60 FPS
    }
}


fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube], light: &Light) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;
    let mut hit_cube: Option<&Cube> = None;

    for object in objects {
        // Verificamos si el objeto está cerca de la cámara antes de calcular intersecciones
        let distance_to_camera = (object.min - *ray_origin).magnitude();
        if distance_to_camera > 20.0 {  // Omitimos objetos que están demasiado lejos (ajusta la distancia según la necesidad)
            continue;
        }

        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
            hit_cube = Some(object);
        }
    }

    if !intersect.is_intersecting {
        return Color::new(135, 206, 235);  // Color de fondo si no hay intersección
    }

    // Aplicamos luz difusa, luz ambiente y reflect
    if let Some(cube) = hit_cube {
        // Dirección de la luz hacia el punto de intersección
        let light_dir = (light.position - intersect.point).normalize();

        // Intensidad difusa basada en el ángulo entre la normal y la dirección de la luz
        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);

        // Intensidad de luz ambiente
        let ambient_intensity = 0.5;  // Ajusta este valor si deseas más o menos luz ambiente

        // Color base (ya sea de la textura o el color difuso del material)
        let base_color = if let Some(texture) = &intersect.material.texture {
            let uv = calculate_uv(intersect.point, intersect.normal, cube.min, cube.max);
            get_texture_color(texture, intersect.material.texture_width, intersect.material.texture_height, uv)
        } else {
            intersect.material.diffuse.unwrap_or(Color::new(255, 255, 255))  // Si no hay textura, usa el color difuso
        };

        // Cálculo de reflect (iluminación especular)
        let view_dir = (ray_origin - intersect.point).normalize();  // Dirección hacia la cámara
        let reflect_dir = reflect(&-light_dir, &intersect.normal);  // Vector reflejado
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);  // Intensidad especular
        let specular = light.color * intersect.material.albedo[1] * specular_intensity * light.intensity;  // Componente especular

        // Combinamos luz difusa, ambiente y reflect (especular)
        let diffuse = base_color * intersect.material.albedo[0] * (diffuse_intensity + ambient_intensity);
        let final_color = diffuse + specular;  // Suma la luz especular

        return final_color;  // Devolvemos el color final con luz difusa, ambiente y especular
    }

    // Si algo sale mal, devolver un color predeterminado
    Color::new(255, 0, 0)
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = std::f32::consts::PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    // Paralelizar el cálculo por filas usando `par_iter_mut`
    framebuffer.buffer.par_chunks_mut(framebuffer.width).enumerate().for_each(|(y, row)| {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = nalgebra_glm::normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let rotated_direction = camera.base_change(&ray_direction);

            // Lanzamos rayos hacia los cubos y calculamos el color
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light);

            // Establecemos el color en el framebuffer
            row[x] = pixel_color.to_hex();
        }
    });
}

/// Función para escalar el framebuffer reducido al tamaño de la ventana completa
pub fn upscale_framebuffer(framebuffer: &Framebuffer, target_width: usize, target_height: usize) -> Vec<u32> {
    let mut scaled_buffer = vec![0; target_width * target_height];
    let x_ratio = framebuffer.width as f32 / target_width as f32;
    let y_ratio = framebuffer.height as f32 / target_height as f32;

    for y in 0..target_height {
        for x in 0..target_width {
            let src_x = (x as f32 * x_ratio).floor() as usize;
            let src_y = (y as f32 * y_ratio).floor() as usize;

            let src_index = src_y * framebuffer.width + src_x;
            let dest_index = y * target_width + x;

            scaled_buffer[dest_index] = framebuffer.buffer[src_index];
        }
    }

    scaled_buffer
}