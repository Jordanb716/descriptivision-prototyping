use coordinate::Cartesian;

mod coordinate;

pub struct CylindricalPolarRay {
	pub angle: f32,
	pub height: f32,
}

pub struct SphericalPolarRay {
	pub azimuthal: f32,
	pub polar: f32
}

pub fn cast_to_sphere(
	eye_coord: &Cartesian,
	sphere_coord: &Cartesian,
	sphere_radius: f32,
	ray_direction: &Cartesian,
) -> Option<f32> {

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