pub fn collatz(n: u64) -> Option<u64> {
      let mut counter: Option<u64> = Some(0);

    while n != 1 {
        if n % 2 == 0 {
            // n = n / 2;
            let _n = n / 2;
            // counter += 1; Not valid for Option type
            counter = counter.map(|c| c + 1);
        } else {
            // n = (3 * n ) + 1;
            let _n = (3 * n ) + 1;
            // counter += 1;
            counter = counter.map(|c| c + 1);
        }
    }
    counter
}
