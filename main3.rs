// use logos::{Logos, Lexer};

// // // Note: callbacks can return `Option` or `Result`
// // fn kilo(lex: &mut Lexer<Token>) -> Option<u64> {
// //     let slice = lex.slice();
// //     let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'k'
// //     Some(n * 1_000)
// // }

// // fn mega(lex: &mut Lexer<Token>) -> Option<u64> {
// //     let slice = lex.slice();
// //     let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'm'
// //     Some(n * 1_000_000)
// // }

// // fn meow<T> (lex: &mut Lexer<Token>) -> Option<&'a str> {
// //     let slice = lex.slice();
// //     Some(slice)
// // }

// #[derive(Logos, Debug, PartialEq)]
// #[logos(skip r"[ \t\n\f]+")]
// enum Token<'a> {
//     #[token("magic")]
//     MAGIC,

//     #[regex("[a-zA-Z]+", |lex| lex.slice() )]
//     STRINGLIT(&'a str),
//     // Callbacks can use closure syntax, or refer
//     // to a function defined elsewhere.
//     //
//     // Each pattern can have it's own callback.
//     // #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
//     // #[regex("[0-9]+k", kilo)]
//     // #[regex("[0-9]+m", mega)]
//     // Number(u64),
// }

// fn main() {
//     let meow = "gbfhfdgbjd";

//     for result in Token::lexer(meow) {
//         // dbg!(&result);
//         match result {
//             Ok(token) => println!("owo {:#?}", token.collect()),
//             Err(_e) => panic!(),
//         }
//     }
// }

// use logos::Logos;

// #[derive(Logos, Debug, PartialEq)]
// #[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
// enum Token<'a> {
//     // Tokens can be literal strings, of any length.
//     #[token("fast")]
//     Fast,

//     #[token(".")]
//     Period,

//     // Or regular expressions.
//     #[regex("[a-zA-Z]+")]
//     Text(&'a str),
// }

// fn main() {
//     for result in Token::lexer("Create ridiculously fast Lexers.") {
//         dbg!(result.unwrap());
//         //println!("{}", result.unwrap());
//         // match result {
//         //     //Ok(token) => println!("{:#?}", token),
//         //     Ok(token) => dbg!(token),
//         //     Err(()) => panic!("some error occured."),
//         // }
//     }

//     let mut lex = Token::lexer("Create ridiculously fast Lexers.");
//     dbg!(&lex);
//     lex.next();
//     println!("{}", &lex.slice());
// }

// use logos::{Logos, Lexer};

// // Note: callbacks can return `Option` or `Result`
// fn kilo(lex: &mut Lexer<Token>) -> Option<u64> {
//     let slice = lex.slice();
//     let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'k'
//     Some(n * 1_000)
// }

// fn mega(lex: &mut Lexer<Token>) -> Option<u64> {
//     let slice = lex.slice();
//     let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'm'
//     Some(n * 1_000_000)
// }



// #[derive(Logos, Debug, PartialEq)]
// #[logos(skip r"[ \t\n\f]+")]
// enum Token {
//     #[token("and")]
//     AND,

//     #[token("magic")]
//     MAGIC,

//     #[token("bool")]
//     BOOL,

//     #[token("else")]
//     ELSE,

//     #[token("false")]
//     FALSE,

//     #[token("if")]
//     IF,

//     #[token("int")]
//     INT,

//     #[token("give")]
//     GIVE,

//     #[token("take")]
//     TAKE,

//     #[token("or")]
//     OR,

//     #[token("perfect")]
//     PERFECT,

//     #[token("return")]
//     RETURN,

//     #[token("class")]
//     CLASS,

//     #[token("true")]
//     TRUE,

//     #[token("exit")]
//     EXIT,

//     #[token("void")]
//     VOID,

//     #[token("while")]
//     WHILE,

//     #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
//     #[regex("[0-9]+k", kilo)]
//     #[regex("[0-9]+m", mega)]
//     Number(u64),

//     #[regex("=")]
//     ASSIGN,

//     #[regex(":")]
//     COLON,

//     #[regex(",")]
//     COMMA,

//     #[regex(r#"\+"#)]
//     CROSS,

//     #[regex("-")]
//     DASH,

//     #[regex("==")]
//     EQUALS,

//     #[regex(">")]
//     GREATER,

//     #[regex(">=")]
//     GREATEREQ,

//     #[regex(r#"\{"#)]
//     LCURLY,

//     #[regex("<")]
//     LESS,

//     #[regex("<=")]
//     LESSEQ,

//     #[regex(r#"\("#)]
//     LPAREN,

//     #[regex("!")]
//     NOT,

//     #[regex("!=")]
//     NOTEQUALS,

//     #[regex("--")]
//     POSTDEC,

//     #[regex(r#"\+\+"#)]
//     POSTINC,

//     #[regex("}")]
//     RCURLY,

//     #[regex(r#"\)"#)]
//     RPAREN,

//     #[regex(";")]
//     SEMICOL,

//     #[regex(r#"/"#)]
//     SLASH,

//     #[regex(r#"\*"#)]
//     STAR,
// }

// fn main() {
//     let owo = "== + {} () ;!=/,/,magic 5 42k 75m";
//     let lex: Vec<_> = Token::lexer(owo).spanned().collect();

//     for (result, span) in lex {
//         let span = span.start + 1;
//         match result {
//             Ok(t) => {
//                 match t {
//                     Token::Number(value) => println!("NUMBER:{} [1,{}]", value, span),
//                     _ => println!("{:#?} [1,{}]", t, span),
//                 }
//             },
//             Err(()) => panic!("some error occured."),
//         }
//     }
// }



use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("fast")]
    Fast,

    #[token(".")]
    Period,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,
}

fn main() {
    let mut lex = Token::lexer("Create ridiculously fast Lexers.");
    // //dbg!(&lex);
    // dbg!(lex.next());
    // //dbg!(&lex);
    // dbg!(lex.slice());
    // //dbg!(&lex);
    // for result in Token::lexer("Create ridiculously fast Lexers.") {
    //     dbg!(&result.slice());
    //     // match result {
    //     //     Ok(token) => {
    //     //         match token {
    //     //             Token::Text => println!("{:#?}", token),
    //     //             _ => println!("{:#?}", token)
    //     //         }
    //     //     },
    //     //     Err(()) => panic!("some error occured."),
    //     // }
    // }
    let mut i = 0;
    loop {
        println!("{:#?}:{} [{:#?}]", lex.next().unwrap().unwrap(), lex.slice(), lex.span());
        i += 1;
        if i == 5 {
            break;
        }
    }
}