use automatica::Tf;

pub mod pid_controller;

// \frac{G}{1 + G} = \frac{a}{b}{1 + \frac{a}{b}} = \frac{a}{a + b}
pub fn feedback_loop(system: &Tf<f64>) -> Tf<f64> {
    Tf::new(system.num().clone(), system.num() + system.den())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
