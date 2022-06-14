use std::ops;

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
