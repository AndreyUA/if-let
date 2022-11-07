fn main() {
    let config_max = Some(6u8);

    // match config_max {
    //     Some(max) => println!("Maximum: {}", max),
    //     _ => (),
    // }

    if let Some(max) = config_max {
        println!("Maximum: {}", max)
    }
}
