fn main() {
    // initialised vector
    let mut v = Vec::new();

    // populate 
    for i in 101..106 {
        v.push(i.to_string());
    }

    // Pop the last value of the vec and assign it to a variable 
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    // Pop the value of a certain index and assign it to a varible, the last value of the vec take the place
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // Change a value with another in the same place
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "substitute"]);
}
