pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut res = Vec::new();
    for element in arr.clone() {
        let mut list: Vec<usize> = arr.iter().cloned().filter(|ele| *ele != element).collect();
        res.push(list.iter().product());
        list.clear();
    }
    res
}
