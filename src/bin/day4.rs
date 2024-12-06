use std::io::Read;

use advent_of_code::get_input;

fn main() {
    let mut input = get_input(4);
    let mut buf = Vec::new();
    input.read_to_end(&mut buf).unwrap();

    let word_search = WordSearch::new(&buf);
    println!("{}", word_search.count_xmas());
    println!("{}", word_search.count_mas_x());
}

struct WordSearch {
    letters: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl WordSearch {
    fn new(input: &[u8]) -> Self {
        let w = input.iter().position(|b| *b == b'\n').unwrap();
        let h = (input.len() + 1) / (w + 1);

        let mut rows = Vec::new();

        let mut row = 0;
        loop {
            if row >= h {
                break;
            }
            let start = (w + 1) * row;
            let end = start + w;

            let row_v = input[start..end].to_vec();

            rows.push(row_v);
            row += 1;
        }

        Self {
            letters: rows,
            width: w,
            height: h,
        }
    }

    fn count_xmas(&self) -> usize {
        let diffs = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ];
        let mut sum = 0;

        for (r, row) in self.letters.iter().enumerate() {
            for (c, byte) in row.iter().enumerate() {
                if *byte != b'X' {
                    continue;
                }

                let letter_sum = diffs
                    .iter()
                    .map(|diffs| self.check_xmas((r, c), *diffs))
                    .fold(0, |acc, value| if value { acc + 1 } else { acc });

                sum += letter_sum;
            }
        }

        sum
    }

    fn count_mas_x(&self) -> usize {
        let mut sum = 0;
        for (r, row) in self.letters.iter().enumerate() {
            for (c, byte) in row.iter().enumerate() {
                if byte != &b'A' {
                    continue;
                }

                if self.check_mas_x(r, c) {
                    sum += 1;
                }
            }
        }

        sum
    }

    fn check_xmas(&self, (r, c): (usize, usize), (rd, cd): (isize, isize)) -> bool {
        let xmas = [b'X', b'M', b'A', b'S'];
        for (i, letter) in xmas.iter().copied().enumerate() {
            let rd = rd * i as isize;
            let cd = cd * i as isize;

            let r = (r as isize + rd) as usize;
            let c = (c as isize + cd) as usize;

            let Some(c) = self.get_letter(r, c) else {
                return false;
            };

            if *c != letter {
                return false;
            }
        }

        true
    }

    fn check_mas_x(&self, r: usize, c: usize) -> bool {
        if r == 0 || r == self.width - 1 || c == 0 || c == self.height - 1 {
            return false;
        }

        match (self.get_letter(r - 1, c - 1), self.get_letter(r + 1, c + 1)) {
            (Some(b'M'), Some(b'S')) => (),
            (Some(b'S'), Some(b'M')) => (),
            _ => return false,
        }

        match (self.get_letter(r - 1, c + 1), self.get_letter(r + 1, c - 1)) {
            (Some(b'M'), Some(b'S')) => (),
            (Some(b'S'), Some(b'M')) => (),
            _ => return false,
        }

        true
    }

    fn get_letter(&self, r: usize, c: usize) -> Option<&u8> {
        if let Some(row) = self.letters.get(r) {
            row.get(c)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::WordSearch;

    #[test]
    fn count_xmas_test_data() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let expected = 18;
        let actual = WordSearch::new(input.as_bytes()).count_xmas();

        assert_eq!(actual, expected);
    }

    #[test]
    fn count_xmas_single() {
        let input = [
            "XMAS\n",
            "SAMX\n",
            r#"X
M
A
S"#,
            r#"S
A
M
X"#,
        ];

        for input in input {
            let actual = WordSearch::new(input.as_bytes()).count_xmas();
            assert_eq!(actual, 1, "input: {}", input);
        }
    }

    #[test]
    fn count_mas_x_single() {
        let input = r#"SFM
FAF
SFM"#;
        let actual = WordSearch::new(input.as_bytes()).count_mas_x();

        assert_eq!(actual, 1);
    }

    #[test]
    fn count_mas_x_test_data() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let actual = WordSearch::new(input.as_bytes()).count_mas_x();

        assert_eq!(actual, 9);
    }
}
