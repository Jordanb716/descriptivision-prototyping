mod primitive;

use colored::*;
//use rand::random;
use primitive::coordinate::Cartesian as CartCoord;
use primitive::coordinate::Spherical as SphCoord;
use primitive::ray;
use ray::SphericalPolarRay as SphRay;
use std::time::Instant;
//use rand::Rng;

struct SpatialResolution {
    width: i16,
    height: i16,
}

fn main() {
    let eye_coordinates = CartCoord {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let sphere_coordinates = CartCoord {
        x: 10.0,
        y: 0.0,
        z: 0.0,
    };

    let sphere_radius = 1.0;

    let direction = SphRay {
        azimuthal: 0.0,
        polar: 0.0,
    };

    let fov = SphRay {
        azimuthal: 90.0,
        polar: 60.0,
    };

    let resolution = SpatialResolution {
        width: 31,
        height: 21,
    };

    /*let mut ray_vec: Vec<CylRay> = Vec::new();

    for _ in 1..100000 {
        let temp_ray = CylRay {
            angle: random::<f32>() * 360.0_f32.to_radians(),
            height: random::<f32>() % 10.0,
        };
        ray_vec.push(temp_ray);
    }

    for i in 0..360 {
        let i = i as f32;
        let temp_ray = CylRay {
            angle: i.to_radians(),
            height: 0.0 /*random::<f32>() % 0.0*/,
        };
        ray_vec.push(temp_ray);
    }*/

    //Start test
    let start_time = Instant::now();

    /*for (i, ray) in ray_vec.iter().enumerate() {
        let result = ray::cast_to_sphere(&eye_coordinates, &sphere_coordinates, sphere_radius, &ray);
        match result {
            Some(x) => println!("{}: {}", i, x),
            None => println!("{}: No Hit.", i),
        }
    }*/

    diplay_grid(angular_resolution_scan_spherical(
        &eye_coordinates,
        &direction,
        &sphere_coordinates,
        sphere_radius,
        &fov,
        &resolution,
    ));

    let test_time = start_time.elapsed();
    println!("Time taken: {:?}.", test_time);
    println!("Time per cast: {:?}.", test_time / 100000);
}

/*
fn cast_to_cylinder() {

}
*/

fn angular_resolution_scan_spherical(
    eye_coord: &CartCoord,
    direction: &SphRay,
    object: &CartCoord,
    sphere_radius: f32,
    fieldofview: &SphRay,
    resolution: &SpatialResolution,
) -> Vec<Vec<Option<f32>>> {
    let projection_radius = 1.0;
    let start_ray = SphRay {
        azimuthal: direction.azimuthal - 0.5 * fieldofview.azimuthal,
        polar: direction.polar - 0.5 * fieldofview.polar,
    };
    let h_angle_delta = fieldofview.azimuthal / (resolution.width as f32);
    let v_angle_delta = fieldofview.polar / (resolution.height as f32);
    let mut view: Vec<Vec<Option<f32>>> = std::vec::Vec::new();
    for j in 0..resolution.height {
        let mut row: Vec<Option<f32>> = std::vec::Vec::new();
        for i in 0..resolution.width {
            //convert to correct coords
            let fire_point = CartCoord::from(SphCoord {
                r: projection_radius,
                phi: start_ray.azimuthal + (i as f32) * h_angle_delta,
                theta: start_ray.polar + (j as f32) * v_angle_delta,
            }) + *eye_coord;
            //fire ray and record result
            row.push(ray::cast_to_sphere(
                eye_coord,
                object,
                sphere_radius,
                &fire_point,
            ));
        }
        view.push(row);
    }
    return view;
}

fn diplay_grid(vec: Vec<Vec<Option<f32>>>) {
    let pixel_char = "█";
    let empty_char = " ".black();
    print!("\x1B[2J\x1B[1;1H"); //clear terminal
    for line in vec {
        for result in line {
            match result {
                Some(x) => print!("{}", pixel_char),
                None => print!("{}", empty_char),
            }
        }
        println!("")
    }
}
