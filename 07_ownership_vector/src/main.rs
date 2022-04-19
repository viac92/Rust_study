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

    // Pop the value of a certain index and assign it to a varible, the last value of the vec take 
    // mettendolo al suo posto.
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // Scambio un valore con un altro nello stesso posto
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // controlliamo quello che è rimasto nel vettore
    assert_eq!(v, vec!["101", "104", "substitute"]);
}
