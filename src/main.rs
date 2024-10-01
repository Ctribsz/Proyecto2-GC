mod framebuffer;
mod ray_intersect;
mod cube;
mod color;
mod camera;
mod light;
mod material;
mod texture;
mod diorama;
mod constants;

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
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Crear un "output stream" para el audio
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Cargar el archivo de música
    let file = File::open("assets/music.wav").unwrap();  // Cambia el nombre del archivo de música
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Hacer que el archivo de música se repita indefinidamente
    let repeated_source = source.repeat_infinite();
    sink.append(repeated_source);

    // Controlar el volumen
    sink.set_volume(1.5);

    let window_width = 800;
    let window_height = 600;
    
    // Reducimos el tamaño del framebuffer a la mitad de la ventana
    let framebuffer_scale_factor = constants::FRAMEBUFFER_SCALE_FACTOR;  // Renderizamos a la mitad de la resolución
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

    let mut light = Light::new(
        Vec3::new(2.5, 3.0, 2.5),  // Ajusta la posición de la luz
        Color::new(128, 255, 128),   // Color de la luz
        constants::INTENSITY  // Intensidad de la luz
    );                    

    let rotation_speed = PI/5.0;
    let mut last_frame_time = Instant::now();

    let mut camera = Camera::new(
        Vec3::new(0.0, 10.0, -10.0),  // Cambiamos la posición de la cámara (más alto y alejado)
        Vec3::new(0.0, 5.0, 0.0),     // Apuntamos hacia el centro del diorama
        Vec3::new(0.0, 1.0, 0.0),     // Vector "up", ajustado para la orientación
    );       

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Calcula delta time
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_time;
    
        let adjusted_rotation_speed = rotation_speed * delta_time;
    
        // Movimientos de la cámara usando el delta time
        if window.is_key_down(Key::A) {
            camera.orbit(adjusted_rotation_speed, 0.0);
        }
        if window.is_key_down(Key::D) {
            camera.orbit(-adjusted_rotation_speed, 0.0);
        }
        if window.is_key_down(Key::W) {
            camera.orbit(0.0, -adjusted_rotation_speed);
        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, adjusted_rotation_speed);
        }

        // Cambiar la intensidad con teclas '+' y '-'
        if window.is_key_down(Key::Equal) {  // Aumentar intensidad con '+'
            light.intensity = (light.intensity + 0.1).min(50.0);  // Limitar la intensidad máxima
        }

        if window.is_key_down(Key::Minus) {  // Disminuir intensidad con '-'
            light.intensity = (light.intensity - 0.1).max(0.0);  // Evitar que sea negativa
        }

        // Cambiar color con teclas numéricas
        if window.is_key_down(Key::Key1) {  // Amarillo reconfortante
            light.color = Color::new(255, 223, 128);
        }
        if window.is_key_down(Key::Key2) {  // Azul de anochecer
            light.color = Color::new(128, 128, 255);
        }
        if window.is_key_down(Key::Key3) {  // Verde brillante
            light.color = Color::new(128, 255, 128);
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
    
        std::thread::sleep(Duration::from_millis(16));  // Aproximadamente 60 FPS
    }

    // Detener la música cuando el ciclo de la ventana se detiene
    sink.stop();
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube], light: &Light, distance: f32) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;
    let mut hit_cube: Option<&Cube> = None;

    // Primer paso: Encontrar el cubo más cercano
    for object in objects {
        // Verificamos si el objeto está cerca de la cámara antes de calcular intersecciones
        let distance_to_camera = (object.min - *ray_origin).magnitude();
        if distance_to_camera > distance {  // Omitimos objetos que están demasiado lejos
            continue;
        }

        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
            hit_cube = Some(object);
        }
    }

    // Si no hay intersección, devolvemos el color de fondo
    if !intersect.is_intersecting {
        return Color::new(64, 128, 96);  // Fondo celeste o el color de fondo deseado
    }

    // Si hay intersección, lanzamos un rayo de sombra desde el punto de intersección hacia la luz
    let shadow_ray_origin = intersect.point;
    let shadow_ray_direction = (light.position - shadow_ray_origin).normalize();
    let light_distance = (light.position - shadow_ray_origin).magnitude();

    // Bandera para determinar si el punto está en sombra
    let mut in_shadow = false;

    for object in objects {
        if let Some(cube) = hit_cube {
            if cube != object {  // Nos aseguramos de no comparar el cubo con sí mismo
                let shadow_intersection = object.ray_intersect(&shadow_ray_origin, &shadow_ray_direction);
                if shadow_intersection.is_intersecting && shadow_intersection.distance < light_distance {
                    in_shadow = true;  // Si encontramos una intersección, el punto está en sombra
                    break;
                }
            }
        }
    }

    // Dirección de la luz hacia el punto de intersección
    let light_dir = (light.position - intersect.point).normalize();

    // Intensidad difusa basada en el ángulo entre la normal y la dirección de la luz
    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);

    // Color base (ya sea de la textura o el color difuso del material)
    let base_color = if let Some(texture) = &intersect.material.texture {
        // Calcular las coordenadas UV basadas en el cubo y la intersección
        let uv = calculate_uv(intersect.point, intersect.normal, hit_cube.unwrap().min, hit_cube.unwrap().max);
        get_texture_color(texture, intersect.material.texture_width, intersect.material.texture_height, uv)
    } else {
        intersect.material.diffuse.unwrap_or(Color::new(255, 255, 255))  // Si no hay textura, usa el color difuso
    };

    // Luz ambiental ajustada
    let ambient_intensity = 0.5;  // Menor luz ambiente

    // Si está en sombra, solo aplicamos la luz ambiente
    if in_shadow {
        return base_color * ambient_intensity;
    }

    // Cálculo de reflect (iluminación especular)
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal);
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light.intensity;

    // Combinamos luz difusa, ambiente y reflect (especular)
    let diffuse = base_color * intersect.material.albedo[0] * (diffuse_intensity + ambient_intensity);
    let final_color = diffuse + specular;

    final_color

}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = std::f32::consts::PI / 4.0;
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
            let distance = 20.0; // O cualquier valor que haga sentido en tu lógica
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light, distance);


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