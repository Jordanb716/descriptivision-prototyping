use rand::random;
use std::time::Instant;

//use rand::Rng;

struct CartCoord {
	x: f32,
	y: f32,
	z: f32,
}

struct CylindricalPolarRay {
	angle: f32,
	height: f32,
}

fn main() {
	let eye_coordinates = CartCoord {
		x: 10.0,
		y: 0.0,
		z: 0.0,
	};

	let sphere_coordinates = CartCoord {
		x: 0.0,
		y: 0.0,
		z: 0.0,
	};

	let sphere_radius = 1.0;

	let mut ray_vec: Vec<CylindricalPolarRay> = Vec::new();

	/*for _ in 1..100000 {
		let temp_ray = CylindricalPolarRay {
			angle: random::<f32>() * 360.0_f32.to_radians(),
			height: random::<f32>() % 10.0,
		};
		ray_vec.push(temp_ray);
	}*/

	for i in 0..360 {
		let i = i as f32;
		let temp_ray = CylindricalPolarRay {
			angle: i.to_radians(),
			height: 0.0 /*random::<f32>() % 0.0*/,
		};
		ray_vec.push(temp_ray);
	}

	//Start test
	let start_time = Instant::now();

	for (i, ray) in ray_vec.iter().enumerate() {
		let result = cast_to_sphere(&eye_coordinates, &sphere_coordinates, sphere_radius, &ray);
		match result {
			Some(x) => println!("{}: {}", i, x),
			None => println!("{}: No Hit.", i),
		}
	}

	let test_time = start_time.elapsed();
	println!("Time taken: {:?}.", test_time);
	println!("Time per cast: {:?}.", test_time / 100000);
}

fn cast_to_sphere(
	eye_coord: &CartCoord,
	sphere_coord: &CartCoord,
	sphere_radius: f32,
	ray_direction: &CylindricalPolarRay,
) -> Option<f32> {
	//Convert cylindrical polar coordinates to cartesian.
	let (ray_y, ray_x) = ray_direction.angle.sin_cos();
	let ray_z = ray_direction.height;

	//Offset eye coordinates by target location so we can treat the sphere as the origin.
	let eye_x = eye_coord.x - sphere_coord.x;
	let eye_y = eye_coord.y - sphere_coord.y;
	let eye_z = eye_coord.z - sphere_coord.z;

	//Do raycast
	let a = ray_x.powi(2) + ray_y.powi(2) + ray_z.powi(2);
	let b = 2.0 * eye_x * ray_x + 2.0 * eye_y * ray_y + 2.0 * eye_z * ray_z;
	let c = eye_x.powi(2) + eye_y.powi(2) + eye_z.powi(2) - sphere_radius.powi(2);
	let result_one = (-1.0 * b + (b.powi(2) - 4.0 * a * c).sqrt()) / 2.0 * a;
	let result_two = (-1.0 * b - (b.powi(2) - 4.0 * a * c).sqrt()) / 2.0 * a;

	if result_one >= 0.0 {
		if result_two >= 0.0 {
			if result_one < result_two {
				return Some(result_one);
			} else {
				return Some(result_two);
			}
		} else {
			return Some(result_one);
		}
	} else if result_two >= 0.0 {
		return Some(result_two);
	} else {
		return None;
	}
}

/*
fn cast_to_cylinder() {

}
*/
