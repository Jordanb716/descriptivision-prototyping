use std::convert::From;
use std::ops::Add;

pub struct Cartesian {      // The standard cartesian coordinate system
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

pub struct Cylindrical {    // The three coordinates (ρ, φ, z) of a point P are defined as:
	pub r: f32,             // The axial distance or radial distance ρ is the Euclidean distance from the z-axis to the point P.
	pub phi: f32,           // The azimuth φ is the angle between the reference direction on the chosen plane and the line from the origin to the projection of P on the plane.
	pub z: f32              // The axial coordinate or height z is the signed distance from the chosen plane to the point P.
}

pub struct Spherical {      // To define a spherical coordinate system, one must choose two orthogonal directions, the zenith and the azimuth reference, and an origin point in space. These choices determine a reference plane that contains the origin and is perpendicular to the zenith. The spherical coordinates of a point P are then defined as follows:
	pub r:f32,              // The radius or radial distance is the Euclidean distance from the origin O to P.
	pub theta:f32,          // The inclination (or polar angle) is the angle between the zenith direction and the line segment OP.
	pub phi:f32             // The azimuth (or azimuthal angle) is the signed angle measured from the azimuth reference direction to the orthogonal projection of the line segment OP on the reference plane.
}

impl From<Spherical> for Cartesian{
	fn from(item: Spherical
) -> Self {
	 Cartesian {
			x:item.r*item.phi.to_radians().cos()*item.theta.to_radians().sin(),
			y:item.r*item.phi.to_radians().sin()*item.theta.to_radians().sin(),
			z:item.r*item.theta.to_radians().cos()
		}
	}
}

impl From<Cylindrical> for Cartesian {
    fn from(item: Cylindrical) -> Self {
     Cartesian {
            x:0.0,                         //semi complex :(
            y:0.0,                         //semi complex :(
            z:0.0                          //semi complex :(
        }
    }
}

impl From<Spherical> for Cylindrical{
	fn from(item: Spherical
) -> Self {
		Cylindrical {
			r:0.0,                         //semi complex :(
			phi:item.phi,
			z:0.0,                         //semi complex :(
		}
	}
}

impl From <Cartesian> for Cylindrical {
    fn from(item: Cartesian) -> Self {
        Cylindrical {
            r:(item.x.powi(2)+item.y.powi(2)).sqrt(),
            phi:0.0,                       //semi complex :(
            z:item.z
        }
    }
}

impl From <Cartesian> for Spherical{
	fn from(item: Cartesian) -> Self {
		let radius = (item.x.powi(2)+item.y.powi(2)+item.z.powi(2)).sqrt();
	    return Spherical {
			r:radius,
			theta:(item.z/radius).acos(),
			phi:(item.y/item.z).atan()
        }
	}
}

impl From<Cylindrical> for Spherical {
    fn from(item: Cylindrical) -> Self {
        Spherical
     {
            r:0.0,                     //semi complex :(
            theta:(item.r.powi(2)+item.z.powi(2)).sqrt(),
            phi:item.phi
        }
    }
}

impl Add for Spherical {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r:self.r+other.r,
            theta:self.theta+other.theta,
            phi:self.phi+other.phi
        }
    }
}

impl Add for Cylindrical {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r:self.r+other.r,
            phi:self.phi+other.phi,
            z:self.z+other.z
        }
    }
}

impl Add for Cartesian {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x:self.x+other.x,
            y:self.y+other.y,
            z:self.z+other.z
        }
    }
}