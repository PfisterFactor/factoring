pub mod poly {

    pub struct Polynomial {
        a: f64,
        b: f64,
        c: f64,
    }
    struct Subpolynomial {
        coefficent: f64,
        degree: u8,
    }

    impl<S> From<S> for Subpolynomial
    where
        S: Into<String>,
    {
        fn from(value: S) -> Self {
            let subpoly = value.into();

            let coefficent_end_index = subpoly.find('x').unwrap_or_else(|| subpoly.len());
            let degree_start_index = subpoly.find('^').and_then(|i| {
                if i + 1 >= subpoly.len() {
                    None
                } else {
                    Some(i + 1)
                }
            });

            let mut coefficent = (&subpoly[0..coefficent_end_index]).to_owned();
            let degree = degree_start_index.map(|i| (&subpoly[i..]));

            if coefficent.is_empty() || coefficent == "-" || coefficent == "+" {
                coefficent += "1";
            }

            let degree = degree.unwrap_or_else(|| {
                if coefficent_end_index == subpoly.len() {
                    "0"
                } else {
                    "1"
                }
            });

            Self {
                coefficent: coefficent.parse().unwrap(),
                degree: degree.parse().unwrap(),
            }
        }
    }

    impl<S> From<S> for Polynomial
    where
        S: Into<String>,
    {
        fn from(value: S) -> Self {
            let polynomial = value.into().replace(" ", "").replace("\t", "");

            let mut subpoly_vector: Vec<Subpolynomial> = Vec::new();

            let mut subpoly_buffer = String::new();

            for i in 0..=polynomial.len() {
                let c = polynomial.as_bytes().get(i).map(|x| *x as char);

                if (c.is_none() || c == Some('+') || c == Some('-')) && !subpoly_buffer.is_empty() {
                    let index: Option<usize> = i.checked_sub(subpoly_buffer.len() + 1);
                    let sign_char = index.map(|i| polynomial.as_bytes()[i] as char);
                    if sign_char.is_some()
                        && (sign_char.unwrap() == '+' || sign_char.unwrap() == '-')
                    {
                        subpoly_buffer.insert(0, sign_char.unwrap());
                    }
                    subpoly_vector.push(Subpolynomial::from(subpoly_buffer.as_str()));
                    subpoly_buffer.clear();
                }

                if c.is_some() {
                    let c = c.unwrap();
                    if c.is_alphanumeric() || c == '^' || c == '.' {
                        subpoly_buffer.push(c);
                    }
                }
            }

            Self {
                a: subpoly_vector
                    .iter()
                    .filter(|x| x.degree == 2_u8)
                    .fold(0_f64, |acc, x| acc + x.coefficent),
                b: subpoly_vector
                    .iter()
                    .filter(|x| x.degree == 1_u8)
                    .fold(0_f64, |acc, x| acc + x.coefficent),
                c: subpoly_vector
                    .iter()
                    .filter(|x| x.degree == 0_u8)
                    .fold(0_f64, |acc, x| acc + x.coefficent),
            }
        }
    }
    impl Polynomial {
        pub fn roots(&self) -> (f64, f64) {
            let root1 =
                (-self.b + (self.b * self.b - 4_f64 * self.a * self.c).sqrt()) / (2_f64 * self.a);
            let root2 =
                (-self.b - (self.b * self.b - 4_f64 * self.a * self.c).sqrt()) / (2_f64 * self.a);
            (root1, root2)
        }
    }

}

use clap::{App, Arg};
use poly::Polynomial;

fn poly_validator(s: String) -> Result<(), String> {
    if !s.is_ascii() {
        return Result::Err("Polynomial not ASCII.".to_owned());
    }
    if s.len() > 100 {
        return Result::Err("Polynomial too long.".to_owned());
    }
    let allowed_chars = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '^', '.', 'x', ' ', '\t',
    ];
    for c in s.chars() {
        if !allowed_chars.contains(&c) {
            return Result::Err("Polynomial has unsupported characters or is not basic.".to_owned());
        }
    }
    Result::Ok(())
}

fn main() {
    let matches = App::new("Factoring")
        .version("1.0")
        .author("Eric Pfister")
        .about("Factors basic polynomials into their (real) factors.")
        .usage("factoring <POLYNOMIAL>")
        .arg(
            Arg::with_name("POLYNOMIAL")
                .required(true)
                .validator(poly_validator)
                .help("A basic polynomial in the form of Ax^2 + Bx + C"),
        )
        .get_matches();

    let polynomial_str: &str = matches.value_of("POLYNOMIAL").unwrap();
    let polynomial: Polynomial = Polynomial::from(polynomial_str);
    let roots = polynomial.roots();
    if roots.0.is_finite() && roots.1.is_finite() {
        println!(
            "Factors of ({}) are {:.4}, and {:.4}",
            polynomial_str, roots.0, roots.1
        );
    } else {
        println!("Factors of ({}) are imaginary", polynomial_str);
    }
}

#[cfg(test)]
mod tests {
    use crate::poly::*;

    #[test]
    fn basic_polynomial() {
        let string = "x^2 + 4x + 4";
        let poly = Polynomial::from(string);
        let roots = poly.roots();

        assert_eq!(roots, (-2_f64, -2_f64));
    }
    #[test]
    fn imaginary_polynomial() {
        let string = "5x^2 + 4x + 4";
        let poly = Polynomial::from(string);
        let roots = poly.roots();

        assert!(!roots.0.is_finite() || !roots.1.is_finite());
    }

    #[test]
    fn format_polynomial() {
        let string =
            "x^2 + -0.5x + -2.5x + 2.5x + 0.5x + 4x + 8x - 4x -+-4x + 4 + 12 --+-8         -4";
        let poly = Polynomial::from(string);
        let roots = poly.roots();
        assert_eq!(roots, (-2_f64, -2_f64));
    }
}
