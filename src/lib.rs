pub fn sort_vec<T: Ord>(input: &mut [T]) {
    if input.len() >= 2 {
        let i_pivot = input.len() - 1;
        let mut i_swap = 0;
        for i in 0..i_pivot {
            if &input[i] < &input[i_pivot] {
                input.swap(i, i_swap);
                i_swap += 1;
            }
        }
        input.swap(i_pivot, i_swap);
        sort_vec(&mut input[0..i_swap]);
        sort_vec(&mut input[i_swap + 1..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![], vec![])]
    #[case(vec![1], vec![1])]
    #[case(vec![1, 2], vec![1, 2])]
    #[case(vec![2, 1], vec![1, 2])]
    #[case(vec![3, 1, 2], vec![1, 2, 3])]
    #[case(vec![3, 4, 2, 1], vec![1, 2, 3, 4])]
    fn test_empty(#[case] mut input: Vec<i32>, #[case] expected: Vec<i32>) {
        sort_vec(&mut input);
        assert_eq!(expected, input);
    }
}
