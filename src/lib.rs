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
    macro_rules! v1 {
        () => {
            Vector{x:1.0, y: 2.0, z: 1.0}
        };
    }

    macro_rules! v2 {
        () => {
            Vector{x:3.0, y: 3.0, z: 2.0}
        };
    }
    #[test]
    fn init() {
        let result = Vector::new();
        assert_eq!(result.x, 0.0, "Assert x");
        assert_eq!(result.y, 0.0, "Assert y");
        assert_eq!(result.z, 0.0, "Assert z");
    }

    #[test]
    fn update_partial(){
        let mut vec = Vector::new();
        vec.x = 3.0;
        vec.y = 1.0;
        vec.z = 2.0;
        assert_eq!(vec.x, 3.0, "changed x to 3.0");
        assert_eq!(vec.y, 1.0, "changed y to 1.0");
        assert_eq!(vec.z, 2.0, "changed z to 2.0");
    }


    #[test]
    fn eq(){
        let v1 = Vector::new();
        let v2 = Vector::new();
        assert!(v1 == v2, "Testing Partial equality");
    }

    #[test]
    fn ne(){

        let v1 = Vector::new();
        let v2 = Vector::new();
        assert!(v1 != v2, "Testing Partial non equality");
    }

    #[test]
    fn clone(){
        let v1 = Vector{x:1.0, y:2.0, z: 3.0};
        let v2 = v1.clone();

        assert!(&v1 != &v2, "testing clone")

    }

    #[test]
    fn add(){
        let vec = Vector{x: 1.1, y: 2.3, z:1.0 };
        let add = Vector{x: 1.0, y: 2.0, z: 3.0};
        let res = vec + add;
        assert!(res == Vector{x: 2.1, y:4.3, z:4.0});   
    }

    #[test]
    fn add_assign(){
        let mut v1 = v1!();
        let v2 = v2!();
        v1 += v2;
        let res = Vector{x: 4.0, y: 5.0, z: 2.0};

        assert!(v1 == res, "add assign test");

    }

    #[test]
    fn sub(){

        let v1 = v1!();
        let v2 = v2!();
        let res = v1 - v2 ;
        assert!(res == Vector{x: -2.0, y:-1.0, z: -1.0});   
    }

    #[test]
    fn sub_assign(){
        let mut v1 = v1!();
        let v2 = v2!();
        v1 -= v2;
        assert!(v1 == Vector{x: -2.0, y: -1.0, z: -1.0});
    }

    #[test]
    fn mul(){
        let v1 = v1!();
        let v2 = v2!();
        let res = v1*v2;
        assert!(res == Vector{x:3.0, y: 6.0, z: 2.0  });
    }

    #[test]
    fn mul_assign(){
        let mut v1 = v1!();
        let v2 = v2!();
        v1 *= v2;
        assert!(v1 == Vector{x: 3.0, y: 6.0, z:2.0});
    }

    #[test]
    fn div(){
        let v1 = Vector{x: 4.0, y:6.0, z:10.0};
        let v2 = Vector{x: 2.0, y:3.0, z: 2.0};

        let res = v1/v2;

        assert!(res == Vector{x: 2.0, y: 2.0, z: 5.0}, "Proper Div");
    }

    #[test]
    #[should_panic]
    fn div_zero(){
        let v1 = v1!();
        let v2 = Vector{x:1.0, y:0.0, z:0.0};
        let _res = v1/v2;

    }

    #[test]
    fn div_assign(){

        let mut v1 = Vector{x: 4.0, y:6.0, z:10.0};
        let v2 = Vector{x: 2.0, y:3.0, z: 2.0};

        v1 /= v2;

        assert!(v1 == Vector{x: 2.0, y: 2.0, z: 5.0}, "Proper Div Assign");
    }
    
    #[test]
    #[should_panic]
    fn div_assign_zero(){
        
        let mut v1 = Vector{x: 4.0, y:6.0, z:10.0};
        let v2 = Vector{x: 2.0, y:0.0, z: 2.0};

        v1 /= v2;

    }
}
