fn f(x: f64) -> f64 {
    x.powi(2)
}

fn f_deriv(x: f64) -> f64 {
    2.0 * x
}

fn main() {
    let mut x = 1.5;
    for _ in 0..10 {
        let y = f(x);
        println!("f({x}) = {y}");

        x -= y / f_deriv(x); // Adjust x
    }
}
