use random::Source;
use std::cmp::max;
use std::time::{SystemTime};
use std::env;

pub mod point;
use crate::point::Point;
pub mod simplex;
use crate::simplex::Simplex;
pub mod providedFunc;
use crate::providedFunc::ProvidedFunc;

fn main() {
   let func = ProvidedFunc::new("( x1 - 2 ) ^ 2 + ( 9 + x2 ) ^ 2 + ( x3 - 1.4 )^2");
   let res = nelder_mead(func.clone(),1.0,0.5,2.0,0.001,1000);
   println!("res {:?}, value {}",res,func.compute(&res)); 

}
pub fn nelder_mead(func:ProvidedFunc,alpha:f64, betta:f64,gamma:f64,eps:f64,max_steps:usize) -> Point {
   let mut stepsn = max_steps;
   //let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("REASON").as_nanos().try_into().unwrap();
   let start =  1679680043162523086;
   let mut rng = random::default(start);
   println!("rng seed {}, ", start);
   let mut pts : Vec<Point> = Vec::new();
   for i in 0..(func.count_x()+1) {
      pts.push(Point::new((0..func.count_x()).map(|_| (rng.read::<f64>()-0.5)*20.0).collect()));
   }
   let mut simplex = Simplex::new(pts,func,alpha,betta,gamma);
   println!("random values {:#?}",simplex);
   for n in 0..max_steps{
      simplex.sort();
      let diff = simplex.getDiff();
      if diff < eps{
         stepsn=n;
         break;
      }
      let xc = simplex.getCentroid();
      simplex.reflect(&xc);
      println!("best {:#?}, diff {}",simplex.getBest(),simplex.getDiff());
   }
   return simplex.getBest();
}
