pub fn day20() {
    let target = 29000000;
    let mut houses = vec![0; target / 10];
    for i in 1..houses.len() {
        let mut loc = i;
        let mut count = 0;
        while loc < houses.len() {
            houses[loc] += i * 11;
            loc += i;
            count += 1;
            if count == 50 {
                break;
            }
        }
    }
    let p = houses.iter().position(|x| x >= &target).unwrap();
    println!("{:?}", p);
}
