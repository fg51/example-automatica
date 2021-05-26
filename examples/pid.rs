use automatica::{poly, Tf};
use automatica::{signals::continuous, Seconds, Ss};

use example_automatica as lib;

use lib::{feedback_loop, pid_controller::PID};

// G = \frac {21.81}{21.81 + 6.25 s + 1 * s^2}
// in = 0.1 (step)
pub fn main() {
    let plant = Tf::new(poly!(4.362), poly!(0., 6.25, 1.));
    println!("plant = {}", plant);

    let pid = PID::default().set_kp(5.);
    let cp = pid.tf() * &plant;
    println!("cp\n{}\n", cp); // [21.810] / [0. + 6.25 s + 1 s^2]

    let system = feedback_loop(&cp);
    println!("system\n{}\n", system); // [21.810] / [21.810. + 6.25 s + 1 s^2]

    let plant = Ss::new_observability_realization(&system).unwrap();
    //println!("ss\n{}\n", plant);

    let (step_value, size) = (0.1, 1);
    let step_input = continuous::step(step_value, size);

    let u0 = [0., 0.];
    let dt = Seconds(0.01);
    let num_of_step = 200;

    //let rk4 = plant.rk4(&step_input, &u0, dt, num_of_step);
    //println!("rk4 stationary values:");
    //let last_rk4 = rk4.last().unwrap();
    //print_step(&last_rk4);

    if true {
        println!("time,output");
        for i in plant.rk4(&step_input, &u0, dt, num_of_step) {
            print_csv(&i);
        }
    }
}

#[allow(dead_code)]
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
    println!("{},{}", s.time(), s.output()[0],);
}
