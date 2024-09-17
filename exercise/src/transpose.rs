#[allow(dead_code)]
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            println!("{}", item);
            new_matrix[j][i] = matrix[i][j];
        }
    }
    new_matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transpose() {
        let matrix = [
            [101, 102, 103], //
            [201, 202, 203],
            [301, 302, 303],
        ];
        let transposed = transpose(matrix);
        assert_eq!(
            transposed,
            [
                [101, 201, 301], //
                [102, 202, 302],
                [103, 203, 303],
            ]
        );
    }
}
