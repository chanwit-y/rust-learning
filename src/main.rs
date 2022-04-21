fn main() {
    println!("{}", calculate_weight_on_mars(2.0));
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 0.81) * 50.0
}
