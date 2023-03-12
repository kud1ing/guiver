/// The location of widgets.
#[derive(Clone, Debug)]
pub enum WidgetsLocation {
    Column(usize),
    FirstColumn,
    FirstRow,
    LastColumn,
    LastRow,
    Row(usize),
}
