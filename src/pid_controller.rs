use automatica::{poly, Tf};

pub fn kp(kp: f64) -> Tf<f64> {
    Tf::new(poly!(kp), poly!(1.))
}

pub fn ki(ki: f64) -> Tf<f64> {
    Tf::new(poly!(ki), poly!(0., 1.))
}

pub fn kd(kd: f64) -> Tf<f64> {
    Tf::new(poly!(0., kd), poly!(1.))
}

#[derive(Default)]
pub struct PID {
    kp: Option<f64>,
    ki: Option<f64>,
    kd: Option<f64>,
}

impl PID {
    pub fn set_kp(self, kp: f64) -> Self {
        Self {
            kp: Some(kp),
            ..self
        }
    }

    pub fn set_ki(self, ki: f64) -> Self {
        Self {
            ki: Some(ki),
            ..self
        }
    }

    pub fn set_kd(self, kd: f64) -> Self {
        Self {
            kd: Some(kd),
            ..self
        }
    }

    pub fn tf(&self) -> Tf<f64> {
        match self.kp_tf() {
            Some(kp) => match self.ki_tf() {
                Some(ki) => match self.kd_tf() {
                    Some(kd) => kp + ki + kd,
                    None => kp + ki,
                },
                None => match self.kd_tf() {
                    Some(kd) => kp + kd,
                    None => kp,
                },
            },
            None => match self.ki_tf() {
                Some(ki) => match self.kd_tf() {
                    Some(kd) => ki + kd,
                    None => ki,
                },
                None => match self.kd_tf() {
                    Some(kd) => kd,
                    None => unreachable!("kp, ki, kd are None."),
                },
            },
        }
    }

    fn kp_tf(&self) -> Option<Tf<f64>> {
        match self.kp {
            Some(v) => Some(kp(v)),
            None => None,
        }
    }

    fn ki_tf(&self) -> Option<Tf<f64>> {
        match self.ki {
            Some(v) => Some(ki(v)),
            None => None,
        }
    }

    fn kd_tf(&self) -> Option<Tf<f64>> {
        match self.kd {
            Some(v) => Some(kd(v)),
            None => None,
        }
    }
}
