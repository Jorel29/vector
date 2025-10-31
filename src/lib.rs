use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}
};

pub struct Vector{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vector{
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self ){
        *self = Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul for Vector {
    type Output = Vector;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z, 
        }
    }
}

impl MulAssign for Vector {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self { 
            x: self.x * rhs.x,
            y: self.y * rhs.y, 
            z: self.z * rhs.z,
        }
    }
}

impl Div for Vector {
    type Output = Vector;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.x == 0.0 || rhs.y == 0.0 || rhs.z == 0.0{
            panic!("Cannot divide by zero x:{} y:{} z:{}", rhs.x, rhs.y, rhs.z);
        };

        Vector{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vector {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.x == 0.0 || rhs.y == 0.0 || rhs.z == 0.0{
            panic!("Cannot divide by zero x:{} y:{} z:{}", rhs.x, rhs.y, rhs.z);
        };
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }        

    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool { 
        self.x != other.x && self.y != other.y && self.z != other.z
    }
}



impl Clone for Vector{
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone(), z: self.z.clone() }
    }
}

impl Vector {
     
    pub fn new() -> Vector {
        Vector { x: 0.0 , y: 0.0, z: 0.0 }
    }

    pub fn equal(&self,  vec:Vector) -> bool{
        self.x == vec.x && self.y == vec.y && self.z == vec.z
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn init() {
        let result = Vector::new();
        assert_eq!(result.x, 0.0, "Assert x");
        assert_eq!(result.y, 0.0, "Assert y");
        assert_eq!(result.z, 0.0, "Assert z");
    }

    #[test]
    fn update_x(){
        let vec = Vector::new();

        assert_eq!(vec.x, 3.0, "changed x to 3.0");
    }

}
