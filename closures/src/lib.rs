pub fn first_fifty_even_square() -> Vec<i32> {
    let arr: Vec<i32> = (1..)
        .filter(|nb| nb % 2 == 0)
        .map(|nb| nb * nb)
        .take(50)
        .collect();
    arr
}
