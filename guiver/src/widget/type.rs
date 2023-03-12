use crate::{GridColumnProperties, GridRowProperties, Size};

/// The type of a widget to construct.
#[derive(Clone, Debug)]
pub enum WidgetType {
    Hyperlink(String),
    LayoutCenter,
    LayoutColumn,
    LayoutExpanded {
        flex_factor: u16,
    },
    LayoutGrid {
        column_properties: GridColumnProperties,
        row_properties: GridRowProperties,
    },
    LayoutPadding,
    Placeholder {
        maximum_size: Size,
    },
    LayoutRow,
    LayoutSizedBox {
        desired_size: Size,
    },
    Text(String),
    TextButton(String),
    TextInput {
        text: String,
        width: f64,
    },
}
