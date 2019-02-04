
// need nongreedy block comment capture

#[derive(Debug)]
enum Token {
    Label { name : String },  
    Class { name : String, value : String },
    Complex { name : String, components : Vec<Token> },
}

#[derive(Debug)]
enum EndCondition {
    Immediate,
    Until(char),
}

// instead of simple vs complex maybe ambigeous and ?
// char to T
#[derive(Debug)]
enum Rule {
    Simple { start : char
           , end : EndCondition
           , accum : Vec<char>
           , token : String
           }, // TODO need internal rules? (internal rules can only kick in with a simple rule because if you are in a complex rule then you wont know which internal rules to start applying)
    Complex { start : char
            , sub_rules : Vec<Rule>  // competting rules
            , accum : Vec<char> 
            }, 
}

fn lex_next(mut a : (Vec<Rule>, Vec<Token>, Option<Rule>), s_c : (usize, char)) -> (Vec<Rule>, Vec<Token>, Option<Rule>) {
    let mut (rules, tokens, current_rule) = a;
    match current_rule {
        Some(r) => {
            let (index, current) = s_c;
            match r {
                Until(c) if c == current => {
                    let tok = Token::Class{ name: r.token}
                    tokens.push(r.accum);
                    (rules, )
                },
                Immediate => panic!("Encountered a current rule that is immediate"),
            }
        },
        None => {
            let (index, current) = s_c;
            for rule in rules {
                match rule {
                    Rule::Simple { s, e, a, t } if s == current => ,
                    Rule::Complex { s, a, sub } => ,
                }
            }
        }
    }
}

fn main() {
    let a = Rule::Simple { start: 'a', end: EndCondition::Immediate, accum: vec! [], token: "A".to_string() };
    let b = Rule::Simple { start: 'b', end: EndCondition::Immediate, accum: vec! [], token: "B".to_string() };
    let c = Rule::Simple { start: 'c', end: EndCondition::Until('c'), accum: vec! [], token: "C".to_string() };
    let d = Rule::Complex { start: 'd'
                          , accum: vec! []
                          , sub_rules: vec! [ Rule::Complex { start: 'e'
                                                            , accum: vec![]
                                                            , sub_rules : vec! [ Rule::Simple { start: 'f'
                                                                                              , end: EndCondition::Immediate
                                                                                              , accum: vec! []
                                                                                              , token: "F".to_string()
                                                                                              }
                                                                                , Rule::Simple { start: 'g'
                                                                                               , end: EndCondition::Immediate
                                                                                               , accum: vec![]
                                                                                               , token: "G".to_string()
                                                                                               }
                                                                               ]
                                                            }
                                            , Rule::Simple { start: 'h'
                                                           , end: EndCondition::Immediate
                                                           , accum: vec! []
                                                           , token: "H".to_string()
                                                           }

                                            ]
                          };

    let z = "abcyuiccaaacdhdefdeg";

    let t = z.char_indices();
    let o = t.fold( (vec![a, b, c, d], vec![], None), lex_next);

    println!("{:?}", o);
}
