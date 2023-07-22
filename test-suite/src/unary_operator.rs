use {
    crate::*,
    gluesql_core::{
        error::{LiteralError, ValueError},
        prelude::{Payload, Value::*},
    },
};

test_case!(unary_operator, async move {
    let test_cases = [
        (
            "CREATE TABLE Test (v1 INT, v2 FLOAT, v3 TEXT, v4 INT, v5 INT, v6 INT8)",
            Ok(Payload::Create),
        ),
        (
            "INSERT INTO Test VALUES (10, 10.5, 'hello', -5, 1000, 20)",
            Ok(Payload::Insert(1)),
        ),
        (
            "SELECT -v1 as v1, -v2 as v2, v3, -v4 as v4, -v6 as v6 FROM Test",
            Ok(select_with_null!(
                v1      |   v2          |   v3                      |   v4      |  v6;
                I64(-10)    F64(-10.5)      Str("hello".to_owned())     I64(5)     I8(-20)
            )),
        ),
        (
            "SELECT -(-10) as v1, -(-10) as v2 FROM Test",
            Ok(select!(
                v1  |   v2
                I64 |   I64;
                10      10
            )),
        ),
        (
            "SELECT -v3 as v3 FROM Test",
            Err(ValueError::UnaryMinusOnNonNumeric.into()),
        ),
        (
            "SELECT -'errrr' as v1 FROM Test",
            Err(LiteralError::UnaryOperationOnNonNumeric.into()),
        ),
        (
            "SELECT +10 as v1, +(+10) as v2 FROM Test",
            Ok(select!(
                v1  |   v2
                I64 |   I64;
                10      10
            )),
        ),
        (
            "SELECT +v3 as v3 FROM Test",
            Err(ValueError::UnaryPlusOnNonNumeric.into()),
        ),
        (
            "SELECT +'errrr' as v1 FROM Test",
            Err(LiteralError::UnaryOperationOnNonNumeric.into()),
        ),
        (
            "SELECT v1! as v1 FROM Test",
            Ok(select!(
                v1
                I128;
                3628800
            )),
        ),
        (
            "SELECT 4! as v1 FROM Test",
            Ok(select!(
                v1
                I128;
                24
            )),
        ),
        (
            "SELECT v2! as v1 FROM Test",
            Err(ValueError::FactorialOnNonInteger.into()),
        ),
        (
            "SELECT v3! as v1 FROM Test",
            Err(ValueError::FactorialOnNonNumeric.into()),
        ),
        (
            "SELECT v4! as v4 FROM Test",
            Err(ValueError::FactorialOnNegativeNumeric.into()),
        ),
        (
            "SELECT v5! as v5 FROM Test",
            Err(ValueError::FactorialOverflow.into()),
        ),
        (
            "SELECT (-v6)! as v6 FROM Test",
            Err(ValueError::FactorialOnNegativeNumeric.into()),
        ),
        (
            "SELECT (v6 * 2)! as v6 FROM Test",
            Err(ValueError::FactorialOverflow.into()),
        ),
        (
            "SELECT (-5)! as v4 FROM Test",
            Err(ValueError::FactorialOnNegativeNumeric.into()),
        ),
        (
            "SELECT (5.5)! as v4 FROM Test",
            Err(ValueError::FactorialOnNonInteger.into()),
        ),
        (
            "SELECT 'errrr'! as v1 FROM Test",
            Err(ValueError::FactorialOnNonNumeric.into()),
        ),
        (
            "SELECT 1000! as v4 FROM Test",
            Err(ValueError::FactorialOverflow.into()),
        ),
        (
            "SELECT ~(CAST(1 AS UINT8)) as v1 FROM Test",
            Ok(select!(
                v1
                U8;
                254
            )),
        ),
        (
            "SELECT ~(CAST(1 AS UINT16)) as v1 FROM Test",
            Ok(select!(
                v1
                U16;
                65534
            )),
        ),
        (
            "SELECT ~(CAST(1 AS UINT32)) as v1 FROM Test",
            Ok(select!(
                v1
                U32;
                4294967294
            )),
        ),
        (
            "SELECT ~(CAST(1 AS UINT64)) as v1 FROM Test",
            Ok(select!(
                v1
                U64;
                18446744073709551614
            )),
        ),
        (
            "SELECT ~(CAST(1 AS UINT128)) as v1 FROM Test",
            Ok(select!(
                v1
                U128;
                340282366920938463463374607431768211454
            )),
        ),
        (
            "SELECT ~(CAST(1 AS INT8)) as v1 FROM Test",
            Ok(select!(
                v1
                I8;
                -2
            )),
        ),
        (
            "SELECT ~(CAST(1 AS INT16)) as v1 FROM Test",
            Ok(select!(
                v1
                I16;
                -2
            )),
        ),
        (
            "SELECT ~(CAST(1 AS INT32)) as v1 FROM Test",
            Ok(select!(
                v1
                I32;
                -2
            )),
        ),
        (
            "SELECT ~1 as v1 FROM Test",
            Ok(select!(
                v1
                I64;
                -2
            )),
        ),
        (
            "SELECT ~(CAST(1 AS INT128)) as v1 FROM Test",
            Ok(select!(
                v1
                I128;
                -2
            )),
        ),
        (
            "SELECT Null as v1 FROM Test",
            Ok(select_with_null!(
                v1;
                Null
            )),
        ),
        (
            "SELECT ~(5.5) as v4 FROM Test",
            Err(ValueError::UnaryBitNotOnNonInteger.into()),
        ),
        (
            "SELECT ~(CAST(5.5 AS FLOAT32)) as v4 FROM Test",
            Err(ValueError::UnaryBitNotOnNonInteger.into()),
        ),
        (
            "SELECT ~'error' as v1 FROM Test",
            Err(ValueError::UnaryBitNotOnNonNumeric.into()),
        ),
    ];

    for (sql, expected) in test_cases {
        test!(sql, expected);
    }
});
