pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut arr: Vec<Box<u32>> = Vec::new();
    for ele in s.split_whitespace() {
        if ele.ends_with('k') {
            let nb: f64 = ele[0..ele.len() - 1].parse().unwrap();
            let mult = (nb * 1000.0) as u32;
            arr.push(Box::new(mult));
        }else{
            arr.push(Box::new(ele.parse().unwrap()));
        }
    }
    arr
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut res = Vec::new();
    for i in a {
        res.push(*i);
    }
    res
}
