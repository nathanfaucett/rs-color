use collections::string::String;

use vec4;
use num::Num;
use regex::Regex;


#[inline]
fn to_256_str<T: Copy + Num>(value: T) -> String {
    (value * T::from_usize(255usize)).round().to_string()
}

#[inline]
pub fn to_rgb<T: Copy + Num>(out: [T; 4]) -> String {
    concat_string!("rgb(", &to_256_str(out[0]), ", ", &to_256_str(out[1]), ", ", &to_256_str(out[2]), ")")
}
#[test]
fn test_to_rgb() {
    assert_eq!(&to_rgb([1.0, 0.5, 0.0, 1.0]), "rgb(255, 128, 0)")
}

#[inline]
pub fn to_rgba<T: Copy + Num>(out: [T; 4]) -> String {
    concat_string!("rgba(", &to_256_str(out[0]), ", ", &to_256_str(out[1]), ", ", &to_256_str(out[2]), ", ", &(out[3].to_string()), ")")
}
#[test]
fn test_to_rgba() {
    assert_eq!(&to_rgba([1.0, 0.5, 0.0, 1.0]), "rgba(255, 128, 0, 1)")
}

#[inline]
fn to_number<T: Copy + Num>(value: &str) -> T {
    T::from_f64(value.parse::<f64>().unwrap())
}

#[inline]
fn to_256<T: Copy + Num>(value: &str) -> T {
    to_number::<T>(value).min(&T::from_usize(255usize)) / T::from_usize(255usize)
}

#[inline]
pub fn from_rgba<T: Copy + Num>(out: &mut [T; 4], string: String) -> &mut [T; 4] {
    let re = Regex::new(r"^rgba\((?:\s+)?(\d+),(?:\s+)?(\d+),(?:\s+)?(\d+),(?:\s+)?((?:\.)?\d+(?:\.\d+)?)\)$").unwrap();

    match re.captures(&string) {
        Some(matches) => {
            out[0] = to_256(matches.get(1).unwrap().as_str());
            out[1] = to_256(matches.get(2).unwrap().as_str());
            out[2] = to_256(matches.get(3).unwrap().as_str());
            out[3] = to_number::<T>(
                matches.get(4).unwrap().as_str()
            ).min(&T::from_usize(1usize));
        },
        None => {
            vec4::set(out, T::zero(), T::zero(), T::zero(), T::one());
        },
    }
    out
}
#[test]
fn test_from_rgba() {
    let mut v = [0.0, 0.0, 0.0, 1.0];

    from_rgba(&mut v, String::from("rgba( 255, 128, 0, 1)"));
    assert_eq!(v, [1.0, 128.0 / 255.0, 0.0, 1.0]);

    from_rgba(&mut v, String::from("rgba(0, 0, 0, 1)"));
    assert_eq!(v, [0.0, 0.0, 0.0, 1.0]);
}

#[inline]
pub fn from_rgb<T: Copy + Num>(out: &mut [T; 4], string: String) -> &mut [T; 4] {
    let re = Regex::new(r"^rgb\((?:\s+)?(\d+),(?:\s+)?(\d+),(?:\s+)?(\d+)\)$").unwrap();

    match re.captures(&string) {
        Some(matches) => {
            out[0] = to_256(matches.get(1).unwrap().as_str());
            out[1] = to_256(matches.get(2).unwrap().as_str());
            out[2] = to_256(matches.get(3).unwrap().as_str());
            out[3] = T::one();
        },
        None => {
            vec4::set(out, T::zero(), T::zero(), T::zero(), T::one());
        },
    }

    out
}
#[test]
fn test_from_rgb() {
    let mut v = [0.0, 0.0, 0.0, 1.0];

    from_rgb(&mut v, String::from("rgb( 255, 128, 0)"));
    assert_eq!(v, [1.0, 128.0 / 255.0, 0.0, 1.0]);

    from_rgb(&mut v, String::from("rgb(0, 0, 0)"));
    assert_eq!(v, [0.0, 0.0, 0.0, 1.0]);
}
