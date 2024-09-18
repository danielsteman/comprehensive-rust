// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

#[allow(dead_code)]
fn magnitude(v: &[f64]) -> f64 {
    v.iter().map(|x| x.sqrt()).sum()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

#[allow(dead_code)]
fn normalize(v: &mut [f64]) {
    let magnitude = magnitude(&v);
    for num in v.iter_mut() {
        *num /= magnitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        println!(
            "Magnitude of a unit vector: {}",
            magnitude(&[0.0, 1.0, 0.0])
        );

        let v = [1.0, 2.0, 9.0];
        println!("Magnitude of {v:?}: {}", magnitude(&v));
    }

    #[test]
    fn test_normalize() {
        let mut v = [1.0, 2.0, 9.0];
        println!("Magnitude of {v:?}: {}", magnitude(&v));
        normalize(&mut v);
        println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
    }
}
