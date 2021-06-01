use automatica::units::RadiansPerSecond;
use automatica::{signals::continuous, Seconds, Ss};

//use example_automatica as lib;

pub fn main() {
    let system = create_system();

    let inns = sin_input();
    let x0 = [1., 1.];
    let dt = Seconds(0.01);
    let num_of_step = 1000;

    let zero_input = continuous::step(0., 1);
    let mut y1 = system.rk4(&zero_input, &x0, dt, num_of_step);
    let mut y2 = system.rk4(&inns, &[0., 0.], dt, num_of_step);
    let y3 = system.rk4(&inns, &x0, dt, num_of_step);

    println!("time,y1,y2,y1+y3");
    for i in y3 {
        let y1 = y1.next().unwrap();
        let y2 = y2.next().unwrap();
        println!(
            "{},{},{},{}",
            i.time(),
            y1.output()[0],
            y2.output()[0],
            i.output()[0],
        );
    }
}

fn sin_input() -> impl Fn(Seconds<f64>) -> Vec<f64> {
    let (amp, omega, phase) = (2., RadiansPerSecond(1.), 0.);
    continuous::sin_siso(amp, omega, phase)
}

fn create_system() -> Ss<f64> {
    let (a, b) = (6.250, 4.362);
    let kp = 1.;
    let (number_of_states, number_of_inputs, number_of_outputs) = (2, 1, 1);
    Ss::new_from_slice(
        number_of_states,
        number_of_inputs,
        number_of_outputs,
        &[0., 1., -b * kp, -a],
        &[0., 1.],
        &[b * kp, 0.],
        &[0.],
    )
}
