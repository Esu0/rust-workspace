use im::HashMap;
fn main() {
    let map = HashMap::new();
    let mut map2 = map.update(1, "one");
    let map3 = map2.update(2, "two");
    let map4 = map3.update(3, "three");
    println!("{:?}", map);
    println!("{:?}", map2);
    println!("{:?}", map3);
    println!("{:?}", map4);

    map2.insert(3, "threee");
    let map5 = map2.clone();
    map2.remove(&3);
    println!("{:?}", map2);
    println!("{:?}", map5);
}
