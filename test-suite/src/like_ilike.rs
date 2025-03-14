use {
    crate::*,
    bigdecimal::BigDecimal,
    gluesql_core::{
        data::Literal,
        error::{LiteralError, ValueError},
        prelude::Value::{self, Bool},
    },
    std::{borrow::Cow, str::FromStr},
};

test_case!(like_ilike, async move {
    test! {
        name: "basic usage - LIKE and ILIKE",
        sql: "
            VALUES
                ('abc' LIKE '%c'),
                ('abc' NOT LIKE '_c'),
                ('abc' LIKE '_b_'),
                ('HELLO' ILIKE '%el%'),
                ('HELLO' NOT ILIKE '_ELLE');
        ",
        expected: Ok(select!(column1 Bool; true; true; true; true; true))
    };

    run!(
        "
        CREATE TABLE Item (
            id INTEGER,
            name TEXT
        );
    "
    );
    run!(
        "
        INSERT INTO Item (id, name) VALUES
            (1,    'Amelia'),
            (2,      'Doll'),
            (3, 'Gascoigne'),
            (4,   'Gehrman'),
            (5,     'Maria');
    "
    );

    let test_cases = [
        (2, "SELECT name FROM Item WHERE name LIKE '_a%'"),
        (2, "SELECT name FROM Item WHERE name LIKE '%r%'"),
        (2, "SELECT name FROM Item WHERE SUBSTR(name, 1) LIKE '%a'"),
        (0, "SELECT name FROM Item WHERE 'name' LIKE SUBSTR('%a', 1)"),
        (
            2,
            "SELECT name FROM Item WHERE SUBSTR(name, 1) LIKE SUBSTR('%a', 1)",
        ),
        (
            2,
            "SELECT name FROM Item WHERE SUBSTR(name, 1) LIKE SUBSTR('%a', 1)",
        ),
        (
            2,
            "SELECT name FROM Item WHERE LOWER(name) LIKE SUBSTR('%a', 1)",
        ),
        (
            2,
            "SELECT name FROM Item WHERE SUBSTR(name, 1) LIKE '%' || LOWER('A')",
        ),
        (5, "SELECT name FROM Item WHERE name LIKE '%%'"),
        (0, "SELECT name FROM Item WHERE name LIKE 'g%'"),
        (2, "SELECT name FROM Item WHERE name ILIKE '_A%'"),
        (2, "SELECT name FROM Item WHERE name ILIKE 'g%'"),
        (5, "SELECT name FROM Item WHERE name ILIKE '%%'"),
        (1, "SELECT name FROM Item WHERE name NOT LIKE '%a%'"),
        (1, "SELECT name FROM Item WHERE name NOT ILIKE '%A%'"),
        (5, "SELECT name FROM Item WHERE 'ABC' LIKE '_B_'"),
        (5, "SELECT name FROM Item WHERE 'abc' ILIKE '_B_'"),
        (5, "SELECT name FROM Item WHERE 'ABC' ILIKE '_B_'"),
    ];

    for (num, sql) in test_cases {
        count!(num, sql);
    }

    let error_sqls = [
        (
            "SELECT name FROM Item WHERE 'ABC' LIKE 10",
            LiteralError::LikeOnNonString {
                base: format!("{:?}", Literal::Text(Cow::Owned("ABC".to_owned()))),
                pattern: format!(
                    "{:?}",
                    Literal::Number(Cow::Owned(BigDecimal::from_str("10").unwrap()))
                ),
                case_sensitive: true,
            }
            .into(),
        ),
        (
            "SELECT name FROM Item WHERE True ILIKE '_B_'",
            LiteralError::LikeOnNonString {
                base: format!("{:?}", Literal::Boolean(true)),
                pattern: format!("{:?}", Literal::Text(Cow::Owned("_B_".to_owned()))),
                case_sensitive: false,
            }
            .into(),
        ),
        (
            "SELECT name FROM Item WHERE name = 'Amelia' AND name LIKE 10",
            ValueError::LikeOnNonString {
                base: Value::Str("Amelia".to_owned()),
                pattern: Value::I64(10),
                case_sensitive: true,
            }
            .into(),
        ),
        (
            "SELECT name FROM Item WHERE name = 'Amelia' AND name ILIKE 10",
            ValueError::LikeOnNonString {
                base: Value::Str("Amelia".to_owned()),
                pattern: Value::I64(10),
                case_sensitive: false,
            }
            .into(),
        ),
    ];

    for (sql, error) in error_sqls {
        test!(sql, Err(error));
    }
});
