///
#[derive(Copy, Clone, Debug)]
pub(crate) struct TextSelection {
    pub(crate) byte_index_begin: usize,
    pub(crate) byte_index_end: usize,
}

///
pub(crate) fn selected_text(_text: &str, _text_selection: TextSelection) -> &str {
    todo!()
}

///
pub(crate) fn selected_text_removed(_text: &str, _text_selection: &TextSelection) -> String {
    todo!()
}

///
pub(crate) fn selected_text_replaced(
    _text: &str,
    _text_selection: &TextSelection,
    _replacement_text: &str,
) -> String {
    todo!()
}

///
pub(crate) fn text_inserted(
    _text: &str,
    _text_selection: &TextSelection,
    _insertion_text: &str,
) -> String {
    todo!()
}
