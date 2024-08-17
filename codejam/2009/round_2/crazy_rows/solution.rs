lib::run!();

fn read() -> Vec<usize> {
    lib::input!(usize as n);
    let mut nums: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        lib::input!(Vec<char> as row);
        while let Some(&v) = row.last() {
            if v == '1' {
                break;
            }
            row.pop();
        }
        nums.push(row.len());
    }
    nums
}

fn solve(mut nums: Vec<usize>) -> String {
    let mut cost = 0usize;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[j] > i + 1 {
                continue;
            }
            cost += j - i;
            let v = nums[j];
            for k in (i..j).rev() {
                nums[k + 1] = nums[k];
            }
            nums[i] = v;
            break;
        }
    }
    format!("{cost}")
}
