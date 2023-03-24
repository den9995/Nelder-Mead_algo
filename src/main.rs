use random::Source;
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};


fn main() {
   let string =  "x1 + x2";
   let func = ProvidedFunc { s: "x1 2 pow x2 2 pow +"};
   func.nelder_mid(1.0,0.5,2.0);
   
}
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
      let maxSteps = 1000000;
      let mut stepsn = maxSteps;
      let mut vec = vec![vec![0.0; self.count_x()]; self.count_x()+1];
      let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("REASON").as_nanos().try_into().unwrap();
      //let start = 15;
      let mut rng = random::default(start);
      println!("rng seed {}, ", start);
      for j in 0..(self.count_x()) {
         vec[0][j]=(rng.read::<f64>()-0.5)*2.0;
      }
      for i in 1..(self.count_x()+1) {
         for j in 0..(self.count_x()) {
            vec[i][j]=vec[i-1][j]+(rng.read::<f64>()-0.5)*2.0;
         }
      }
      //println!("vec {:#?}, ", vec);
      //println!("values {:#?}",vec.iter().map(|row| self.compute(row)).collect::<Vec<f64>>());
      for n in 0..maxSteps{
         vec.sort_by(|a, b| self.compute(a).partial_cmp(&self.compute(b)).unwrap_or(Ordering::Equal));
         let diff = self.compute(&vec[self.count_x()])-self.compute(&vec[0]);
         if diff < 0.0000001 {
            stepsn=n;
            break;
         }
         let mut xc = vec![0.0; self.count_x()];
         for v in (&vec).into_iter().take(self.count_x()) {
            for (i, x) in v.into_iter().enumerate() {
               xc[i] += x/(self.count_x()) as f64;
            }
         }
         self.reflect(&mut vec, &xc,a,b,g);
      }
      
      vec.sort_by(|a, b| self.compute(a).partial_cmp(&self.compute(b)).unwrap_or(Ordering::Equal));
      println!("vec {:#?}, {}", vec, stepsn);
      println!("values {:#?}",vec.iter().map(|row| self.compute(row)).collect::<Vec<f64>>());
   }
   fn reflect(&self, vec: &mut Vec<Vec<f64>>, xc: &Vec<f64>, a:f64, b:f64, g:f64)  {
      let mut xr = vec![0.0; self.count_x()];
      for i in 0..self.count_x() {
         xr[i] = (1.0+a)*xc[i]+a*vec[vec.len()-1][i];
      }
      let worst = vec.len()-1;
      let good =  vec.len()-2;
      if self.compute(&xr) <= self.compute(&vec[0]) {
         self.expand(vec, &xc, &xr,g);
      } else if self.compute(&xr) <= self.compute(&vec[good]) {
         for i in 0..self.count_x() {
            vec[worst][i]=xr[i];
         }
         self.shrink(vec, &xc,b);
      } else if self.compute(&xr) <= self.compute(&vec[worst]) {
         for i in 0..self.count_x() {
            vec[worst][i]=xr[i];
         }
         self.shrink(vec, &xc,b);
      } else { 
         self.shrink(vec, &xc,b);
      }
   }
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
   fn expand(&self, vec: &mut Vec<Vec<f64>>, xc: &Vec<f64>, xr: &Vec<f64>, g:f64) {
      let mut xe = vec![0.0; self.count_x()];
      for i in 0..self.count_x() {
         xe[i] = (1.0-g)*xc[i]+g*xr[i];
      }
      let worst = vec.len()-1;
      if self.compute(&xe) <= self.compute(&xr) {
         for i in 0..self.count_x() {
            vec[worst][i] = xe[i];
         }
      } else {
         for i in 0..self.count_x() {
            vec[worst][i] = xr[i];
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
   }
   #[test]
   fn test_expand() {
      let func = ProvidedFunc { s:"x1 2 pow x2 2 pow +" };
      let mut vec = [ [1.0, 1.0].to_vec(), [1.0, 3.0].to_vec(), [3.0, 3.0].to_vec()].to_vec();
      let xc = [5.0/3.0, 7.0/3.0].to_vec();
      let mut xr = [1.0, 1.0].to_vec();
      func.expand(&mut vec, &xc , &xr,2.0);
      assert_eq!(vec[vec.len()-1],[0.33333333333333326, -0.3333333333333335].to_vec());
      xr = [5.0, 7.0].to_vec();
      func.expand(&mut vec, &xc, &xr ,2.0);
      assert_eq!(vec[vec.len()-1],[5.0, 7.0].to_vec());
   }
}

