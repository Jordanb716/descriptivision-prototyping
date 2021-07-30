use std::convert::From;
use std::ops::Add;

const PRECISION: usize = 2;

/// The standard cartesian coordinate system
pub struct Cartesian {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// The three coordinates (ρ, φ, z) of a point P are defined as:
/// The axial distance or radial distance ρ is the Euclidean distance from the z-axis to the point P.
/// The azimuth φ is the angle between the reference direction on the chosen plane and the line from the origin to the projection of P on the plane.
/// The axial coordinate or height z is the signed distance from the chosen plane to the point P.
pub struct Cylindrical {
    pub r: f32,
    pub azimuthal: f32,
    pub z: f32,
}

/// To define a spherical coordinate system, one must choose two orthogonal directions, the zenith and the azimuth reference, and an origin point in space. These choices determine a reference plane that contains the origin and is perpendicular to the zenith. The spherical coordinates of a point P are then defined as follows:
/// The radius or radial distance is the Euclidean distance from the origin O to P.
/// The inclination (or polar angle) is the angle between the zenith direction and the line segment OP.
/// The azimuth (or azimuthal angle) is the signed angle measured from the azimuth reference direction to the orthogonal projection of the line segment OP on the reference plane.
///Spherical coordinates (r, θ, φ) as often used in mathematics: radial distance r, azimuthal angle θ, and polar angle φ. The meanings of θ and φ have been swapped compared to the physics convention. As in physics, ρ (rho) is often used instead of r, to avoid confusion with the value r in cylindrical and 2D polar coordinates.
pub struct Spherical {
    pub r: f32,
    pub polar: f32,
    pub azimuthal: f32,
}

///Implementation of the std::convert::From function for the struct primitive::coordinate::Cartesian
///Allows conversion from Spherical coordinates to Cartesian coordinates
impl From<Spherical> for Cartesian {
    fn from(item: Spherical) -> Self {
        Cartesian {
            x: item.r * item.azimuthal.cos() * item.polar.sin(),
            y: item.r * item.azimuthal.sin() * item.polar.sin(),
            z: item.r * item.polar.cos()
        }
    }
}

///Implementation of the std::convert::From function for the struct primitive::coordinate::Cartesian
///Allows conversion from Cylindrical coordinates to Cartesian coordinates
///semi complex :(
///semi complex :(
///semi complex :(
impl From<Cylindrical> for Cartesian {
    fn from(item: Cylindrical) -> Self {
        Cartesian {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

///Implementation of the std::convert::From function for the struct primitive::coordinate::Cylindrical
///Allows conversion from Spherical coordinates to Cylindrical coordinates
///semi complex :(
///semi complex :(
impl From<Spherical> for Cylindrical {
    fn from(item: Spherical) -> Self {
        Cylindrical {
            r: 0.0,
            azimuthal: item.azimuthal,
            z: 0.0,
        }
    }
}

///Implementation of the std::convert::From function for the struct primitive::coordinate::Cylindrical
///Allows conversion from Cartesian coordinates to Cylindrical coordinates
///semi complex :(
impl From<Cartesian> for Cylindrical {
    fn from(item: Cartesian) -> Self {
        Cylindrical {
            r: (item.x.powi(2) + item.y.powi(2)).sqrt(),
            azimuthal: 0.0,
            z: item.z,
        }
    }
}

///Implementation of the std::convert::From function for the struct primitive::coordinate::Spherical
///Allows conversion from Cartesian coordinates to Spherical coordinates
impl From<Cartesian> for Spherical {
    fn from(item: Cartesian) -> Self {
        let radius = (item.x.powi(2) + item.y.powi(2) + item.z.powi(2)).sqrt();
        return Spherical {
            r: radius,
            polar: (item.z / radius).acos(),
            azimuthal: (item.y / item.z).atan(),
        };
    }
}

///Implementation of the std::convert::From function for the struct primitive::coordinate::Spherical
///Allows conversion from Cylindrical coordinates to Spherical coordinates
///semi complex :(
impl From<Cylindrical> for Spherical {
    fn from(item: Cylindrical) -> Self {
        Spherical {
            r: 0.0,
            polar: (item.r.powi(2) + item.z.powi(2)).sqrt(),
            azimuthal: item.azimuthal,
        }
    }
}

///Implementation of the Copy function for the struct primitive::coordinate::Spherical
impl Copy for Cartesian {}

///Implementation of the Copy function for the struct primitive::coordinate::Spherical
impl Copy for Spherical {}

///Implementation of the Copy function for the struct primitive::coordinate::Spherical
impl Copy for Cylindrical {}

///Implementation of the Clone function for the struct primitive::coordinate::Spherical
impl Clone for Cartesian {
    fn clone(&self) -> Cartesian {
        *self
    }
}

///Implementation of the Clone for the struct primitive::coordinate::Spherical
impl Clone for Spherical {
    fn clone(&self) -> Spherical {
        *self
    }
}

///Implementation of the Clone for the struct primitive::coordinate::Spherical
impl Clone for Cylindrical {
    fn clone(&self) -> Cylindrical {
        *self
    }
}

///Implementation of the std::ops::Add function for the struct primitive::coordinate::Spherical
///this implements the addition operation for shperical points
///(ρ1, φ1, θ1) + (ρ2, φ2, θ2) = (ρ1+ρ2, φ1+φ2, θ1+θ2) = (ρ3, φ3, θ3)
impl Add for Spherical {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            polar: self.polar + other.polar,
            azimuthal: self.azimuthal + other.azimuthal,
        }
    }
}

///Implementation of the std::ops::Add function for the struct primitive::coordinate::Cylindrical
///this implements the addition operation for cylindrical points
///(ρ1, φ1, z1) + (ρ2, φ2, z2) = (ρ1+ρ2, φ1+φ2, z1+z2) = (ρ3, φ3, z3)
impl Add for Cylindrical {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            azimuthal: self.azimuthal + other.azimuthal,
            z: self.z + other.z,
        }
    }
}

///Implementation of the std::ops::Add function for the struct primitive::coordinate::Cartesian
///this implements the addition operation for cartesianal points
///(x1, y1, z1) + (x2, y2, z2) = (x1+x2, y1+y2, z1+z2) = (x3, y3, z3)
impl Add for Cartesian {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

///Implementation of the std::ops::Mul function for the struct primitive::coordinate::Cartesian
///this implements the scalar multiplication operation for cartesianal points
///k(x, y, z) = (kx, ky, kz) such that k is an element of the real numbers ie k is a scalar value
impl Mul for Cartesian {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x:scalar * self.x,
            y:scalar * self.y,
            z:scalar * self.z,
        }
    }

    fn mul(self, scalar: i32) -> Self {
        mul(self, scalar as f32)
    }
}

///Implementation of the std::ops::Mul function for the struct primitive::coordinate::Cylindrical
///this implements the scalar multiplication operation for cylindrical points
///k(ρ, φ, z) = (kρ, kφ, kz) such that k is an element of the real numbers ie k is a scalar value
impl Mul for Cylindrical {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            r:scalar * self.r,
            azimuthal:scalar * self.azimuthal,
            z:scalar * self.z,
        }
    }

    fn mul(self, scalar: i32) -> Self {
        mul(self, scalar as f32)
    }
}

///Implementation of the std::ops::Mul function for the struct primitive::coordinate::Spherical
///this implements the scalar multiplication operation for spherical points
///k(ρ, φ, θ) = (kρ, kφ, kθ) such that k is an element of the real numbers ie k is a scalar value
impl Mul for Spherical {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            r:scalar * self.r,
            azimuthal:scalar * self.azimuthal,
            polar:scalar * self.polar,
        }
    }

    fn mul(self, scalar: i32) -> Self {
        mul(self, scalar as f32)
    }
}

///Implementation of the std::ops::Sub function for the struct primitive::coordinate::Cartesian
///this implements the subtraction operation for cartesian points
///(x1, y1, z1) - (x2, y2, z2) = (x1-x2, y1-y2, z1-z2) = (x3, y3, z3)
impl Sub for Cartesian {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -1 * other 
    }
}

///Implementation of the std::ops::Sub function for the struct primitive::coordinate::Cylindrical
///this implements the subtraction operation for cylindrical points
///(ρ1, φ1, z1) - (ρ2, φ2, z2) = (ρ1-ρ2, φ1-φ2, z1-z2) = (ρ3, φ3, z3)
impl Sub for Cylindrical {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -1 * other 
    }
}

///Implementation of the std::ops::Sub function for the struct primitive::coordinate::Spherical
///this implements the subtraction operation for spherical points
///(ρ1, φ1, θ1) - (ρ2, φ2, θ2) = (ρ1-ρ2, φ1-φ2, θ1-θ2) = (ρ3, φ3, θ3)
impl Sub for Spherical {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -1 * other 
    }
}

///Implementation of the std::fmt::Display function for the struct primitive::coordinate::Cartesian
///pretty prints struct values in human readable coordinates
impl std::fmt::Display for Cartesian {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"(x:{},y:{},z:{})",self.x,self.y,self.z)
    }
}

///Implementation of the std::fmt::Display function for the struct primitive::coordinate::Spherical
///pretty prints struct values in human readable coordinates
impl std::fmt::Display for Spherical {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"(r:{},θ:{:.*},φ:{:.*})",self.r,PRECISION,self.polar.to_degrees(),PRECISION,self.azimuthal.to_degrees())
    }
}

///Implementation of the std::fmt::Display function for the struct primitive::coordinate::Cylindrical
///pretty prints struct values in human readable coordinates
impl std::fmt::Display for Cylindrical {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"(ρ:{},φ:{:.*},z:{})",self.r,PRECISION,self.azimuthal.to_degrees(),self.z)
    }
}