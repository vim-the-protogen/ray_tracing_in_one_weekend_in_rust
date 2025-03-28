use std::io::{self, Write};
use std::ops;
use std::fmt;

pub mod vec3 {
    #[derive(Copy,Clone, Debug)]
    pub struct Vec3 {
        e: [f64; 3],
    }

    impl Vec3 {

        pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
            Self { e: [e0, e1, e2] }
        }

        /// get the z coord
        pub fn x(&self) -> f64 {
            self.e[0]
        }

        /// get the y coord
        pub fn y(&self) -> f64 {
            self.e[1]
        }

        /// get the Z coord
        pub fn z(&self) -> f64 {
            self.e[2]
        }

        pub fn length_squared(&self) -> f64 {
            self.e[0] * self.e[0] + 
            self.e[1] * self.e[1] + 
            self.e[2] * self.e[2]
        }

        pub fn length(&self) -> f64 {
            self.length_squared().sqrt()
        }
    }

    impl Default for Vec3 {
        fn default() -> Self {
            Self { e: [0.0, 0.0, 0.0] }
        }
    }

    // Add type operators
    //
    
    /// Add two Vec3
    impl std::ops::Add for Vec3 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Self {
                e: [
                    self.e[0] + rhs.e[0],
                    self.e[1] + rhs.e[1],
                    self.e[2] + rhs.e[2]
                ]
            }
        }
    }

    /// Add Vec3 and a f64
    impl std::ops::Add<f64> for Vec3 {
        type Output = Self;

        fn add(self, rhs: f64) -> Self {
            Self {
                e: [
                    self.e[0] + rhs, 
                    self.e[1] + rhs,
                    self.e[2] + rhs
                ]
            }
        }
    }

    /// AddAssign Vec3s
    impl std::ops::AddAssign for Vec3 {
        fn add_assign(&mut self, rhs: Self) {
            *self = Self {
                e: [
                    self.e[0] + rhs.e[0],
                    self.e[1] + rhs.e[1],
                    self.e[2] + rhs.e[2]
                ]
            }
        }
    }

    // Sub opreations
    

    impl std::ops::Sub for Vec3 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self{
            Self {
                e: [
                    self.e[0] - rhs.e[0],
                    self.e[1] - rhs.e[1],
                    self.e[2] - rhs.e[2]
                ]
            }
        } 
    }

    impl std::ops::Sub<f64> for Vec3 {
        type Output = Self;

        fn sub(self, rhs: f64) -> Self {
            Self {
                e: [
                    self.e[0] - rhs, 
                    self.e[1] - rhs,
                    self.e[2] - rhs
                ]
            }
        }
    }

    /// AddAssign Vec3s
    impl std::ops::SubAssign for Vec3 {
        fn sub_assign(&mut self, rhs: Self) {
            *self = Self {
                e: [
                    self.e[0] - rhs.e[0],
                    self.e[1] - rhs.e[1],
                    self.e[2] - rhs.e[2]
                ]
            }
        }
    }

    impl std::ops::Mul for Vec3 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self{
            Self {
                e: [
                    self.e[0] * rhs.e[0],
                    self.e[1] * rhs.e[1],
                    self.e[2] * rhs.e[2]
                ]
            }
        } 
    }

    impl std::ops::Mul<f64> for Vec3 {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self {
            Self {
                e: [
                    self.e[0] * rhs, 
                    self.e[1] * rhs,
                    self.e[2] * rhs
                ]
            }
        }
    }

    /// AddAssign Vec3s
    impl std::ops::MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, rhs: f64) {
            *self = Self {
                e: [
                    self.e[0] * rhs,
                    self.e[1] * rhs,
                    self.e[2] * rhs
                ]
            }
        }
    }

    // Div ops

    /// div two Vec3
    impl std::ops::Div for Vec3 {
        type Output = Self;

        fn div(self, rhs: Self) -> Self {
            Self {
                e: [
                    self.e[0] / rhs.e[0],
                    self.e[1] / rhs.e[1],
                    self.e[2] / rhs.e[2]
                ]
            }
        }
    }

    /// Div Vec3 and a f64
    impl std::ops::Div<f64> for Vec3 {
        type Output = Self;

        fn div(self, rhs: f64) -> Self {
            Self {
                e: [
                    self.e[0] / rhs, 
                    self.e[1] / rhs,
                    self.e[2] / rhs
                ]
            }
        }
    }

    /// DivAssign Vec3s
    impl std::ops::DivAssign<f64> for Vec3 {
        fn div_assign(&mut self, rhs: f64) {
            *self *= rhs
        }
    }

    // matrix operations
    /// Dot product of two vectors. Yields a scalar
    pub fn dot(rhs: Vec3, lhs: Vec3) -> f64 {
        lhs.e[0] * rhs.e[0] +
        lhs.e[1] * rhs.e[1] +
        lhs.e[2] * rhs.e[2]
    }

    /// cross product of two vectors, yields a vector
    pub fn cross(rhs: Vec3, lhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                rhs.e[1] * lhs.e[2] - rhs.e[2] * rhs.e[1],
                rhs.e[2] * lhs.e[0] - rhs.e[0] * rhs.e[2],
                rhs.e[0] * lhs.e[1] - rhs.e[1] * rhs.e[0]
            ]
        }
    }

    /// calculatse the unit vector
    pub fn unit_vector(vec: Vec3) -> Vec3 {
        let len = vec.length();
        vec / len
    }
}
