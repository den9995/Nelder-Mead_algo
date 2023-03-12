fn main() {
    //println!("Hello, world!");
	let f = ProvidedFunc { x: [0.1,0.2].to_vec(), s: "a"};
	println!("val {}",f.compute());
}
pub struct ProvidedFunc<'a> {
	x: Vec<f64>,
	s: &'a str ,
}
impl ProvidedFunc<'_> {
	pub fn compute(&self) -> f64{
		return self.x.iter().sum();
	}
}

