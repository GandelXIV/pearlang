use crate::*;

#[test]
fn test_line_and_spaces_stmnt() {
    assert_eq!(
        parse("echo abc    def\n".to_string()),
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

#[test]
fn test_multiline() {
    assert_eq!(
        parse(
            "
echo 1\n
echo 2\n
exit"
                .into()
        ),
        vec![
            Statement::new_warg("echo".into(), vec![Token::from("1".to_string())]),
            Statement::new_warg("echo".into(), vec![Token::from("2".to_string())]),
            Statement::new("exit".into())
        ]
    );
}

#[test]
fn test_strings_and_tokencats() {
    assert_eq!(
        parse("echo \"Hello World!\" \"123\" 456".into()),
        vec![Statement {
            cmd: Token {
                category: TokenCategory::Keyword,
                name: "echo".to_string()
            },
            arg: vec![
                Token {
                    category: TokenCategory::Text,
                    name: "Hello World!".to_string()
                },
                Token {
                    category: TokenCategory::Text,
                    name: "123".to_string()
                },
                Token {
                    category: TokenCategory::Number,
                    name: "456".to_string()
                }
            ],
            block: vec![]
        }]
    )
}
