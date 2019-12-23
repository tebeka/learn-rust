use std::error::Error;

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn sqrt(val: f64) -> Result<f64, String> {
    if val < 0.0 {
        return Err(format!("negative value - {}", val));
    }

    if val == 0.0 {
        return Ok(val);
    }

    let epsilon: f64 = 0.001;
    let mut guess: f64 = 1.0;
    for _ in 0..10_000 {
        if (val - guess * guess).abs() < epsilon {
            return Ok(guess);
        }
        guess = (val / guess + guess) / 2.0;
    }

    Err(format!("can't find sqrt for {}", val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hold() {
        let r1 = Rect {
            width: 10,
            height: 20,
        };
        let r2 = Rect {
            width: 5,
            height: 17,
        };
        assert!(r1.can_hold(&r2));
        assert!(!r2.can_hold(&r1));
    }

    #[test]
    fn check_sqrt() {
        assert_eq!(1.4142156862745097, sqrt(2.0).unwrap());
        assert_eq!(0.0, sqrt(0.0).unwrap());
    }
}
