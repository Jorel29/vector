use std::fmt::{Result};

pub struct Vector{
    x: f64,
    y: f64,
    z: f64,
}



impl Vector {
    
    pub fn new() -> Vector {
        Vector { x: 0.0 , y: 0.0, z: 0.0 }
    }

    pub fn add(&mut self, vec: Vector) -> Result{
        self.x += vec.x;
        self.y += vec.y;
        self.z += vec.z;
        Ok(())
    }

    pub fn subtract(&mut self, vec: Vector) -> Result{
        self.x -= vec.x;
        self.y -= vec.y;
        self.z -= vec.z;
        Ok(())
    }

    pub fn divide(&mut self, vec: Vector) -> Result{
        if vec.x != 0.0 && vec.y != 0.0 && vec.z != 0.0{
            self.x /= vec.x;
            self.y /= vec.y;
            self.z /= vec.z;
            return Ok(())
        }else{
            return Err(std::fmt::Error)
        }
    }

    pub fn multiply(&mut self, vec:Vector) -> Result {
        self.x *= vec.x;
        self.y *= vec.y;
        self.z *= vec.z;
        Ok(())
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
        let mut vec = Vector::new();
        vec.x = 3.0;

        assert_eq!(vec.x, 3.0, "changed x to 3.0");
    }

}
