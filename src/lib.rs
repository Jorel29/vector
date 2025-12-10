use std::{
    f64, fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}
};

//TODO: consider tolerance versions of functions
#[derive(Clone, Copy, Debug)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self ){
        *self = Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z, 
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self { 
            x: self.x * rhs.x,
            y: self.y * rhs.y, 
            z: self.z * rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.x == 0.0 || rhs.y == 0.0 || rhs.z == 0.0{
            panic!("Cannot divide by zero x:{} y:{} z:{}", rhs.x, rhs.y, rhs.z);
        };

        Vec3{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vec3 {
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

impl Mul<f64> for Vec3{
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output{
        Vec3{x:self.x*rhs, y:self.y*rhs, z:self.z*rhs}
    }
}

impl MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self { 
            x: self.x*rhs,
            y: self.y*rhs,
            z: self.z*rhs,
        }
    }
}

impl Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero rhs: {}", rhs);
        };
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        
        if rhs == 0.0 {
            panic!("Cannot divide by zero rhs: {}", rhs);
        };

        *self = Self { 
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs 
        }
    }
}
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool { 
        self.x != other.x && self.y != other.y && self.z != other.z
    }
}


impl Display for Vec3{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X: {} Y: {} Z: {}", self.x, self.y, self.z)
    }
}

impl Vec3 {
     
    pub fn new() -> Vec3 {
        Vec3 { x: 0.0 , y: 0.0, z: 0.0 }
    }

    pub fn distance(a: &Vec3, b: &Vec3) -> f64 {
        let x = (b.x -a.x) * (b.x - a.x);
        let y = (b.y - a.y) * (b.y - a.y);
        let z = (b.z - a.z) * (b.z - a.z);

        (x + y + z).sqrt()                
    }

    pub fn distance_squared(a: &Vec3, b:&Vec3) -> f64{
        let x = (b.x -a.x) * (b.x - a.x);
        let y = (b.y - a.y) * (b.y - a.y);
        let z = (b.z - a.z) * (b.z - a.z);

        x + y + z
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f64{
        a.x * b.x + a.y * b.y + a.z * b.z
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    macro_rules! v1 {
        () => {
            Vec3{x:1.0, y: 2.0, z: 1.0}
        };
    }

    macro_rules! v2 {
        () => {
            Vec3{x:3.0, y: 3.0, z: 2.0}
        };
    }
    #[test]
    fn init() {
        let result = Vec3::new();
        assert_eq!(result.x, 0.0, "Assert x");
        assert_eq!(result.y, 0.0, "Assert y");
        assert_eq!(result.z, 0.0, "Assert z");
    }

    #[test]
    fn update_partial(){
        let mut vec = Vec3::new();
        vec.x = 3.0;
        vec.y = 1.0;
        vec.z = 2.0;
        assert_eq!(vec.x, 3.0, "changed x to 3.0");
        assert_eq!(vec.y, 1.0, "changed y to 1.0");
        assert_eq!(vec.z, 2.0, "changed z to 2.0");
    }


    #[test]
    fn eq(){
        let v1 = Vec3::new();
        let v2 = Vec3::new();
        assert!(v1 == v2, "Testing Partial equality");
    }

    #[test]
    fn ne(){

        let v1 = Vec3::new();
        let v2 = Vec3 { x: 1.0, y: 1.0, z:1.0 };
        assert!(v1 != v2, "Testing Partial non equality");
    }

    #[test]
    fn clone(){
        let v1 = Vec3{x:1.0, y:2.0, z: 3.0};
        let v2 = v1.clone();

        assert!(v1 == v2, "testing clone, makes deep copy, but same values");

    }

    #[test]
    fn add(){
        let vec = Vec3{x: 1.1, y: 2.3, z:1.0 };
        let add = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let res = vec + add;
        assert!(res == Vec3{x: 2.1, y:4.3, z:4.0});   
    }

    #[test]
    fn add_assign(){
        let mut v1 = v1!();
        let v2 = v2!();
        v1 += v2;
        let res = Vec3{x: 4.0, y: 5.0, z: 3.0};

        assert!(v1 == res, "add assign test");

    }

    #[test]
    fn sub(){

        let v1 = v1!();
        let v2 = v2!();
        let res = v1 - v2 ;
        assert!(res == Vec3{x: -2.0, y:-1.0, z: -1.0});   
    }

    #[test]
    fn sub_assign(){
        let mut v1 = v1!();
        let v2 = v2!();
        v1 -= v2;
        assert!(v1 == Vec3{x: -2.0, y: -1.0, z: -1.0});
    }

    #[test]
    fn mul(){
        let v1 = v1!();
        let v2 = v2!();
        let res = v1*v2;
        assert!(res == Vec3{x:3.0, y: 6.0, z: 2.0  });
    }

    #[test]
    fn mul_assign(){
        let mut v1 = v1!();
        let v2 = v2!();
        v1 *= v2;
        assert!(v1 == Vec3{x: 3.0, y: 6.0, z:2.0});
    }

    #[test]
    fn div(){
        let v1 = Vec3{x: 4.0, y:6.0, z:10.0};
        let v2 = Vec3{x: 2.0, y:3.0, z: 2.0};

        let res = v1/v2;

        assert!(res == Vec3{x: 2.0, y: 2.0, z: 5.0}, "Proper Div");
    }

    #[test]
    #[should_panic]
    fn div_zero(){
        let v1 = v1!();
        let v2 = Vec3{x:1.0, y:0.0, z:0.0};
        let _res = v1/v2;

    }

    #[test]
    fn div_assign(){

        let mut v1 = Vec3{x: 4.0, y:6.0, z:10.0};
        let v2 = Vec3{x: 2.0, y:3.0, z: 2.0};

        v1 /= v2;

        assert!(v1 == Vec3{x: 2.0, y: 2.0, z: 5.0}, "Proper Div Assign");
    }
    
    #[test]
    #[should_panic]
    fn div_assign_zero(){
        
        let mut v1 = Vec3{x: 4.0, y:6.0, z:10.0};
        let v2 = Vec3{x: 2.0, y:0.0, z: 2.0};

        v1 /= v2;

    }
    
    #[test]
    fn mul_element_wise(){
        let v1 = v1!();
        let x = 2.0;

        let res = v1 * x;

        assert!(res== Vec3{x: 2.0, y: 4.0, z: 2.0});
    }

    #[test]
    fn mul_assign_element_wise(){
        let mut v1 = v1!();
        let x = 2.0;

        v1 *= x;

        
        assert!(v1 == Vec3{x: 2.0, y: 4.0, z: 2.0});
    }

    #[test]
    fn div_element_wise(){
        let v1 = v1!();
        let x = 2.0;

        let res = v1 / x;

        assert!(res == Vec3{x: 0.5, y: 1.0, z: 0.5});
    }

    #[test]
    fn div_assign_element_wise(){
        let mut v1 = v1!();
        let x = 2.0;

        v1 /= x;

        assert!(v1 == Vec3{x: 0.5, y: 1.0, z: 0.5})
    }

    #[test]
    #[should_panic]
    fn div_element_wise_zero(){
        let v1 = v1!();
        let x = 0.0;

        let _res = v1 / x;
    }

    #[test]
    #[should_panic]
    fn div_assign_element_wise_zero(){
        let mut v1 = v1!();
        let x = 0.0;

        v1 /= x;

    }

    #[test]
    fn distance(){
        let v1 = v1!();
        let v2 = v2!();

        let dist = Vec3::distance(&v1, &v2);

        assert!(dist != 0.0, "Checking dist")
    }

    #[test]
    fn distance_squared(){

        let v1 = v1!();
        let v2 = v2!();

        let dist = Vec3::distance_squared(&v1, &v2);

        assert!(dist != 0.0, "Checking dist squared")
    }

    #[test]
    fn simple_dot(){
        let v1 = v1!();
        let v2 = v2!();

        let dot = Vec3::dot(&v1, &v2);
        assert_eq!(dot, 11.0, "Checking dot product")
    }
}
