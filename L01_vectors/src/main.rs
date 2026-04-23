fn main() {
    let mut scores: Vec<u16> = vec![10, 20, 30, 40];
    scores.push(15);
    scores.push(100);
    println!("{:?}", scores);

    // for s in scores.iter(){
    //     println!("{s}");
    // }

    let last_score = scores.pop().unwrap();
    println!("last_score: {:?}", last_score);
    println!("{:?}", scores);

    let mut colors = Vec::new();
    colors.push(String::from("green"));
    colors.push(String::from("red"));
    colors.push(String::from("blue"));
    println!("{:?}", colors);
    colors.reverse();
    println!("{:?}", colors);

    let codes: Vec<u8> = (50..=100).collect();
    println!("{:?}", codes);

    let numbers = (8..=27).collect::<Vec<u8>>();
    let first_two = numbers[10..16].to_vec();
    println!("{:?}", first_two);
}
