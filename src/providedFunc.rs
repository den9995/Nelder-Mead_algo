use crate::point::Point;
use std::cmp::max;

#[derive( Clone, Debug)]
pub struct ProvidedFunc<'a> {
   s: &'a str ,
}
impl ProvidedFunc<'_> {
   pub fn new(value:&str) -> ProvidedFunc{
      return ProvidedFunc {s:value};
   }
   pub fn compute(&self, x: &Point) -> f64 {
      let val = x.getValue();
      let parts = self.s.split(' ').collect::<Vec<_>>();
      let mut result : String = Default::default();
      for p in &parts {
         if p.starts_with('x') {
            let num = (&p[1..]).parse::<usize>().unwrap();
            result.push_str("(");
            result.push_str(&val[num-1].to_string());
            result.push_str(")");
         } else {
            result.push_str(&p);
         }
      }
      return mexprp::eval::<f64>(result.as_str()).unwrap().unwrap_single();
   }
   pub fn count_x(&self) -> usize {
      let parts = self.s.split(' ').collect::<Vec<_>>();
      let mut count = 0;
      for p in &parts {
         if p.starts_with('x') {
            count = max(count, (&p[1..]).parse::<usize>().unwrap());
         }
      }
      return count;
   }
}
