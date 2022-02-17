fn merge(left: &Vec<u32>, right: &Vec<u32>) -> Vec<u32> {
    let mut merged: Vec<u32> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        merged.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        merged.push(right[j]);
        j += 1;
    }

    merged
}

fn merge_sort(v: Vec<u32>) -> Vec<u32> {
    return if v.len() < 2 {
        v.to_vec()
    } else {
        let size = v.len() / 2;
        let left = merge_sort(v[0..size].to_vec());
        let right = merge_sort(v[size..].to_vec());
        merge(&left, &right)
    };
}

#[cfg(test)]
mod tests {
    use crate::merge_sort;

    #[test]
    fn properly_sort_an_array_of_odd_length() {
        assert_eq!(
            merge_sort([8, 2, 5, 11, 0].to_vec()),
            [0, 2, 5, 8, 11].to_vec()
        );
    }
}

fn main() {}
