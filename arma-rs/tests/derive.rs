mod derive {
    use arma_rs::{FromArma, FromArmaError, IntoArma, Value};

    #[derive(Debug, PartialEq, Default)]
    enum ValueNoImpl {
        #[default]
        Even,
        Odd,
    }

    impl std::fmt::Display for ValueNoImpl {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Even => write!(f, "even"),
                Self::Odd => write!(f, "odd"),
            }
        }
    }

    impl std::str::FromStr for ValueNoImpl {
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
        fn stringify_field() {
            #[derive(IntoArma, FromArma, Debug, PartialEq)]
            struct DeriveTest {
                #[arma(stringify)]
                expected: ValueNoImpl,
            }

            let serialized = DeriveTest {
                expected: ValueNoImpl::Odd,
            };
            let deserialized = Value::Array(vec![Value::Array(vec![
                Value::String("expected".to_string()),
                Value::String("odd".to_string()),
            ])]);
            assert_eq!(serialized.to_arma(), deserialized);
            assert_eq!(
                DeriveTest::from_arma(deserialized.to_string()),
                Ok(serialized)
            );
        }

        #[test]
        fn stringify_default_field() {
            #[derive(IntoArma, FromArma, Debug, PartialEq)]
            struct DeriveTest {
                #[arma(stringify, default)]
                expected: ValueNoImpl,
            }

            let serialized = DeriveTest {
                expected: ValueNoImpl::default(),
            };
            assert_eq!(
                serialized.to_arma(),
                Value::Array(vec![Value::Array(vec![
                    Value::String("expected".to_string()),
                    Value::String(ValueNoImpl::default().to_string()),
                ])])
            );

            let input = Value::Array(vec![]);
            assert_eq!(DeriveTest::from_arma(input.to_string()), Ok(serialized));
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
        fn default_field_precedence() {
            #[derive(FromArma, Debug, PartialEq)]
            #[arma(default)]
            struct DeriveTest {
                first: String,
                #[arma(default)]
                second: String,
            }

            impl Default for DeriveTest {
                fn default() -> Self {
                    Self {
                        first: "first".to_string(),
                        second: "second".to_string(),
                    }
                }
            }

            let input = Value::Array(vec![]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest {
                    first: "first".to_string(),
                    second: Default::default()
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
        fn stringify_field() {
            #[derive(IntoArma, FromArma, Debug, PartialEq)]
            struct DeriveTest(String, #[arma(stringify)] ValueNoImpl);

            let serialized = DeriveTest("first".to_string(), ValueNoImpl::Odd);
            let deserialized = Value::Array(vec![
                Value::String("first".to_string()),
                Value::String("odd".to_string()),
            ]);
            assert_eq!(serialized.to_arma(), deserialized);
            assert_eq!(
                DeriveTest::from_arma(deserialized.to_string()),
                Ok(serialized)
            );
        }

        #[test]
        fn stringify_default_field() {
            #[derive(IntoArma, FromArma, Debug, PartialEq)]
            struct DeriveTest(String, #[arma(stringify, default)] ValueNoImpl);

            let serialized = DeriveTest("first".to_string(), ValueNoImpl::default());
            assert_eq!(
                serialized.to_arma(),
                Value::Array(vec![
                    Value::String("first".to_string()),
                    Value::String(ValueNoImpl::default().to_string()),
                ])
            );

            let input = Value::Array(vec![Value::String("first".to_string())]);
            assert_eq!(DeriveTest::from_arma(input.to_string()), Ok(serialized));
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
                    Self("first".to_string(), !bool::default())
                }
            }

            let input = Value::Array(vec![]);
            assert_eq!(
                DeriveTest::from_arma(input.to_string()),
                Ok(DeriveTest("first".to_string(), Default::default()))
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
                Err(arma_rs::FromArmaError::InvalidLength {
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
        fn transparent() {
            #[derive(IntoArma, FromArma, Debug, PartialEq)]
            #[arma(transparent)]
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
        fn stringify() {
            #[derive(IntoArma, FromArma, Debug, PartialEq)]
            struct DeriveTest(#[arma(stringify)] u64);

            let serialized = DeriveTest(42);
            let deserialized = "42".to_string();
            assert_eq!(serialized.to_arma(), Value::String(deserialized.clone()));
            assert_eq!(DeriveTest::from_arma(deserialized), Ok(serialized));
        }
    }
}
