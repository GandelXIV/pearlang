use std;

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
    fn from(s: String) -> Self {
        let cat = match s.chars().nth(0).expect("token should not be empty") {
            '\"' => TokenCategory::Text,
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
            cmd: Token::from(cmd.to_string()),
            arg: vec![],
            block: Block::default(),
        }
    }
}

type Block = Vec<Statement>;

fn parse(raw: String) -> Block {
    let mut raw: String = raw.chars().rev().collect();
    raw.push('\n');

    let mut block = Block::default();
    let mut statement: Option<Statement> = None;
    let mut word: Option<String> = None;
    while let Some(chr) = raw.pop() {
        match chr {
            ' ' => {
                println!("{:?} {:?}", statement, word);
                if let Some(word) = word.take() {
                    match &mut statement {
                        Some(statement) => {
                            statement.arg.push(Token::from(word));
                        }
                        None => statement = Some(Statement::new(word)),
                    }
                }
            }
            '\n' => {
                if let Some(mut statement) = statement.take() {
                    if let Some(word) = word.take() {
                        statement.arg.push(Token::from(word));
                    }
                    println!("{:?} {:?}", statement, word);

                    block.push(statement);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        assert_eq!(
            parse("echo abc def\n".to_string()),
            vec![Statement {
                cmd: Token {
                    category: TokenCategory::Keyword,
                    name: "echo".to_string()
                },
                arg: vec![
                    Token {
                        category: TokenCategory::Keyword,
                        name: "abc".to_string()
                    },
                    Token {
                        category: TokenCategory::Keyword,
                        name: "def".to_string()
                    }
                ],
                block: vec![]
            }]
        );
    }
}

fn main() {
    parse("echo abc def\n".to_string());
    println!("Hello, world!");
}
