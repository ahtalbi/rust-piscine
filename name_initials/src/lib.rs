pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for name in names {
        let mut init = String::new();

        for word in name.split_whitespace() {
            let first_char = word.chars().next().unwrap();

            if first_char.is_uppercase() {
                init.push(first_char);
                init.push('.');
                init.push(' ');
            }
        }

        init.pop();
        res.push(init);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        println!("{:?}", initials(names));
        assert_eq!(initials(names), vec!["H. P.".to_string(), "S. E.".to_string(), "J. L.".to_string(), "B. O.".to_string()]);
    }
}
