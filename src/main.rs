use rand::random;
use std::time::Instant;
use coordinate::Cartesian;
use coordinate::Spherical;
use ray::SphericalPolarRay;
use ray::CylindricalPolarRay;
//use rand::Rng;

mod ray;
mod coordinate; 

struct SpatialResolution {
	width:i16,
	height:i16
}

struct AngularResolution {
	horizontal:i16,
	vertical:i16
}

fn main() {
	let eye_coordinates = Cartesian {
		x: 10.0,
		y: 0.0,
		z: 0.0,
	};

	let sphere_coordinates = Cartesian {
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
	eye_coord: &Cartesian,
	direction:&SphericalPolarRay, 
	object:&Cartesian, 
	sphere_radius: f32,
	fieldofview:&SphericalPolarRay,
	resolution:&SpatialResolution
) -> Option<f32> {

	let projection_radius = 1.0;

	let angle_one = SphericalPolarRay {
		horz_angle:direction.horz_angle-fieldofview.horz_angle/2,
		vert_angle:direction.vert_angle
	};
	let angle_two = SphericalPolarRay {
		horz_angle:direction.horz_angle,
		vert_angle:direction.vert_angle-fieldofview.vert_angle/2
	};

	let horz_proj_rad = 1;
	let vert_proj_rad = 1;
	let point_horz = Cartesian {
		x: projection_radius*angle_one.polar.to_radians.sin()*angle_one.azimuthal.cos(),
		y: projection_radius*angle_one.polar.to_radians.sin()*angle_one.azimuthal.to_radians.sin(),
		z: projection_radius*angle_one.polar.to_radians.cos()
	};
	let point_vert = Cartesian {
		x: projection_radius*angle_two.polar.to_radians.sin()*angle_two.azimuthal.cos(),
		y: projection_radius*angle_two.polar.to_radians.sin()*angle_two.azimuthal.to_radians.sin(),
		z: projection_radius*angle_two.polar.to_radians.cos()
	};
	let point_center = Cartesian {
		x: projection_radius*direction.polar.to_radians.sin()*direction.azimuthal.cos(),
		y: projection_radius*direction.polar.to_radians.sin()*direction.azimuthal.to_radians.sin(),
		z: projection_radius*direction.polar.to_radians.cos()
	};

	let horizontal_direction = Cartesian {
		x:0,
		y:0,
		z:0
	};
	let vertical_direction = Cartesian {
		x:0,
		y:0,
		z:0
	};
	/*loop object cast in field of view direct*/
}

fn angular_resolution_scan_spherical(
	eye_coord: &Cartesian,
	direction:&SphericalPolarRay, 
	object:&Cartesian, 
	sphere_radius: f32,
	fieldofview:&SphericalPolarRay,
	resolution:&SpatialResolution
) -> Option<f32> {
	let projection_radius = 1.0;
	let start_ray = SphericalPolarRay {
		azimuthal:direction.azimuthal-0.5*fieldofview.azimuthal, 
		polar:direction.polar-0.5*fieldofview.polar
	};
	let h_angle_delta = fieldofview.azimuthal / resolution.width;
	let v_angle_delta = fieldofview.polar / resolution.height;	
	for i in 0..resolution.width {
		for j in 0..resolution.height {
			//convert to correct coords
			let fire_point = Cartesian::from(Spherical {
				r:projection_radius,
				azimuthal:start_ray.azimuthal+i*h_angle_delta,
				polar:start_ray.polar+j*v_angle_delta
			}) + eye_coord;
			//fire ray
			let result = cast_to_sphere(eye_coord, object, sphere_radius, fire_point);
			//record result
		}
	}
}

fn angular_resolution_scan_cylindrical(
	eye_coord: &Cartesian,
	direction:&SphericalPolarRay, 
	object:&Cartesian, 
	sphere_radius: f32,
	fieldofview:&SphericalPolarRay,
	resolution:&SpatialResolution
) -> Option<f32> {
	projection_radius = 1.0;
	start_ray = SphericalPolarRay {
		azimuthal:direction.azimuthal-0.5*fieldofview.azimuthal, 
		polar:direction.polar-0.5*fieldofview.polar
	};
	for i in 0..resolution.width {
		for j in 0..resolution.height {
			//convert to correct coords
			fire_point = Cartesian::from(Spherical {
				r:projection_radius,
				azimuthal:start_ray.azimuthal+i*angle_delta,
				polar:start_ray.polar+j*angle_delta
			});
			//fire ray
			//record result
		}
	}
}