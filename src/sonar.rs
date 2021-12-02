use itertools::Itertools;

pub fn increments<I: AsRef<[usize]>>(depths: I) -> usize {
    depths.as_ref().windows(2).filter(|w| w[0] < w[1]).count()
}

pub fn sliding_increments<I: AsRef<[usize]>>(depths: I) -> usize {
    depths
        .as_ref()
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(d1, d2, d3)| d1 + d2 + d3)
        .tuple_windows::<(_, _)>()
        .filter(|(w1, w2)| w1 < w2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEPTHS: [usize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn it_can_count_the_number_of_increments() {
        let result = increments(DEPTHS);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_can_count_the_number_of_sliding_increments() {
        let result: usize = sliding_increments(DEPTHS);
        assert_eq!(result, 5);
    }
}
