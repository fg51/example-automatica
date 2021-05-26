use automatica::{poly, Tf};
use automatica::{signals::continuous, Seconds, Ss};

// G = \frac {43.62}{43.62 + 6.25 s + 1 * s^2}
// in = 0.1 (step)
pub fn main() {
    let tf = Tf::new(poly!(43.62), poly!(43.62, 6.25, 1.));
    let plant = Ss::new_observability_realization(&tf).unwrap();

    let (step_value, size) = (0.1, 1);
    let step_input = continuous::step(step_value, size);

    let u0 = [0., 0.];
    let rk4 = plant.rk4(&step_input, &u0, Seconds(0.01), 200);

    println!("rk4 stationary values:");
    let last_rk4 = rk4.last().unwrap();
    print_step(&last_rk4);

    if true {
        println!("time,state[0],state[1],output");
        for i in plant.rk4(&step_input, &[0., 0.], Seconds(0.01), 200) {
            print_csv(&i);
        }
    }
}

fn print_step(s: &automatica::linear_system::solver::Step<f64>) {
    println!(
        "Time: {:.2}; state: [{:.4}; {:.4}]; output: [{:.4}; ]",
        s.time(),
        s.state()[0],
        s.state()[1],
        s.output()[0],
    );
}

fn print_csv(s: &automatica::linear_system::solver::Step<f64>) {
    println!(
        "{},{:.4},{:.4},{}",
        s.time(),
        s.state()[0],
        s.state()[1],
        s.output()[0],
    );
}
