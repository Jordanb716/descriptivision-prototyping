use std::convert::From;
use std::ops::Add;

const PRECISION: usize = 2;

pub struct Cartesian {
    // The standard cartesian coordinate system
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Cylindrical {
    // The three coordinates (ρ, φ, z) of a point P are defined as:
    pub r: f32, // The axial distance or radial distance ρ is the Euclidean distance from the z-axis to the point P.
    pub azimuthal: f32, // The azimuth φ is the angle between the reference direction on the chosen plane and the line from the origin to the projection of P on the plane.
    pub z: f32, // The axial coordinate or height z is the signed distance from the chosen plane to the point P.
}

pub struct Spherical {
    // To define a spherical coordinate system, one must choose two orthogonal directions, the zenith and the azimuth reference, and an origin point in space. These choices determine a reference plane that contains the origin and is perpendicular to the zenith. The spherical coordinates of a point P are then defined as follows:
    pub r: f32, // The radius or radial distance is the Euclidean distance from the origin O to P.
    pub polar: f32, // The inclination (or polar angle) is the angle between the zenith direction and the line segment OP.
    pub azimuthal: f32, // The azimuth (or azimuthal angle) is the signed angle measured from the azimuth reference direction to the orthogonal projection of the line segment OP on the reference plane.
}

impl From<Spherical> for Cartesian {
    fn from(item: Spherical) -> Self {
        Cartesian {
            x: item.r * item.azimuthal.cos() * item.polar.sin(),
            y: item.r * item.azimuthal.sin() * item.polar.sin(),
            z: item.r * item.polar.cos()
        }
    }
}

impl From<Cylindrical> for Cartesian {
    fn from(item: Cylindrical) -> Self {
        Cartesian {
            x: 0.0, //semi complex :(
            y: 0.0, //semi complex :(
            z: 0.0, //semi complex :(
        }
    }
}

impl From<Spherical> for Cylindrical {
    fn from(item: Spherical) -> Self {
        Cylindrical {
            r: 0.0, //semi complex :(
            azimuthal: item.azimuthal,
            z: 0.0, //semi complex :(
        }
    }
}

impl From<Cartesian> for Cylindrical {
    fn from(item: Cartesian) -> Self {
        Cylindrical {
            r: (item.x.powi(2) + item.y.powi(2)).sqrt(),
            azimuthal: 0.0, //semi complex :(
            z: item.z,
        }
    }
}

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

impl From<Cylindrical> for Spherical {
    fn from(item: Cylindrical) -> Self {
        Spherical {
            r: 0.0, //semi complex :(
            polar: (item.r.powi(2) + item.z.powi(2)).sqrt(),
            azimuthal: item.azimuthal,
        }
    }
}

impl Copy for Cartesian {}

impl Copy for Spherical {}

impl Copy for Cylindrical {}

impl Clone for Cartesian {
    fn clone(&self) -> Cartesian {
        *self
    }
}

impl Clone for Spherical {
    fn clone(&self) -> Spherical {
        *self
    }
}

impl Clone for Cylindrical {
    fn clone(&self) -> Cylindrical {
        *self
    }
}

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

impl std::fmt::Display for Cartesian {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"(x:{},y:{},z:{})",self.x,self.y,self.z)
    }
}

impl std::fmt::Display for Spherical {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"(r:{},θ:{:.*},φ:{:.*})",self.r,PRECISION,self.polar.to_degrees(),PRECISION,self.azimuthal.to_degrees())
    }
}

impl std::fmt::Display for Cylindrical {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"(ρ:{},φ:{:.*},z:{})",self.r,PRECISION,self.azimuthal.to_degrees(),self.z)
    }
}