use lalrpop_util::lalrpop_mod;

/*
macro_rules! lalrpop_mod_doc {
    ($vis:vis $name:ident) => {
        lalrpop_util::lalrpop_mod!(
            #[allow(clippy::ptr_arg)]
            #[allow(clippy::vec_box)]
            $vis $name);
    }
}
*/

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP

#[test]
fn calculator1() {
    let result = calculator1::TermParser::new().parse("and").unwrap();
    println!("{}",result);
    assert_eq!(result, "AND");

    let result = calculator1::TermParser::new().parse("24Kmagic").unwrap();
    println!("{}",result);
    assert_eq!(result, "MAGIC");

    let result = calculator1::TermParser::new().parse("false").unwrap();
    println!("{}",result);
    assert_eq!(result, "FALSE");

    let result = calculator1::TermParser::new().parse("too hot").unwrap();
    println!("{}",result);
    assert_eq!(result, "FALSE");

    let result = calculator1::TermParser::new().parse("today I don't feel like doing any work").unwrap();
    println!("{}",result);
    assert_eq!(result, "EXIT");

    let result = calculator1::TermParser::new().parse("test").unwrap();
    println!("{}",result);
    assert_eq!(result, "ID:test");

    let result = calculator1::TermParser::new().parse("134135343").unwrap();
    println!("{}",result);
    assert_eq!(result, "INTLITERAL:134135343");

    let result = calculator1::TermParser::new().parse(r#""String Literals""#).unwrap();
    println!("{}",result);
    assert_eq!(result, r#"STRINGLITERAL:"String Literals""#);


    let result = calculator1::TermParser::new().parse("~").unwrap();
    println!("{}",result);

    let result = calculator1::TermParser::new().parse("+").unwrap();
    println!("{}",result);
    assert_eq!(result, "CROSS");

    let result = calculator1::TermParser::new().parse("++").unwrap();
    println!("{}",result);
    assert_eq!(result, "POSTINC");

    let result = calculator1::TermParser::new().parse("{").unwrap();
    println!("{}",result);
    assert_eq!(result, "LCURLY");

    let result = calculator1::TermParser::new().parse("}").unwrap();
    println!("{}",result);
    assert_eq!(result, "RCURLY");
}


#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
