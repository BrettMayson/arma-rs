mod derive {
    use arma_rs::{FromArma, FromArmaError, IntoArma, Value};
    use arma_rs_proc::{FromArma, IntoArma};

    #[test]
    #[cfg(not(miri))]
    fn compile() {
        let tests = trybuild::TestCases::new();
        tests.compile_fail("tests/derive/*fail*.rs");
        tests.pass("tests/derive/*pass*.rs");
    }

    mod map {
        use super::*;

        #[test]
        fn transparent() {
            #[derive(IntoArma, FromArma, Debug, PartialEq)]
            #[arma(transparent)]
            struct DeriveTest {
                expected: String,
            }

            let serialized = DeriveTest {
                expected: "expected".to_string(),
            };
            let deserialized = Value::String("expected".to_string());
            assert_eq!(serialized.to_arma(), deserialized);
            assert_eq!(
                DeriveTest::from_arma(deserialized.to_string()),
                Ok(serialized)
            );
        }

        #[test]
        fn default() {
            #[derive(FromArma, Default, Debug, PartialEq)]
            #[arma(default)]
            struct DeriveTest {
                first: String,
                second: String,
            }

            let input = Value::Array(vec![]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest::default())
            );

            let input = Value::Array(vec![Value::Array(vec![
                Value::String("first".to_string()),
                Value::String("first".to_string()),
            ])]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    first: "first".to_string(),
                    ..Default::default()
                })
            );

            let input = Value::Array(vec![Value::Array(vec![
                Value::String("second".to_string()),
                Value::String("second".to_string()),
            ])]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    second: "second".to_string(),
                    ..Default::default()
                })
            );

            let input = Value::Array(vec![
                Value::Array(vec![
                    Value::String("first".to_string()),
                    Value::String("first".to_string()),
                ]),
                Value::Array(vec![
                    Value::String("second".to_string()),
                    Value::String("second".to_string()),
                ]),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    first: "first".to_string(),
                    second: "second".to_string()
                })
            );
        }

        #[test]
        fn default_field() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest {
                first: String,
                #[arma(default)]
                second: String,
            }

            let input = Value::Array(vec![Value::Array(vec![
                Value::String("first".to_string()),
                Value::String("first".to_string()),
            ])]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    first: "first".to_string(),
                    second: Default::default()
                })
            );

            let input = Value::Array(vec![
                Value::Array(vec![
                    Value::String("first".to_string()),
                    Value::String("first".to_string()),
                ]),
                Value::Array(vec![
                    Value::String("second".to_string()),
                    Value::String("second".to_string()),
                ]),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    first: "first".to_string(),
                    second: "second".to_string()
                })
            );
        }

        #[test]
        fn error_missing() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest {
                _expected: String,
            }

            let input = Value::Array(vec![]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Err(FromArmaError::MapMissingField("_expected".to_string()))
            );
        }

        #[test]
        fn error_unknown() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest {
                _expected: String,
            }

            let input = Value::Array(vec![
                Value::Array(vec![
                    Value::String("_expected".to_string()),
                    Value::String("_expected".to_string()),
                ]),
                Value::Array(vec![
                    Value::String("unknown".to_string()),
                    Value::String("unknown".to_string()),
                ]),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Err(FromArmaError::MapUnknownField("unknown".to_string()))
            );
        }

        #[test]
        fn default_error_unknown() {
            #[derive(FromArma, Default, Debug, PartialEq)]
            #[arma(default)]
            struct DeriveTest {
                _expected: String,
            }

            let input = Value::Array(vec![Value::Array(vec![
                Value::String("unknown".to_string()),
                Value::String("unknown".to_string()),
            ])]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Err(FromArmaError::MapUnknownField("unknown".to_string()))
            );
        }

        #[test]
        fn default_field_error_missing() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest {
                _expected: String,
                #[arma(default)]
                _default: String,
            }

            let input = Value::Array(vec![]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Err(FromArmaError::MapMissingField("_expected".to_string()))
            );
        }
    }

    mod tuple {
        use super::*;

        #[test]
        fn default() {
            #[derive(FromArma, Default, Debug, PartialEq)]
            #[arma(default)]
            struct DeriveTest(String, bool);

            let input = Value::Array(vec![]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest::default())
            );

            let input = Value::Array(vec![Value::String("first".to_string())]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), false))
            );

            let input = Value::Array(vec![
                Value::String("first".to_string()),
                Value::Boolean(true),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), true))
            );
        }

        #[test]
        fn default_field() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest(String, #[arma(default)] bool);

            let input = Value::Array(vec![Value::String("first".to_string())]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), false))
            );

            let input = Value::Array(vec![
                Value::String("first".to_string()),
                Value::Boolean(true),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), true))
            );
        }

        #[test]
        fn default_multi_field() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest(String, #[arma(default)] bool, #[arma(default)] bool);

            let input = Value::Array(vec![Value::String("first".to_string())]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), false, false))
            );

            let input = Value::Array(vec![
                Value::String("first".to_string()),
                Value::Boolean(true),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), true, false))
            );

            let input = Value::Array(vec![
                Value::String("first".to_string()),
                Value::Boolean(true),
                Value::Boolean(true),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), true, true))
            );
        }

        #[test]
        fn default_error() {
            #[derive(FromArma, Default, Debug, PartialEq)]
            #[arma(default)]
            struct DeriveTest(String, String);

            let input = Value::Array(vec![
                Value::String("first".to_string()),
                Value::String("second".to_string()),
                Value::String("third".to_string()),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Err(arma_rs::FromArmaError::SizeMismatch {
                    expected: 2,
                    actual: 3,
                })
            );
        }

        #[test]
        fn default_field_error() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest(String, #[arma(default)] String);

            let input = Value::Array(vec![]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Err(arma_rs::FromArmaError::SizeMismatch {
                    expected: 2,
                    actual: 0,
                })
            );
        }
    }
}
