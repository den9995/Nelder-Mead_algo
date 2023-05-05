use crate::point::Point;
use crate::providedFunc::ProvidedFunc;
use std::cmp::Ordering;

#[derive( Clone, Debug)]
pub struct Simplex<'a> {
   pointSize:usize,
   points:Vec<Point>,
   alpha:f64,
   betta:f64,
   gamma:f64,
   func:ProvidedFunc<'a>,
}
impl Simplex<'_> {
   pub fn new(value:Vec<Point>, function:ProvidedFunc, alpha:f64, betta:f64,gamma:f64) -> Simplex {
      let size = value[0].getSize();
      return Simplex {points:value, pointSize:size, func:function, alpha:alpha, betta:betta, gamma:gamma};
   }
   pub fn getCentroid(&self) -> Point{
      let mut p = self.points[0].clone();
      for i in 1..(self.func.count_x()) {
         p = &p + &self.points[i];
      }
      let divider = Point::new(vec![ 1.0 / self.func.count_x() as f64; self.func.count_x()]);
      p = &p * &divider;
      return p;
   }
   pub fn sort(&mut self) {
      self.points.sort_by(|a, b| self.func.compute(a).partial_cmp(&self.func.compute(b)).unwrap_or(Ordering::Equal));
   }
   pub fn getBest(&mut self) -> Point{
      self.sort();
      let mut p = self.points[0].clone();
      return p;
   }
   pub fn getDiff(&mut self) -> f64{
      self.sort();
      let mut p = self.func.compute(&self.points[self.points.len()-1]) - self.func.compute(&self.points[0]);
      return p;
   }

   
   pub fn expand(&mut self, xc:&Point, xr:&Point) {
      self.sort();
      let xe = &(&Point::new(vec![1.0-self.gamma;self.pointSize])*xc)+&(&Point::new(vec![self.gamma;self.pointSize])*xr);
      let worst_index = self.points.len()-1;
      let worst;
      if self.func.compute(&xe) <= self.func.compute(&xr) {
         worst = xe;
      } else {
         worst = (*xr).clone();
      }
      self.points[worst_index] = worst.clone();
   }
   pub fn shrink(&mut self, xc: &Point)  {
      self.sort();
      let worst_index = self.points.len()-1;
      let xs = &(&Point::new(vec![1.0-self.betta;self.pointSize])*xc)+&(&Point::new(vec![self.betta;self.pointSize])*&self.points[worst_index]);
      if self.func.compute(&xs) <= self.func.compute(&self.points[worst_index]) {
         self.points[worst_index] = xs.clone();
      } else {
         for i in 1..self.pointSize+1 {
            self.points[i] = &(&self.points[i].clone()+&self.points[0].clone())*&Point::new(vec![1.0/2.0;self.pointSize]);
         }
      }
   }
   pub fn reflect(&mut self, xc: &Point)  {
      self.sort();
      let worst_index = self.points.len()-1;
      let good_index  = self.points.len()-2;
      let best_index  = 0;
      let xr = (&Point::new(vec![1.0+self.alpha;self.pointSize]))+&(&Point::new(vec![self.alpha;self.pointSize])*&self.points[worst_index]);
      if self.func.compute(&xr) <= self.func.compute(&self.points[best_index]) {
         self.expand(&xc, &xr);
      } else if self.func.compute(&xr) <= self.func.compute(&self.points[good_index]) {
         self.points[worst_index]=xr.clone();
         self.shrink(&xc);
      } else if self.func.compute(&xr) <= self.func.compute(&self.points[worst_index]) {
         self.points[worst_index]=xr.clone();
         self.shrink(&xc);
      } else { 
         self.shrink(&xc);
      }
   }
}
