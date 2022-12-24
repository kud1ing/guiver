///
#[derive(Copy, Clone, Debug)]
pub(crate) struct TextSelection {
    pub(crate) left_of_byte_index_begin: usize,
    pub(crate) left_of_byte_index_end: usize,
}

// =================================================================================================

/// Returns normalized indices, that is sorted indices where the end index is within range of the
/// given string.
fn normalized_indices(text: &str, text_selection: &TextSelection) -> (usize, usize) {
    // Make sure that the indices are ordered.
    let (mut left_of_byte_index_begin, mut left_of_byte_index_end) =
        if text_selection.left_of_byte_index_begin > text_selection.left_of_byte_index_end {
            (
                text_selection.left_of_byte_index_end,
                text_selection.left_of_byte_index_begin,
            )
        } else {
            (
                text_selection.left_of_byte_index_begin,
                text_selection.left_of_byte_index_end,
            )
        };

    // Find valid character boundaries.
    {
        while !text.is_char_boundary(left_of_byte_index_begin) && left_of_byte_index_begin > 0 {
            left_of_byte_index_begin -= 1;
        }

        while !text.is_char_boundary(left_of_byte_index_end) && left_of_byte_index_end > 0 {
            left_of_byte_index_end -= 1;
        }
    }

    // Trim the selection.
    if left_of_byte_index_end > text.len() {
        left_of_byte_index_end = text.len();
    }

    // TODO: use `str::is_char_boundary()`

    (left_of_byte_index_begin, left_of_byte_index_end)
}

/// Tries to return a sub-string as defined by the given text selection.
pub(crate) fn selected_text<'a>(text: &'a str, text_selection: &TextSelection) -> &'a str {
    // Normalize the indices.
    let (left_of_byte_index_begin, left_of_byte_index_end) =
        normalized_indices(text, text_selection);

    &text[left_of_byte_index_begin..left_of_byte_index_end]
}

/// Tries to replace a sub-string as defined by the given text selection.
pub(crate) fn selected_text_replaced(
    mut text: String,
    text_selection: &TextSelection,
    replacement_text: &str,
) -> String {
    // Normalize the indices.
    let (left_of_byte_index_begin, left_of_byte_index_end) =
        normalized_indices(&text, text_selection);

    // The selection is out of range.
    if left_of_byte_index_begin > text.len() {
        return text;
    }

    // Replace the text selection.
    text.replace_range(
        left_of_byte_index_begin..left_of_byte_index_end,
        replacement_text,
    );

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selected_text() {
        // Out of range text selection.
        {
            // Empty string.
            {
                assert_eq!(
                    selected_text(
                        "",
                        &TextSelection {
                            left_of_byte_index_begin: 10,
                            left_of_byte_index_end: 20,
                        }
                    ),
                    ""
                );
                assert_eq!(
                    selected_text(
                        "",
                        &TextSelection {
                            left_of_byte_index_begin: 20,
                            left_of_byte_index_end: 10,
                        }
                    ),
                    ""
                );
            }

            // Non-empty string.
            {
                assert_eq!(
                    selected_text(
                        "abc",
                        &TextSelection {
                            left_of_byte_index_begin: 10,
                            left_of_byte_index_end: 20,
                        }
                    ),
                    ""
                );
                assert_eq!(
                    selected_text(
                        "abc",
                        &TextSelection {
                            left_of_byte_index_begin: 20,
                            left_of_byte_index_end: 10,
                        }
                    ),
                    ""
                );
            }
        }

        // Valid text selection.
        {
            // Empty result.
            {
                assert_eq!(
                    selected_text(
                        "",
                        &TextSelection {
                            left_of_byte_index_begin: 0,
                            left_of_byte_index_end: 0,
                        }
                    ),
                    ""
                );
                assert_eq!(
                    selected_text(
                        "",
                        &TextSelection {
                            left_of_byte_index_begin: 0,
                            left_of_byte_index_end: 10,
                        }
                    ),
                    ""
                );

                assert_eq!(
                    selected_text(
                        "abc",
                        &TextSelection {
                            left_of_byte_index_begin: 1,
                            left_of_byte_index_end: 1,
                        }
                    ),
                    ""
                );
            }

            // Non-empty result.
            {
                {
                    assert_eq!(
                        selected_text(
                            "abc",
                            &TextSelection {
                                left_of_byte_index_begin: 0,
                                left_of_byte_index_end: 1,
                            }
                        ),
                        "a"
                    );
                    assert_eq!(
                        selected_text(
                            "abc",
                            &TextSelection {
                                left_of_byte_index_begin: 1,
                                left_of_byte_index_end: 0,
                            }
                        ),
                        "a"
                    );
                }

                {
                    assert_eq!(
                        selected_text(
                            "abc",
                            &TextSelection {
                                left_of_byte_index_begin: 0,
                                left_of_byte_index_end: 2,
                            }
                        ),
                        "ab"
                    );
                    assert_eq!(
                        selected_text(
                            "abc",
                            &TextSelection {
                                left_of_byte_index_begin: 2,
                                left_of_byte_index_end: 0,
                            }
                        ),
                        "ab"
                    );
                }

                {
                    assert_eq!(
                        selected_text(
                            "abc",
                            &TextSelection {
                                left_of_byte_index_begin: 0,
                                left_of_byte_index_end: 3,
                            }
                        ),
                        "abc"
                    );
                    assert_eq!(
                        selected_text(
                            "abc",
                            &TextSelection {
                                left_of_byte_index_begin: 3,
                                left_of_byte_index_end: 0,
                            }
                        ),
                        "abc"
                    );
                }

                {
                    assert_eq!(
                        selected_text(
                            "abc",
                            &TextSelection {
                                left_of_byte_index_begin: 0,
                                left_of_byte_index_end: 10,
                            }
                        ),
                        "abc"
                    );
                    assert_eq!(
                        selected_text(
                            "abc",
                            &TextSelection {
                                left_of_byte_index_begin: 10,
                                left_of_byte_index_end: 0,
                            }
                        ),
                        "abc"
                    );
                }

                assert_eq!(
                    selected_text(
                        "abc",
                        &TextSelection {
                            left_of_byte_index_begin: 1,
                            left_of_byte_index_end: 2,
                        }
                    ),
                    "b"
                );
            }
        }
    }

    #[test]
    fn test_selected_text_replaced() {
        // Out of range text selection.
        {
            // Empty destination string.
            {
                assert_eq!(
                    selected_text_replaced(
                        "".to_string(),
                        &TextSelection {
                            left_of_byte_index_begin: 10,
                            left_of_byte_index_end: 20,
                        },
                        "foo"
                    ),
                    "".to_string()
                );
                assert_eq!(
                    selected_text_replaced(
                        "".to_string(),
                        &TextSelection {
                            left_of_byte_index_begin: 20,
                            left_of_byte_index_end: 10,
                        },
                        "foo"
                    ),
                    "".to_string()
                );
            }

            // Non-empty destination string.
            {
                assert_eq!(
                    selected_text_replaced(
                        "abc".to_string(),
                        &TextSelection {
                            left_of_byte_index_begin: 10,
                            left_of_byte_index_end: 20,
                        },
                        "foo"
                    ),
                    "abc".to_string()
                );
                assert_eq!(
                    selected_text_replaced(
                        "abc".to_string(),
                        &TextSelection {
                            left_of_byte_index_begin: 20,
                            left_of_byte_index_end: 10,
                        },
                        "foo"
                    ),
                    "abc".to_string()
                );
            }
        }

        // Valid text selection.
        {
            // Empty destination string.
            {
                // Empty source string.
                {
                    assert_eq!(
                        selected_text_replaced(
                            "".to_string(),
                            &TextSelection {
                                left_of_byte_index_begin: 0,
                                left_of_byte_index_end: 0,
                            },
                            ""
                        ),
                        "".to_string()
                    );
                }

                // Non-empty source string.
                {
                    assert_eq!(
                        selected_text_replaced(
                            "".to_string(),
                            &TextSelection {
                                left_of_byte_index_begin: 0,
                                left_of_byte_index_end: 0,
                            },
                            "foo"
                        ),
                        "foo".to_string()
                    );
                    assert_eq!(
                        selected_text_replaced(
                            "".to_string(),
                            &TextSelection {
                                left_of_byte_index_begin: 0,
                                left_of_byte_index_end: 10,
                            },
                            "foo"
                        ),
                        "foo".to_string()
                    );
                }
            }

            // Non-empty destination result.
            {
                // Empty source string.
                {
                    assert_eq!(
                        selected_text_replaced(
                            "abc".to_string(),
                            &TextSelection {
                                left_of_byte_index_begin: 1,
                                left_of_byte_index_end: 1,
                            },
                            ""
                        ),
                        "abc".to_string()
                    );
                    assert_eq!(
                        selected_text_replaced(
                            "abc".to_string(),
                            &TextSelection {
                                left_of_byte_index_begin: 0,
                                left_of_byte_index_end: 2,
                            },
                            ""
                        ),
                        "c".to_string()
                    );
                    assert_eq!(
                        selected_text_replaced(
                            "abc".to_string(),
                            &TextSelection {
                                left_of_byte_index_begin: 0,
                                left_of_byte_index_end: 3,
                            },
                            ""
                        ),
                        "".to_string()
                    );
                }

                // Non-empty source string.
                {
                    assert_eq!(
                        selected_text_replaced(
                            "abc".to_string(),
                            &TextSelection {
                                left_of_byte_index_begin: 0,
                                left_of_byte_index_end: 1,
                            },
                            "foo"
                        ),
                        "foobc".to_string()
                    );
                }
            }
        }
    }
}
