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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance(&"".to_string(), &"".to_string()), 0);
        assert_eq!(edit_distance(&"edit".to_string(), &"edito".to_string()), 1);
        assert_eq!(
            edit_distance(&"GUMBO".to_string(), &"GAMBOL".to_string()),
            2
        );
    }
}
