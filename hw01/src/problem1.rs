pub fn sum(slice: &[i32]) -> i32 {
    let mut acc = 0;

    for i in slice {
        acc += i;
    }

    acc
}

pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut acc: Vec<i32> = Vec::new();

    for x in vs {
        if !acc.contains(x) {
            acc.push(*x);
        }
    }

    acc
}

pub fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut acc: Vec<i32> = Vec::new();

    for x in vs {
        if pred(*x) {
            acc.push(*x);
        }
    }

    acc
}
