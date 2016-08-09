use std;


struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

trait HasAera {
    fn aera() -> f64;
}

trait Printable {
    fn print_aera();
}

impl HasAera for Circle {
    fn aera(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Printable for Circle {
    pub fn print_aera(&self) {
        println!("self.aera: {}", self.aera());
    }
}

pub print(obj:Printable) {
    obj.print_aera()
}
