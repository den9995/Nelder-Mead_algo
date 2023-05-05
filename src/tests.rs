#[cfg(test)]
mod tests {
   // Note this useful idiom: importing names from outer (for mod tests) scope.
   use super::*;
   #[test]
   fn test_add_squares_func() {
      let func = ProvidedFunc { s:"( x1 -0.5)^2+( x2 +0.5)^ 2" };
      assert_eq!(func.compute(&[1.0, 2.0].to_vec()), 6.5);
   }
   #[test]
   fn test_shrink() {
      let func = ProvidedFunc { s:"( x1 -0.5)^2+( x2 +0.5)^ 2" };
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
      let func = ProvidedFunc { s:"( x1 -0.5)^2+( x2 +0.5)^ 2" };
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

