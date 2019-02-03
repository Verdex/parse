
// need nongreedy block comment capture

enum Token {
    Label { name : String },  
    Class { name : String, value : String },
    Complex { name : String, components : Vec<Token> },
}

enum EndCondition {
    Immediate,
    Until(char),
}

// instead of simple vs complex maybe ambigeous and ?
// char to T
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

fn lex_next() {

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



}
