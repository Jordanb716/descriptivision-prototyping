// based on info from the Spherical Coordinated Wikipedia page
// https://en.wikipedia.org/wiki/Spherical_coordinate_system

struct SphereCoord {    // To define a spherical coordinate system, one must choose two orthogonal directions, the zenith and the azimuth reference, and an origin point in space. These choices determine a reference plane that contains the origin and is perpendicular to the zenith. The spherical coordinates of a point P are then defined as follows:
	r:f32,              // The radius or radial distance is the Euclidean distance from the origin O to P.
	theta:f32,          // The inclination (or polar angle) is the angle between the zenith direction and the line segment OP.
	phi:f32             // The azimuth (or azimuthal angle) is the signed angle measured from the azimuth reference direction to the orthogonal projection of the line segment OP on the reference plane.
}

impl From<CartCoord> for SphereCoord{
	fn from(item: CartCoord) -> Self {
		radius = (item.x.powi(2)+item.y.powi(2)+item.z.powi(2)).sqrt()
		SphereCoord {
			r:radius,
			theta:(item.z/radius).acos(),
			phi:(item.y/item.z).atan()}
	}
}

impl From<CylCoord> for SphereCoord {
    fn from(item: CylCoord) -> Self {
        SphereCoord {
            r:,                     //semi complex :(
            theta:(item.r.powi(2)+item.z.powi(2)).sqrt(),
            phi:item.phi
        }
    }
}

impl Add for SphereCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r:self.r+other.r,
            theta:self.theta+other.theta,
            phi:self.polar+other.polar
        }
    }
}