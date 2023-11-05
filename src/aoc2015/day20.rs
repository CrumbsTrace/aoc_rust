pub fn run() -> (usize, usize) {
    let target = 36_000_000;
    let mut houses = vec![0_i32; target as usize / 10];
    let mut houses_p2 = vec![0; target as usize / 10];
    for elf in 1..houses.len() {
        for house in (elf..houses.len()).step_by(elf) {
            houses[house] += elf as i32 * 10;
            if elf * 50 >= house {
                houses_p2[house] += elf as i32 * 11;
            }
        }
    }
    let p1 = houses.iter().position(|&p| p >= target).unwrap();
    let p2 = houses_p2.iter().position(|&p| p >= target).unwrap();
    (p1, p2)
}

#[test]
fn real_input() {
    let result = run();
    assert_eq!(result, (831600, 884520));
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    bencher.bench(run);
}
