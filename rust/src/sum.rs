fn sum(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    return sum;
}

fn sum_all_tails(vecs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sums: Vec<i32> = Vec::new();
    for vec in vecs {
        if vec.len() == 0 {
            sums.push(0)
        } else {
            sums.push(sum(vec[1..vec.len()].to_vec()))
        }
    }
    return sums;
}

#[cfg(test)]
mod tests {
    use crate::sum::sum;
    use crate::sum::sum_all_tails;

    #[test]
    fn test_sum() {
        assert_eq!(sum(vec![1, 2, 3]), 6)
    }

    #[test]
    fn test_tail_sum() {
        let vecs1 = vec![
            vec![1, 2],
            vec![0, 9]
        ];
        assert_eq!(sum_all_tails(vecs1), vec![2, 9]);

        let vecs2 = vec![
            vec![],
            vec![3, 4, 5]
        ];
        assert_eq!(sum_all_tails(vecs2), vec![0, 9]);
    }
}