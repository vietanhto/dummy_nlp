use std::cmp;

pub fn edit_distance(a: &String, b: &String) -> usize {
    let m = a.len();
    let n = b.len();

    let mut mem = vec![vec![0; n + 1]; m + 1];

    for i in 1..(m + 1) {
        mem[i][0] = i;
    }

    for i in 1..(n + 1) {
        mem[0][i] = i;
    }

    for (x, ca) in a.chars().enumerate() {
        for (y, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            mem[x + 1][y + 1] = cmp::min(
                mem[x][y] + cost,
                cmp::min(mem[x][y + 1] + 1, mem[x + 1][y] + 1),
            );
        }
    }

    mem[m][n]
}

pub fn edit_distance_str(a: &str, b: &str) -> usize {
    edit_distance(&a.to_string(), &b.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance_str("", ""), 0);
        assert_eq!(edit_distance_str("edit", "edito"), 1);
        assert_eq!(edit_distance_str("GUMBO", "GAMBOL"), 2);
        assert_eq!(edit_distance_str("kitten", "sitting"), 3);
    }
}
