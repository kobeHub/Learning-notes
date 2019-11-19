/*Usage of the vector*/
pub fn test_vec() {
    let mut v_mut: Vec<i32> = Vec::new();
    let v = vec![12, 56, 9];
    v_mut.push(100);
    v_mut.push(12);
    v_mut.push(10000);
    let first = &v_mut[0];
    let second = &v_mut[1];
    //v_mut.push(13);e
    println!("v:{:?}, v_mut:{:?}", v, v_mut);
    println!("the first element of v:{}, v_mut:{}, {}", &v[0], first, second);
    match v_mut.get(10) {
        Some(value) => println!("the 10th element of v_mut:{}", value),
        None => println!("There is no 10th element"),
    }
    let a = v_mut[0];
    v_mut.push(123);
    println!("{}, {:?}", a, v_mut);
}

pub fn iter_vec(v: &Vec<i32>) {
    for i in v {
        print!("{} ", i);
    }
    println!();
}

pub fn add_one(v: &mut Vec<i32>) {
    let v1 = v.clone();
    for i in v {
        *i += 1;
        //print!("{} ", i);
    }
    for i in v1 {
        print!("{} ", i);
    }
    println!();
}
