mod derive {
    use arma_rs::{FromArma, FromArmaError, IntoArma, Value};

    fn sort_value_array(value: &mut Value) -> &Value {
        if let Value::Array(values) = value {
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        value
    }

    #[derive(Debug, PartialEq, Default)]
    enum ValueStringImpl {
        #[default]
        Even,
        Odd,
    }

    impl std::fmt::Display for ValueStringImpl {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Even => write!(f, "even"),
                Self::Odd => write!(f, "odd"),
            }
        }
    }

    impl std::str::FromStr for ValueStringImpl {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.chars().count() % 2 {
                0 => Ok(Self::Even),
                _ => Ok(Self::Odd),
            }
        }
    }

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
        fn derive() {
            #[derive(FromArma, IntoArma, Debug, PartialEq)]
            struct DeriveTest {
                first: String,
                second: bool,
            }

            let serialized = DeriveTest {
                first: "first".to_string(),
                second: true,
            };
            let deserialized = Value::Array(vec![
                Value::Array(vec![
                    Value::String("first".to_string()),
                    Value::String("first".to_string()),
                ]),
                Value::Array(vec![
                    Value::String("second".to_string()),
                    Value::Boolean(true),
                ]),
            ]);

            assert_eq!(sort_value_array(&mut serialized.to_arma()), &deserialized);
            assert_eq!(
                DeriveTest::from_arma(deserialized.to_string()),
                Ok(serialized)
            );
        }

        #[test]
        fn transparent() {
            #[derive(FromArma, IntoArma, Debug, PartialEq)]
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
        fn from_str_field() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest {
                #[arma(from_str)]
                expected: ValueStringImpl,
            }

            let input = Value::Array(vec![Value::Array(vec![
                Value::String("expected".to_string()),
                Value::String("odd".to_string()),
            ])]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    expected: ValueStringImpl::Odd,
                })
            );
        }

        #[test]
        fn to_string_field() {
            #[derive(IntoArma, Debug, PartialEq)]
            struct DeriveTest {
                #[arma(to_string)]
                expected: ValueStringImpl,
            }

            assert_eq!(
                DeriveTest {
                    expected: ValueStringImpl::Odd,
                }
                .to_arma(),
                Value::Array(vec![Value::Array(vec![
                    Value::String("expected".to_string()),
                    Value::String("odd".to_string()),
                ])])
            );
        }

        #[test]
        fn from_str_default_field() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest {
                #[arma(from_str, default)]
                expected: ValueStringImpl,
            }

            let input = Value::Array(vec![]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    expected: ValueStringImpl::default(),
                })
            );
        }

        #[test]
        fn default() {
            #[derive(FromArma, Default, Debug, PartialEq)]
            #[arma(default)]
            struct DeriveTest {
                first: String,
                second: bool,
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
                    ..DeriveTest::default()
                })
            );

            let input = Value::Array(vec![Value::Array(vec![
                Value::String("second".to_string()),
                Value::Boolean(true),
            ])]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    second: true,
                    ..DeriveTest::default()
                })
            );

            let input = Value::Array(vec![
                Value::Array(vec![
                    Value::String("first".to_string()),
                    Value::String("first".to_string()),
                ]),
                Value::Array(vec![
                    Value::String("second".to_string()),
                    Value::Boolean(true),
                ]),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    first: "first".to_string(),
                    second: true
                })
            );
        }

        #[test]
        fn default_field() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest {
                first: String,
                #[arma(default)]
                second: bool,
            }

            let input = Value::Array(vec![Value::Array(vec![
                Value::String("first".to_string()),
                Value::String("first".to_string()),
            ])]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    first: "first".to_string(),
                    second: false
                })
            );

            let input = Value::Array(vec![
                Value::Array(vec![
                    Value::String("first".to_string()),
                    Value::String("first".to_string()),
                ]),
                Value::Array(vec![
                    Value::String("second".to_string()),
                    Value::Boolean(true),
                ]),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    first: "first".to_string(),
                    second: true
                })
            );
        }

        #[test]
        fn default_field_precedence() {
            #[derive(FromArma, Debug, PartialEq)]
            #[arma(default)]
            struct DeriveTest {
                first: String,
                #[arma(default)]
                second: bool,
            }

            impl Default for DeriveTest {
                fn default() -> Self {
                    Self {
                        first: "first".to_string(),
                        second: true,
                    }
                }
            }

            let input = Value::Array(vec![]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    first: "first".to_string(),
                    second: false
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
                Err(FromArmaError::MissingField("_expected".to_string()))
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
                Err(FromArmaError::UnknownField("unknown".to_string()))
            );
        }

        #[test]
        fn error_duplicate() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest {
                _expected: String,
            }

            let input = Value::Array(vec![
                Value::Array(vec![
                    Value::String("_expected".to_string()),
                    Value::String("first".to_string()),
                ]),
                Value::Array(vec![
                    Value::String("_expected".to_string()),
                    Value::String("second".to_string()),
                ]),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Err(FromArmaError::DuplicateField("_expected".to_string()))
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
                Err(FromArmaError::UnknownField("unknown".to_string()))
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
                Err(FromArmaError::MissingField("_expected".to_string()))
            );
        }
    }

    mod tuple {
        use super::*;

        #[test]
        fn derive() {
            #[derive(FromArma, IntoArma, Debug, PartialEq)]
            struct DeriveTest(String, bool);

            let serialized = DeriveTest("first".to_string(), true);
            let deserialized = Value::Array(vec![
                Value::String("first".to_string()),
                Value::Boolean(true),
            ]);
            assert_eq!(serialized.to_arma(), deserialized);
            assert_eq!(
                DeriveTest::from_arma(deserialized.to_string()),
                Ok(serialized)
            );
        }

        #[test]
        fn from_string_field() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest(String, #[arma(from_str)] ValueStringImpl);

            let input = Value::Array(vec![
                Value::String("first".to_string()),
                Value::String("odd".to_string()),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), ValueStringImpl::Odd))
            );
        }

        #[test]
        fn to_string_field() {
            #[derive(IntoArma, Debug, PartialEq)]
            struct DeriveTest(String, #[arma(to_string)] ValueStringImpl);

            assert_eq!(
                DeriveTest("first".to_string(), ValueStringImpl::Odd).to_arma(),
                Value::Array(vec![
                    Value::String("first".to_string()),
                    Value::String("odd".to_string()),
                ])
            );
        }

        #[test]
        fn from_str_default_field() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest(String, #[arma(from_str, default)] ValueStringImpl);

            let input = Value::Array(vec![Value::String("first".to_string())]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), ValueStringImpl::default()))
            );
        }

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
        fn default_field_precedence() {
            #[derive(FromArma, Debug, PartialEq)]
            #[arma(default)]
            struct DeriveTest(String, #[arma(default)] bool);

            impl Default for DeriveTest {
                fn default() -> Self {
                    Self("first".to_string(), true)
                }
            }

            let input = Value::Array(vec![]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), false))
            );
        }

        #[test]
        fn error_length() {
            #[derive(FromArma, Default, Debug, PartialEq)]
            struct DeriveTest(String, String);

            let input = Value::Array(vec![
                Value::String("first".to_string()),
                Value::String("second".to_string()),
                Value::String("third".to_string()),
                Value::String("forth".to_string()),
            ]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Err(arma_rs::FromArmaError::InvalidLength {
                    expected: 2,
                    actual: 4,
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
                Err(arma_rs::FromArmaError::InvalidLength {
                    expected: 2,
                    actual: 0,
                })
            );
        }
    }

    mod newtype {
        use super::*;

        #[test]
        fn derive() {
            #[derive(FromArma, IntoArma, Debug, PartialEq)]
            struct DeriveTest(String);

            let serialized = DeriveTest("expected".to_string());
            let deserialized = Value::String("expected".to_string());
            assert_eq!(serialized.to_arma(), deserialized);
            assert_eq!(
                DeriveTest::from_arma(deserialized.to_string()),
                Ok(serialized)
            );
        }

        #[test]
        fn from_str() {
            #[derive(FromArma, Debug, PartialEq)]
            struct DeriveTest(#[arma(from_str)] ValueStringImpl);

            let input = Value::String("odd".to_string());
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest(ValueStringImpl::Odd))
            );
        }

        #[test]
        fn to_string() {
            #[derive(IntoArma, Debug, PartialEq)]
            struct DeriveTest(#[arma(to_string)] ValueStringImpl);

            assert_eq!(
                DeriveTest(ValueStringImpl::Odd).to_arma(),
                Value::String("odd".to_string()),
            );
        }
    }
}
