use crate::text::text_selection::{selected_text_replaced, TextSelection};

///
#[derive(Copy, Clone, Debug)]
pub(crate) struct TextCursor {
    pub(crate) left_of_byte_index: usize,
}

///
pub(crate) fn left_character_removed(mut text: String, text_cursor: &TextCursor) -> String {
    // The text cursor is out of range.
    if text_cursor.left_of_byte_index == 0 || text_cursor.left_of_byte_index > text.len() {
        return text;
    }

    let mut left_char_boundary = text_cursor.left_of_byte_index - 1;
    let mut right_char_boundary = text_cursor.left_of_byte_index;

    // Find valid character boundaries.
    {
        while !text.is_char_boundary(right_char_boundary) && right_char_boundary > 0 {
            right_char_boundary -= 1;
        }

        while !text.is_char_boundary(left_char_boundary) && left_char_boundary > 0 {
            left_char_boundary -= 1;
        }
    }

    // Remove the character.
    text.replace_range(left_char_boundary..right_char_boundary, "");

    text
}

///
pub(crate) fn right_character_removed(mut text: String, text_cursor: &TextCursor) -> String {
    let mut left_char_boundary = text_cursor.left_of_byte_index;
    let mut right_char_boundary = text_cursor.left_of_byte_index + 1;

    // Find valid character boundaries.
    {
        while !text.is_char_boundary(right_char_boundary) && right_char_boundary > 0 {
            right_char_boundary += 1;
        }

        while !text.is_char_boundary(left_char_boundary) && left_char_boundary > 0 {
            left_char_boundary += 1;
        }
    }

    // The text cursor is out of range.
    if left_char_boundary >= text.len() {
        return text;
    }

    // The text cursor is out of range.
    if right_char_boundary >= text.len() {
        right_char_boundary = text.len();
    }

    // Remove the character.
    text.replace_range(left_char_boundary..right_char_boundary, "");

    text
}

///
pub(crate) fn text_inserted(
    text: String,
    text_cursor: &TextCursor,
    insertion_text: &str,
) -> String {
    selected_text_replaced(
        text,
        &TextSelection {
            left_of_byte_index_begin: text_cursor.left_of_byte_index,
            left_of_byte_index_end: text_cursor.left_of_byte_index,
        },
        insertion_text,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_character_removed() {
        // Out of range text cursor.
        {
            // Empty string.
            {
                assert_eq!(
                    left_character_removed(
                        "".to_string(),
                        &TextCursor {
                            left_of_byte_index: 0
                        },
                    ),
                    "".to_string()
                );
                assert_eq!(
                    left_character_removed(
                        "".to_string(),
                        &TextCursor {
                            left_of_byte_index: 10
                        },
                    ),
                    "".to_string()
                );
            }

            // Non-empty string.
            {
                assert_eq!(
                    left_character_removed(
                        "abc".to_string(),
                        &TextCursor {
                            left_of_byte_index: 0
                        },
                    ),
                    "abc".to_string()
                );
                assert_eq!(
                    left_character_removed(
                        "abc".to_string(),
                        &TextCursor {
                            left_of_byte_index: 10
                        },
                    ),
                    "abc".to_string()
                );
                assert_eq!(
                    left_character_removed(
                        "äbc".to_string(),
                        &TextCursor {
                            left_of_byte_index: 1
                        },
                    ),
                    "äbc".to_string()
                );
            }
        }

        // In range text cursor.
        {
            assert_eq!(
                left_character_removed(
                    "a".to_string(),
                    &TextCursor {
                        left_of_byte_index: 1
                    },
                ),
                "".to_string()
            );
            assert_eq!(
                left_character_removed(
                    "abc".to_string(),
                    &TextCursor {
                        left_of_byte_index: 2
                    },
                ),
                "ac".to_string()
            );
            assert_eq!(
                left_character_removed(
                    "abc".to_string(),
                    &TextCursor {
                        left_of_byte_index: 3
                    },
                ),
                "ab".to_string()
            );
            assert_eq!(
                left_character_removed(
                    "äbc".to_string(),
                    &TextCursor {
                        left_of_byte_index: 2
                    },
                ),
                "bc".to_string()
            );
        }
    }

    #[test]
    fn test_right_character_removed() {
        // Out of range text cursor.
        {
            // Empty string.
            {
                assert_eq!(
                    right_character_removed(
                        "".to_string(),
                        &TextCursor {
                            left_of_byte_index: 0
                        },
                    ),
                    "".to_string()
                );
                assert_eq!(
                    right_character_removed(
                        "".to_string(),
                        &TextCursor {
                            left_of_byte_index: 10
                        },
                    ),
                    "".to_string()
                );
            }

            // Non-empty string.
            {
                assert_eq!(
                    right_character_removed(
                        "abc".to_string(),
                        &TextCursor {
                            left_of_byte_index: 3
                        },
                    ),
                    "abc".to_string()
                );
                assert_eq!(
                    right_character_removed(
                        "abc".to_string(),
                        &TextCursor {
                            left_of_byte_index: 10
                        },
                    ),
                    "abc".to_string()
                );
                assert_eq!(
                    right_character_removed(
                        "äbc".to_string(),
                        &TextCursor {
                            left_of_byte_index: 1
                        },
                    ),
                    "bc".to_string()
                );
            }
        }

        // In range text cursor.
        {
            assert_eq!(
                right_character_removed(
                    "a".to_string(),
                    &TextCursor {
                        left_of_byte_index: 0
                    },
                ),
                "".to_string()
            );
            assert_eq!(
                right_character_removed(
                    "abc".to_string(),
                    &TextCursor {
                        left_of_byte_index: 1
                    },
                ),
                "ac".to_string()
            );
            assert_eq!(
                right_character_removed(
                    "abc".to_string(),
                    &TextCursor {
                        left_of_byte_index: 2
                    },
                ),
                "ab".to_string()
            );
            assert_eq!(
                right_character_removed(
                    "äbc".to_string(),
                    &TextCursor {
                        left_of_byte_index: 0
                    },
                ),
                "bc".to_string()
            );
            assert_eq!(
                right_character_removed(
                    "äbc".to_string(),
                    &TextCursor {
                        left_of_byte_index: 2
                    },
                ),
                "äc".to_string()
            );
        }
    }

    #[test]
    fn test_text_inserted() {
        // TODO
    }
}
