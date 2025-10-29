use std::{fmt::{Error, Result}, ops::{Add, AddAssign, Sub}};

pub struct Vector{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub trait MathOps<T> {
    fn add(&mut self, b:T) -> Result;
    fn subtract(&mut self, b:T) -> Result;
    fn multiply(&mut self, b:T) -> Result;
    fn divide(&mut self, b:T) -> Result;
}
impl MathOps<Vector> for Vector{

    fn add(&mut self, b: Vector) -> Result{
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
        Ok(())
    }
    fn subtract(&mut self, b: Vector) -> Result{
        self.x -= b.x;
        self.y -= b.y;
        self.z -= b.z;
        Ok(())
    }
    fn multiply(&mut self, b:Vector) -> Result{
        
        self.x *= b.x;
        self.y *= b.y;
        self.z *= b.z;
        Ok(())
    }
    fn divide(&mut self, b:Vector) -> Result{
        if b.x != 0.0 && b.y != 0.0 && b.z != 0.0{
            self.x /= b.x;
            self.y /= b.y;
            self.z /= b.z;
            Ok(())
        }else{
            Err(Error)
        }
    }
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
