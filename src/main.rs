use random::Source;
use std::cmp::Ordering;

fn main() {
   let string =  "x1 + x2";
   let func = ProvidedFunc { s: "x1 2 pow x2 2 pow +"};
   func.nelder_mid(1.0,0.5,2.0);
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
      return (x[0]-0.5).powf(2.0)+(x[1]+0.5).powf(2.0);
   }
   pub fn count_x(&self) -> usize {
      return 2;
   }
   pub fn nelder_mid(&self, a: f64, b: f64, g: f64) {
      let mut vec = vec![vec![0.0; self.count_x()]; self.count_x()+1];
      let mut rng = random::default(0);
      vec = vec.iter().map(|row| row.iter().map(|_| rng.read::<f64>()).collect()).collect();
      for n in 0..1 {
         vec.sort_by(|a, b| self.compute(a).partial_cmp(&self.compute(b)).unwrap_or(Ordering::Equal));
         let mut xc = vec![0.0; self.count_x()];
         for v in &vec{
            for (i, x) in v.into_iter().enumerate() {
               xc[i] += x/(self.count_x()+1) as f64;
            }
         }
         println!("xc {:#?}",xc);
         println!("vec {:#?}, values {:#?}, {} {} {}",vec,vec.iter().map(|row| self.compute(row)).collect::<Vec<f64>>(),a,b,g);
         self.shrink(&mut vec, &xc,b);
         println!("vec {:#?}, values {:#?}, {} {} {}",vec,vec.iter().map(|row| self.compute(row)).collect::<Vec<f64>>(),a,b,g);
      }
   }
//   fn reflect(&self, xw: &Vec<f64>, xc: &Vec<f64>, a:f64) -> Vec<f64> {
//      let mut xr = vec![0.0; self.count_x()];
//      for i in 0..self.count_x() {
//         xr[i] = (1.0+a)*xc[i]+a*xw[i];
//      }
//      return xr;
//   }
   fn shrink(&self, vec: &mut Vec<Vec<f64>>, xc: &Vec<f64>, b:f64)  {
      let mut xs = vec![0.0; self.count_x()];
      for i in 0..self.count_x() {
         xs[i] = (1.0-b)*xc[i]+b*vec[vec.len()-1][i];
      }
      let worst = vec.len()-1; 
      if self.compute(&xs) <= self.compute(&vec[worst]) {
         for i in 0..self.count_x() {
            vec[worst][i]=xs[i];
         }
      } else {
         for i in 1..self.count_x()+1 {
            for j in 0..self.count_x() {
               vec[i][j]=(vec[i][j] + vec[0][j])/2.0 ;
            }
         }
      }
   }
}
#[cfg(test)]
mod tests {
   // Note this useful idiom: importing names from outer (for mod tests) scope.
   use super::*;
   #[test]
   fn test_add_squares_func() {
      let func = ProvidedFunc { s:"x1 2 pow x2 2 pow +" };
      assert_eq!(func.compute(&[1.0, 2.0].to_vec()), 6.5);
   }
   #[test]
   fn test_shrink() {
      let func = ProvidedFunc { s:"x1 2 pow x2 2 pow +" };
      let mut vec = [ [1.0, 1.0].to_vec(), [1.0, 3.0].to_vec(), [3.0, 3.0].to_vec()].to_vec();
      let mut xc = [5.0/3.0, 7.0/3.0].to_vec();
      func.shrink(&mut vec, &xc,0.5);
      assert_eq!(vec[vec.len()-1],[2.3333333333333335, 2.666666666666667].to_vec());
      xc = [5.0, 7.0].to_vec();
      func.shrink(&mut vec, &xc,0.5);
      assert_eq!(vec[vec.len()-2],[1.0, 2.0].to_vec());
      //assert_eq!(func.reflect(&xw,&xc,1.0),[3.0, 3.0].to_vec());
   }
}

