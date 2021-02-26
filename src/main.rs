fn great_world() {
    let poland = "cześć";
    let usa = "hello";
    let regions = [poland, usa];
    for region in regions.iter() {
        println!("{}", region);
    }
}

fn main() {
    great_world();
}
