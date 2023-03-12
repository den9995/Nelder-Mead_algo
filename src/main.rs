fn main() {
    //println!("Hello, world!");
	let f = ProvidedFunc { s: "x1 + x2"};
	println!("val {}",f.compute([0.1,0.2].to_vec()));
}
pub struct ProvidedFunc<'a> {
	s: &'a str ,
}
impl ProvidedFunc<'_> {
	pub fn compute(&self, x: Vec<f64>) -> f64{
		return x.iter().sum();
	}
}
#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;
	fn test_add_func() {
		let func = ProvidedFunc { s:"x1 + x2" };
		assert_eq!(func.compute([1.0, 2.0].to_vec()), 3.0);
	}
}

