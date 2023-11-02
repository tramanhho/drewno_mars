pub fn add_tabs(input: String) -> String {
    let text = input.lines();
    let mut output : String = "".to_string();
    let mut tabs = 0;

    for mut line in text {
        line = line.trim();
        let first_char = &line.chars().next();
        let last_char = &line.chars().last();

        match first_char {
            Some(x) => match x {
                '}' => {if tabs > 0 { tabs -= 1; }},
                _    => ()
            },
            None => (),
        }

        output.push_str(&"    ".repeat(tabs));

        match last_char {
            Some(x) => match x {
                '{' => tabs += 1,
                _   => ()
            },
            None => (),
        }

        output.push_str(&line);
        output.push_str("\n");
    }

    output
}