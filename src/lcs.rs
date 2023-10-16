#[derive(Debug, Clone, PartialEq)]
pub struct Lcs {
    table: Vec<Vec<usize>>,
}

impl Lcs {
    fn example_lcs(source: &[u8], target: &[u8]) -> Vec<u8> {
        let s_len = source.len();
        let t_len = target.len();

        let mut table = vec![vec![0; t_len + 1]; s_len + 1];

        for i in 0..=s_len {
            for j in 0..=t_len {
                if i == 0 || j == 0 {
                    table[i][j] = 0
                } else if source[i - 1] == target[j - 1] {
                    table[i][j] = table[i - 1][j - 1] + 1
                } else {
                    table[i][j] = table[i - 1][j].max(table[i][j - 1])
                }
            }
        }

        let mut index = table[s_len][t_len];
        let mut lcs = vec![0; index + 1];
        lcs[index] = 0;

        let mut i = s_len;
        let mut j = t_len;
        while i > 0 && j > 0 {
            if source[i - 1] == target[j - 1] {
                lcs[index - 1] = source[i - 1];
                i -= 1;
                j -= 1;
                index -= 1
            } else if table[i - 1][j] > table[i][j - 1] {
                i -= 1
            } else {
                j -= 1
            }
        }
        lcs.resize(table[s_len][t_len], 0);

        lcs
    }

    pub fn new(source: &[u8], target: &[u8]) -> Self {
        let source_length = source.len();
        let target_length = source.len();
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

        Self { table }
    }
}


