use random::Source;
use std::cmp::Ordering;

fn main() {
   let string =  "x1 + x2";
   let func = ProvidedFunc { s: string };
   func.nelder_mid(1.,1.,1.);
   //let mut vt = vec![0.0;func.count_x()];
   //std::mem::swap(&mut vec[0], &mut vt);
   //std::mem::swap(&mut vec[1], &mut vt);
   //std::mem::swap(&mut vec[0], &mut vt);
   //println!("vec {:#?}",vec.iter().map(|row| func.compute(row)));
   //println!("vec {:#?}",vec);
   
}
//pub struct xval {
//   x: Vec<f64>,
//}
pub struct ProvidedFunc<'a> {
   s: &'a str ,
}
impl ProvidedFunc<'_> {
   pub fn compute(&self, x: &Vec<f64>) -> f64 {
      return (x[0]-1.0).powf(2.0)+(x[1]+1.0).powf(2.0);
   }
   pub fn count_x(&self) -> usize {
      return 2;
   }
   fn nelder_mid(&self, a: f64, b: f64, g: f64) {
   let mut vec = vec![vec![0.1; self.count_x()]; self.count_x()+1];
   let mut rng = random::default(0);
   vec = vec.iter().map(|row| row.iter().map(|_| rng.read::<f64>()).collect()).collect();
   vec.sort_by(|a, b| self.compute(a).partial_cmp(&self.compute(b)).unwrap_or(Ordering::Equal));
   println!("vec {:#?}, {} {} {}",vec.iter().map(|row| self.compute(row)).collect::<Vec<f64>>(),a,b,g);
   }
}
#[cfg(test)]
mod tests {
   // Note this useful idiom: importing names from outer (for mod tests) scope.
   use super::*;
   #[test]
   fn test_add_squares_func() {
      let func = ProvidedFunc { s:"x1 2 pow x2 2 pow +" };
      assert_eq!(func.compute(&[1.0, 2.0].to_vec()), 5.0);
   }
}

