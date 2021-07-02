struct CartCoord {
	x: f32,
	y: f32,
	z: f32,
}

impl From<SphereCoord> for CartCoord{
	fn from(item: SphereCoord) -> Self {
		CartCoord {
			x:item.r*item.phi.to_radians().cos()*item.theta.to_radians().sin(),
			y:item.r*item.phi.to_radians().sin()*item.theta.to_radians().sin(),
			z:item.r*item.theta.to_radians().cos()
		}
	}
}

impl From<CylCoord> for CartCoord {
    fn from(item: CylCoord) -> Self {
        CartCoord {
            x:,
            y:,
            z:
        }
    }
}

impl Add for CartCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x:self.x+other.x,
            y:self.y+other.y,
            z:self.z+other.z
        }
    }
}