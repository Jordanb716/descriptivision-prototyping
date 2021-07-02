// based on info from the Cylindrical Coordinated Wikipedia page
// https://en.wikipedia.org/wiki/Cylindrical_coordinate_system

struct CylCoord {       // The three coordinates (ρ, φ, z) of a point P are defined as:
	r: f32,             // The axial distance or radial distance ρ is the Euclidean distance from the z-axis to the point P.
	phi: f32,           // The azimuth φ is the angle between the reference direction on the chosen plane and the line from the origin to the projection of P on the plane.
	z: f32              // The axial coordinate or height z is the signed distance from the chosen plane to the point P.
}

impl From<SphereCoord> for CylCoord{
	fn from(item: SphereCoord) -> Self {
		CylCoord {
			r:,                         //semi complex :(
			phi:item.phi,
			z:,                         //semi complex :(
		}
	}
}

impl From<CartCoord> for CylCoord {
    fn from(item: CartCoord) -> Self {
        CylCoord {
            r:(item.x.powi(2)+item.y.powi(2)).sqrt(),
            phi:,                       //semi complex :(
            z:item.z
        }
    }
}

impl Add for CylCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x:self.x+other.x,
            y:self.y+other.y,
            z:self.z+other.z
        }
    }
}