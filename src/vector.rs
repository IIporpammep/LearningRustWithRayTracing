use std::ops;

use rand::{distributions::Uniform, prelude::Distribution};

#[derive(Copy, Clone, Default)]
pub struct Vector {
    pub data: [f32; 3],
}

pub fn dot(vec1: &Vector, vec2: &Vector) -> f32 {
    vec1.data[0] * vec2.data[0] + vec1.data[1] * vec2.data[1] + vec1.data[2] * vec2.data[2]
}

pub fn cross(vec1: &Vector, vec2: &Vector) -> Vector {
    let data: [f32; 3] = [
        vec1.data[1] * vec2.data[2] - vec1.data[2] * vec2.data[1],
        -(vec1.data[0] * vec2.data[2] - vec1.data[2] * vec2.data[0]),
        vec1.data[0] * vec2.data[1] - vec1.data[1] * vec2.data[0],
    ];
    return Vector { data };
}

pub fn lerp(start: &Vector, end: &Vector, t: f32) -> Vector {
    (*start) + t * ((*end) - (*start))
}

pub fn random_on_unit_sphere() -> Vector {
    let mut random = rand::thread_rng();
    let chance = Uniform::<f32>::from(-1.0..1.0);

    loop {
        let data = [
            chance.sample(&mut random),
            chance.sample(&mut random),
            chance.sample(&mut random),
        ];

        let random_vector = Vector { data: data };

        if random_vector.squared_length() >= 1.0 {
            continue;
        }

        return random_vector.normalize();
    }
}

impl Vector {
    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn r(&self) -> f32 {
        self.data[0]
    }

    pub fn g(&self) -> f32 {
        self.data[1]
    }

    pub fn b(&self) -> f32 {
        self.data[2]
    }

    pub fn length(&self) -> f32 {
        ((self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2])
            as f32)
            .sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        (self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2])
            as f32
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();

        let data: [f32; 3] = [
            self.data[0] / length,
            self.data[1] / length,
            self.data[2] / length,
        ];
        return Self { data };
    }

    pub fn is_near_zero(&self) -> bool {
        let epsilon: f32 = 1e-8;
        self.data[0].abs() < epsilon && self.data[1].abs() < epsilon && self.data[2].abs() < epsilon
    }

    pub fn reflect(&self, normal : &Vector) -> Vector
    {
        *self - 2.0 * dot(self, normal) * (*normal)
    }

    pub fn refract(&self, normal : &Vector, refraction_ratio : f32) -> Vector
    {
      let cos_theta = dot(&-(*self), normal).min(1.0);
      let r_out_perp = refraction_ratio * ((*self) + cos_theta * (*normal));
      let r_out_parallel = - (1.0 - r_out_perp.squared_length()).abs().sqrt() * (*normal);
      r_out_perp + r_out_parallel
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let data: [f32; 3] = [
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1],
            self.data[2] + rhs.data[2],
        ];
        return Self { data };
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        let data: [f32; 3] = [
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1],
            self.data[2] + rhs.data[2],
        ];

        *self = Vector { data: data }
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let data: [f32; 3] = [
            self.data[0] - rhs.data[0],
            self.data[1] - rhs.data[1],
            self.data[2] - rhs.data[2],
        ];
        return Self { data };
    }
}

impl ops::Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data: [f32; 3] = [
            self.data[0] * rhs.data[0],
            self.data[1] * rhs.data[1],
            self.data[2] * rhs.data[2],
        ];
        return Self { data };
    }
}

impl ops::Div for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let data: [f32; 3] = [
            self.data[0] / rhs.data[0],
            self.data[1] / rhs.data[1],
            self.data[2] / rhs.data[2],
        ];
        return Self { data };
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        let data: [f32; 3] = [-self.data[0], -self.data[1], -self.data[2]];
        return Self { data };
    }
}

impl ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let data: [f32; 3] = [self * rhs.data[0], self * rhs.data[1], self * rhs.data[2]];
        return Self::Output { data };
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        let data: [f32; 3] = [self.data[0] / rhs, self.data[1] / rhs, self.data[2] / rhs];
        return Self::Output { data };
    }
}
