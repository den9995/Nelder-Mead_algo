use std::ops;

#[derive( Clone, Debug)]
pub struct Point {
   v:Vec<f64>
}
impl Point {
   pub fn new(value:Vec<f64>) -> Point {
      return Point {v:value};
   }
   pub fn getValue(&self) -> Vec<f64> {
      return self.v.clone();
   }
   pub fn getSize(&self) -> usize{
      return self.v.len();
   }
}
impl<'a> ops::Add for &'a Point {
   type Output = Point;
   fn add(self, other: &'a Point) -> Point {
      let mut vec: Vec<f64> = self.v.clone();
      for i in 0..vec.len() {
         vec[i]+=other.v[i];
      }
      return Point {v: vec}
   }
}
impl<'a> ops::Sub for &'a Point {
   type Output = Point;
   fn sub(self, other: &'a Point) -> Point {
      let mut vec: Vec<f64> = self.v.clone();
      for i in 0..vec.len() {
         vec[i]-=other.v[i];
      }
      return Point {v: vec}
   }
}
impl<'a> ops::Mul for &'a Point {
   type Output = Point;
   fn mul(self, other: &'a Point) -> Point {
      let mut vec: Vec<f64> = self.v.clone();
      for i in 0..vec.len() {
         vec[i]*=other.v[i];
      }
      return Point {v: vec}
   }
}
