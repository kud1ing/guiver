///
#[derive(Copy, Clone, Debug)]
pub(crate) struct TextCursor {
    pub(crate) byte_index: usize,
}

///
pub(crate) fn left_character_removed(_text: &str, _text_cursor: TextCursor) -> String {
    todo!()
}

///
pub(crate) fn right_character_removed(_text: &str, _text_cursor: TextCursor) -> String {
    todo!()
}
