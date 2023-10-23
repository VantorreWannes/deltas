#[derive(Debug, Clone, PartialEq)]
pub struct Lcs<'a> {
    source: &'a [u8],
    target: &'a [u8],
    table: Vec<Vec<usize>>,
}

impl<'a> Lcs<'a> {
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        let source_length = source.len();
        let target_length = target.len();
        let mut table = vec![vec![0usize; target_length + 1]; source_length + 1];

        for x in 0..=source_length {
            for y in 0..=target_length {
                if x == 0 || y == 0 {
                    table[x][y] = 0
                } else if source[x - 1] == target[y - 1] {
                    table[x][y] = table[x - 1][y - 1] + 1
                } else {
                    table[x][y] = table[x - 1][y].max(table[x][y - 1])
                }
            }
        }

        Self {
            table,
            source,
            target,
        }
    }

    pub fn length(&self) -> usize {
        let source_length = self.source.len();
        let target_length = self.target.len();
        self.table[source_length][target_length]
    }

    pub fn subsequence(&self) -> Vec<u8> {
        let mut index = self.length();
        let mut subsequence: Vec<u8> = vec![0; index + 1];

        let mut x = self.source.len();
        let mut y = self.target.len();
        while x > 0 && y > 0 {
            if self.source[x - 1] == self.target[y - 1] {
                subsequence[index - 1] = self.source[x - 1];
                x -= 1;
                y -= 1;
                index -= 1
            } else if self.table[x - 1][y] > self.table[x][y - 1] {
                x -= 1
            } else {
                y -= 1
            }
        }

        subsequence.pop();
        subsequence
    }
}

#[cfg(test)]
mod lcs_tests {
    use super::*;

    #[test]
    fn new() {
        let lcs = Lcs::new(&[0, 0, 0], &[0, 0, 0]);
        assert_eq!(lcs.table.iter().flatten().sum::<usize>(), 14);
    }

    #[test]
    fn length() {
        let mut lcs = Lcs::new(&[], &[]);
        assert_eq!(lcs.length(), 0);

        lcs = Lcs::new(&[0], &[0]);
        assert_eq!(lcs.length(), 1);
    }

    #[test]
    fn subsequence() {
        let lcs = Lcs::new(&[0, 1, 2], &[0, 1, 2]);
        assert_eq!(lcs.subsequence(), &[0, 1, 2]);

        let lcs = Lcs::new(b"XMJYAUZ", b"MZJAWXU");
        assert_eq!(lcs.subsequence(), b"MJAU");

        let lcs = Lcs::new(b"AAA", b"");
        assert_eq!(lcs.subsequence(), b"");
    }
}
