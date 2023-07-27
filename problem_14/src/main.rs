fn main() {
    let max_num = 1000000;
    let mut longest_cycle = 0;
    let mut longest_number = 1;

    for n in 2..max_num {
        let mut cycle_len = 1;
        let mut k = n as u64;

        while k != 1 {
            if k % 2 == 0 {
                k /= 2;
                cycle_len += 1;
            } else {
                k = (3 * k + 1) / 2;
                cycle_len += 2;
            }
        }

        if cycle_len > longest_cycle {
            longest_cycle = cycle_len;
            longest_number = n;
        }
    }

    println!("{}", longest_number);
}
