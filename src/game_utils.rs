
pub fn input_to_int(input: &str) -> vec<usize> {
    let parts: vec<&str> = input.split_whitespace().collect();
    let mut position: vec<usize> = vec::new();

    for part in parts{
        if let (some::<char>(posy), some::<char>(posx)) = (part.chars().next(), part.chars().nth(1)) {
            let posy_usize = posy as usize - 'a' as usize;
            let posx_usize = posx.to_digit(10).unwrap();
            position.push(posy_usize);
            position.push(posx_usize.try_into().unwrap());
        }
    }
    position
}
