mod framebuffer;
mod ray_intersect;
mod cube;
mod color;
mod camera;
mod light;
mod material;
mod texture;

use minifb::{ Window, WindowOptions, Key };
use nalgebra_glm::{Vec3, normalize};
use std::time::Duration;
use std::f32::consts::PI;
use crate::color::Color;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::cube::Cube;
use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use crate::light::Light;
use crate::material::Material;
use std::time::Instant;
use crate::texture::{calculate_uv, get_texture_color, load_texture};

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube], light: &Light) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return Color::new(135, 206, 235); // Color de fondo
    }

    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal);

    // Calcular iluminación difusa y especular
    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    
    let mut base_color = Color::new(0, 0, 0);

    // Si el material tiene una textura, obtenemos el color de la textura
    if let Some(texture) = &intersect.material.texture {
        let uv = calculate_uv(intersect.point); // Función para calcular las coordenadas UV
        base_color = get_texture_color(texture, intersect.material.texture_width, intersect.material.texture_height, uv);
    } else if let Some(diffuse_color) = &intersect.material.diffuse {
        base_color = *diffuse_color;  // Si no hay textura, usamos el color difuso
    }

    // Ajustamos el color de la textura o difuso basado en la iluminación
    let diffuse = base_color * intersect.material.albedo[0] * diffuse_intensity;
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light.intensity;

    diffuse + specular
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let rotated_direction = camera.base_change(&ray_direction);

            // Ahora lanzamos rayos hacia los cubos
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light);

            framebuffer.set_current_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(20);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Diorama",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Cargar texturas para los cubos
    let (grass_texture, grass_width, grass_height) = load_texture("assets/grass_carried.png");
    let (dirt_texture, dirt_width, dirt_height) = load_texture("assets/wool_black.png");

    // Crear materiales con texturas
    let grass_material = Material {
        diffuse: None,
        texture: Some(grass_texture),
        texture_width: grass_width,
        texture_height: grass_height,
        specular: 50.0,
        albedo: [0.9, 0.1],
    };

    let dirt_material = Material {
        diffuse: None,
        texture: Some(dirt_texture),
        texture_width: dirt_width,
        texture_height: dirt_height,
        specular: 30.0,
        albedo: [0.8, 0.2],
    };

    // Crear los cubos
    let objects = [
        Cube {
            min: Vec3::new(-0.5, -0.5, -2.0),
            max: Vec3::new(0.5, 0.5, -1.0),
            material: grass_material,
        },
        Cube {
            min: Vec3::new(1.0, -0.5, -3.0),
            max: Vec3::new(2.0, 0.5, -2.0),
            material: dirt_material,
        },
    ];

    let light = Light::new(
        Vec3::new(10.0, 10.0, 10.0),
        Color::new(255, 255, 255),
        1.0
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
    
        framebuffer.clear();  // Limpiar el framebuffer con el color de fondo antes de renderizar
    
        render(&mut framebuffer, &objects, &camera, &light);
    
        // Actualizar la ventana con el framebuffer
        if let Err(e) = window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height) {
            println!("Error al actualizar el buffer: {:?}", e);
        }
    
        std::thread::sleep(frame_delay);
    }    
}