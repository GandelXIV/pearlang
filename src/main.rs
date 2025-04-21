#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
enum TokenCategory {
    Text,
    Number,
    Keyword,
}

#[derive(Debug, PartialEq)]
struct Token {
    category: TokenCategory,
    name: String,
}

impl From<String> for Token {
    // only handles Number and Keyword, not Text
    fn from(s: String) -> Self {
        let cat = match s.chars().nth(0).expect("token should not be empty") {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => TokenCategory::Number,
            _ => TokenCategory::Keyword,
        };
        Self {
            category: cat,
            name: s,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Expression {
    cmd: Token,
    arg: Vec<Token>,
}

#[derive(Debug, PartialEq)]
struct Statement {
    cmd: Token,
    arg: Vec<Token>,
    block: Block,
}

impl Statement {
    fn new(cmd: String) -> Self {
        Self {
            cmd: Token {
                name: cmd,
                category: TokenCategory::Keyword,
            },
            arg: vec![],
            block: Block::default(),
        }
    }
    fn new_warg(cmd: String, arg: Vec<Token>) -> Self {
        Self {
            cmd: Token {
                name: cmd,
                category: TokenCategory::Keyword,
            },
            arg,
            block: Block::default(),
        }
    }
}

type Block = Vec<Statement>;

fn parse(raw: String) -> Block {
    let mut raw: String = raw.chars().rev().collect();
    raw.insert(0, '\n');

    let mut block = Block::default();
    let mut statement: Option<Statement> = None;
    let mut word: Option<String> = None;

    let mut isstring = false;
    let mut islaststring = false;
    while let Some(chr) = raw.pop() {
        match chr {
            '"' => {
                isstring = !isstring;
                if !isstring {
                    islaststring = true;
                }
            }
            x if isstring => match &mut word {
                Some(word) => word.push(x),
                None => word = Some(x.to_string()),
            },
            ' ' | '\n' => {
                if let Some(word) = word.take() {
                    match &mut statement {
                        Some(statement) => statement.arg.push(if islaststring {
                            Token {
                                name: word,
                                category: TokenCategory::Text,
                            }
                        } else {
                            Token::from(word)
                        }),
                        None => statement = Some(Statement::new(word)),
                    }
                }
                islaststring = false;
                // kinda ugly but who cares
                match chr {
                    '\n' => {
                        if let Some(mut statement) = statement.take() {
                            if let Some(word) = word.take() {
                                statement.arg.push(Token::from(word));
                            }
                            block.push(statement);
                        }
                    }
                    _ => {}
                }
            }
            x => match &mut word {
                Some(word) => word.push(x),
                None => word = Some(x.to_string()),
            },
        }
    }
    return block;
}

fn main() {
    println!("Hello, world!");
}
