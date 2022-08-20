#[derive(Debug)]
pub enum MapCharError {
    /// A mapping for this char does not exit.
    NoMapping(char),
}

pub fn map_str(s: &str) -> Result<[String; 5], MapCharError> {
    let mut out: [String; 5] = ["".into(), "".into(), "".into(), "".into(), "".into()];

    for c in s.chars() {
        let letter = map_char(c)?;
        for (letter_row, out_row) in letter.iter().zip(out.iter_mut()) {
            out_row.push_str(letter_row);
            out_row.push(' ');
        }
    }

    for s in out.iter_mut() {
        *s = s.trim_end().to_owned();
    }

    Ok(out)
}

pub const fn map_char(c: char) -> Result<[&'static str; 5], MapCharError> {
    use crate::letters::MapCharError::NoMapping;

    Ok(match c.to_ascii_uppercase() {
        'A' => A,
        'B' => B,
        'C' => C,
        'D' => D,
        'E' => E,
        'F' => F,
        'G' => G,
        'H' => H,
        'I' => I,
        'J' => J,
        'K' => K,
        'L' => L,
        'M' => M,
        'N' => N,
        'O' => O,
        'P' => P,
        'Q' => Q,
        'R' => R,
        'S' => S,
        'T' => T,
        'U' => U,
        'V' => V,
        'W' => W,
        'X' => X,
        'Y' => Y,
        'Z' => Z,
        ' ' => SPACE,
        o => return Err(NoMapping(o))
    })
}


const A: [&'static str; 5] = [
    " ## ",
    "#  #",
    "####",
    "#  #",
    "#  #",
];

const B: [&'static str; 5] = [
    "### ",
    "#  #",
    "### ",
    "#  #",
    "### ",
];

const C: [&'static str; 5] = [
    " ###",
    "#   ",
    "#   ",
    "#   ",
    " ###",
];

const D: [&'static str; 5] = [
    "### ",
    "#  #",
    "#  #",
    "#  #",
    "### ",
];

const E: [&'static str; 5] = [
    "####",
    "#   ",
    "####",
    "#   ",
    "####",
];

const F: [&'static str; 5] = [
    "####",
    "#   ",
    "### ",
    "#   ",
    "#   ",
];

const G: [&'static str; 5] = [
    " ###",
    "#   ",
    "# ##",
    "#  #",
    " ###",
];

const H: [&'static str; 5] = [
    "#  #",
    "#  #",
    "####",
    "#  #",
    "#  #",
];

const I: [&'static str; 5] = [
    "###",
    " # ",
    " # ",
    " # ",
    "###",
];

const J: [&'static str; 5] = [
    " ###",
    "  # ",
    "  # ",
    "# # ",
    "### ",
];

const K: [&'static str; 5] = [
    "#  #",
    "# # ",
    "### ",
    "#  #",
    "#  #",
];

const L: [&'static str; 5] = [
    "#   ",
    "#   ",
    "#   ",
    "#   ",
    "####",
];

const M: [&'static str; 5] = [
    "#   #",
    "## ##",
    "# # #",
    "#   #",
    "#   #",
];

const N: [&'static str; 5] = [
    "#   #",
    "##  #",
    "# # #",
    "#  ##",
    "#   #",
];

const O: [&'static str; 5] = [
    " ## ",
    "#  #",
    "#  #",
    "#  #",
    " ## ",
];

const P: [&'static str; 5] = [
    "### ",
    "#  #",
    "### ",
    "#   ",
    "#   ",
];

const Q: [&'static str; 5] = [
    " ## ",
    "#  #",
    "#  #",
    "#  #",
    " ###",
];

const R: [&'static str; 5] = [
    "### ",
    "#  #",
    "### ",
    "#  #",
    "#  #",
];

const S: [&'static str; 5] = [
    "####",
    "#   ",
    "####",
    "   #",
    "####",
];

const T: [&'static str; 5] = [
    "#####",
    "  #  ",
    "  #  ",
    "  #  ",
    "  #  ",
];

const U: [&'static str; 5] = [
    "#  #",
    "#  #",
    "#  #",
    "#  #",
    "####",
];

const V: [&'static str; 5] = [
    "#   #",
    "#   #",
    "#   #",
    " # # ",
    "  #  ",
];

const W: [&'static str; 5] = [
    "#   #",
    "#   #",
    "# # #",
    "# # #",
    "#### ",
];

const X: [&'static str; 5] = [
    "#   #",
    " # # ",
    "  #  ",
    " # # ",
    "#   #",
];

const Y: [&'static str; 5] = [
    "#   #",
    "#   #",
    " ### ",
    "  #  ",
    "  #  ",
];

const Z: [&'static str; 5] = [
    "####",
    "   #",
    "  # ",
    " #  ",
    "####",
];

const SPACE: [&'static str; 5] = [
    "  ",
    "  ",
    "  ",
    "  ",
    "  ",
];

#[test]
fn equal_length_letters() {
    for letter in [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z] {
        assert!(letter.iter().zip(letter.iter().skip(1))
            .all(|(&c1, &c2)| c1.len() == c2.len()),
            "letter {:?} failed",
            letter
        )
    }
}