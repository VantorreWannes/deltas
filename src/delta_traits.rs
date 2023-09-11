use std::slice::Iter;

pub trait ConvertBetweenBytes {
    type Error;

    fn to_bytes(&self) -> Vec<u8>;
    fn try_from_bytes(bytes: &mut Iter<u8>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait ApplyDeltaTo {
    type Error; 

    fn apply_to(&self, source: &mut Iter<u8>) -> Result<Vec<u8>, Self::Error>;
}

pub trait LCS {

    fn lcs(source: &[u8], target: &[u8]) -> Vec<u8> {
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
}
