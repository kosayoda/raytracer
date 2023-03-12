use std::num::NonZeroU32;

use rand::thread_rng;
use rand::Rng;

use crate::{
    config::{CameraConfig, ImageConfig},
    material::{Dielectric, Lambertian, Material, Metal},
    object::{Object, Sphere},
    primitive::{Color, Point},
    Config,
};

pub fn final_scene() -> Config {
    let ground = Material::from(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });

    // -- World --
    let mut world: Vec<Object> = Vec::new();
    world.push(Object::from(Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        ground.clone(),
    )));

    let mut rng = thread_rng();

    for _a in -11..11 {
        for _b in -11..11 {
            let a = _a as f32;
            let b = _b as f32;
            let choose_mat: f32 = rng.gen();
            let center = Point::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());

            if (center - Point::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::new_random(&mut rng) * Color::new_random(&mut rng);
                    world.push(Object::from(Sphere::new(
                        center,
                        0.2,
                        Material::from(Lambertian { albedo }),
                    )))
                } else if choose_mat < 0.95 {
                    world.push(Object::from(Sphere::new(
                        center,
                        0.2,
                        Material::from(Dielectric {
                            refractive_index: 1.5,
                        }),
                    )))
                } else {
                }
            }
        }
    }
    world.push(Object::from(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Material::from(Dielectric {
            refractive_index: 1.5,
        }),
    )));
    world.push(Object::from(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Material::from(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    )));
    world.push(Object::from(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Material::from(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    )));

    // -- Configuration
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_to = Point::new(0.0, 0.0, 0.0);

    Config {
        camera: CameraConfig {
            look_from,
            look_to,
            vertical_fov: 20.0,
            aperture: 0.1,
            focus_dist: Some(10.0),
        },
        image: ImageConfig {
            width: NonZeroU32::new(1200).unwrap(),
            height: NonZeroU32::new(800).unwrap(),
            samples_per_pixel: 10,
            max_ray_depth: 50,
        },
        world,
    }
}
