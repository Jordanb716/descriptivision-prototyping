mod primitive;

use colored::*;
use rand::random;
use std::time::Instant;
use primitive::coordinate::Cartesian as CartCoord;
use primitive::coordinate::Spherical as SphCoord;
use primitive::ray as ray;
use ray::SphericalPolarRay as SphRay;
use ray::CylindricalPolarRay as CylRay;
//use rand::Rng;

struct SpatialResolution {
	width:i16,
	height:i16
}

struct AngularResolution {
	horizontal:i16,
	vertical:i16
}

static mut min_found:f32 = 0.0;
static mut max_found:f32 = 0.0;

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

	let mut ray_vec: Vec<CylRay> = Vec::new();

	/*for _ in 1..100000 {
		let temp_ray = Cyl
Ray {
			angle: random::<f32>() * 360.0_f32.to_radians(),
			height: random::<f32>() % 10.0,
		};
		ray_vec.push(temp_ray);
	}*/

	for i in 0..360 {
		let i = i as f32;
		let temp_ray = CylRay {
			angle: i.to_radians(),
			height: 0.0 /*random::<f32>() % 0.0*/,
		};
		ray_vec.push(temp_ray);
	}

	//Start test
	let start_time = Instant::now();

	for (i, ray) in ray_vec.iter().enumerate() {
		let result = ray::cast_to_sphere(&eye_coordinates, &sphere_coordinates, sphere_radius, &ray);
		match result {
			Some(x) => println!("{}: {}", i, x),
			None => println!("{}: No Hit.", i),
		}
	}

	let test_time = start_time.elapsed();
	println!("Time taken: {:?}.", test_time);
	println!("Time per cast: {:?}.", test_time / 100000);
}

/*
fn cast_to_cylinder() {

}
*/

/*
direct spherical polar (horizontal angle, vertical angle)
assume the object is a sphere
field of view is an angle of view singular thus creating a square as this angle is horizontal and vertical

given these 3 draw the object in a grid
*/
fn spatial_resolution_scan_spherical(
	eye_coord: &CartCoord,
	direction:&SphRay, 
	object:&CartCoord, 
	sphere_radius: f32,
	fieldofview:&SphRay,
	resolution:&SpatialResolution,
) {

	let projection_radius = 1.0;

	let angle_one = SphRay {
		azimuthal:direction.azimuthal-fieldofview.azimuthal/2.0,
		polar:direction.polar
	};
	let angle_two = SphRay {
		azimuthal:direction.azimuthal,
		polar:direction.polar-fieldofview.polar/2.0
	};

	let horz_proj_rad = 1;
	let vert_proj_rad = 1;
	let point_horz = CartCoord {
		x: projection_radius*angle_one.polar.to_radians().sin()*angle_one.azimuthal.cos(),
		y: projection_radius*angle_one.polar.to_radians().sin()*angle_one.azimuthal.to_radians().sin(),
		z: projection_radius*angle_one.polar.to_radians().cos()
	};
	let point_vert = CartCoord {
		x: projection_radius*angle_two.polar.to_radians().sin()*angle_two.azimuthal.cos(),
		y: projection_radius*angle_two.polar.to_radians().sin()*angle_two.azimuthal.to_radians().sin(),
		z: projection_radius*angle_two.polar.to_radians().cos()
	};
	let point_center = CartCoord {
		x: projection_radius*direction.polar.to_radians().sin()*direction.azimuthal.cos(),
		y: projection_radius*direction.polar.to_radians().sin()*direction.azimuthal.to_radians().sin(),
		z: projection_radius*direction.polar.to_radians().cos()
	};

	let horizontal_direction = CartCoord {
		x:0.0,
		y:0.0,
		z:0.0
	};
	let vertical_direction = CartCoord {
		x:0.0,
		y:0.0,
		z:0.0
	};
	/*loop object cast in field of view direct*/
}

fn angular_resolution_scan_spherical(
	eye_coord: &CartCoord,
	direction:&SphRay, 
	object:&CartCoord, 
	sphere_radius: f32,
	fieldofview:&SphRay,
	resolution:&SpatialResolution
) -> Vec<Vec<f32>> {
	let projection_radius = 1.0;
	let start_ray = SphRay {
		azimuthal:direction.azimuthal-0.5*fieldofview.azimuthal, 
		polar:direction.polar-0.5*fieldofview.polar
	};
	let h_angle_delta = fieldofview.azimuthal / (resolution.width as f32);
	let v_angle_delta = fieldofview.polar / (resolution.height as f32);	
	let mut view:Vec<Vec<f32>> = std::vec::Vec::new();
	for i in 0..resolution.width {
		let row:Vec<f32> = std::vec::Vec::new();
		for j in 0..resolution.height {
			//convert to correct coords
			let fire_point = CartCoord::from(SphCoord {
				r:projection_radius,
				phi:start_ray.azimuthal+(i as f32)*h_angle_delta,
				theta:start_ray.polar+(j as f32)*v_angle_delta
			}) + *eye_coord;
			//fire ray and record result
			row.push(ray::cast_to_sphere(eye_coord, object, sphere_radius, &fire_point));
		}
	}
	return view;
}

fn angular_resolution_scan_cylindrical(
	eye_coord: &CartCoord,
	direction:&SphRay, 
	object:&CartCoord, 
	sphere_radius: f32,
	fieldofview:&SphRay,
	resolution:&SpatialResolution
) {
	let projection_radius = 1.0;
	let start_ray = SphRay {
		azimuthal:direction.azimuthal-0.5*fieldofview.azimuthal, 
		polar:direction.polar-0.5*fieldofview.polar
	};
	let h_angle_delta = fieldofview.azimuthal / (resolution.width as f32);
	let v_angle_delta = fieldofview.polar / (resolution.height as f32);	
	for i in 0..resolution.width {
		for j in 0..resolution.height {
			//convert to correct coords
			let fire_point = CartCoord::from(SphCoord {
				r:projection_radius,
				phi:start_ray.azimuthal+(i as f32)*h_angle_delta,
				theta:start_ray.polar+(j as f32)*v_angle_delta
			});
			//fire ray
			//record result
		}
	}
}

fn diplayGrid(vec: &Vec<Vec<u8>>) {
	let pixel_char = "█";
	let empty_char = " ".black();
	let diff = max_found - min_found;
	if diff <= 0.0 {
		return;
	}
	print!("\x1B[2J\x1B[1;1H");		//clear terminal
	for line in vec {
		for color in line {
			print!("{}",if color > &(0 as u8) { pixel_char.truecolor(*color,*color,*color) } else { empty_char });
		}
		println!("")
	}
}