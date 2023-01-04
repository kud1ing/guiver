use druid_shell::kurbo::Size;
use guiver::{GridColumnProperties, GridRowProperties};

///
#[derive(Clone, Debug)]
pub enum WidgetType {
    Center,
    Column,
    Expanded {
        flex_factor: u16,
    },
    Grid {
        column_properties: GridColumnProperties,
        row_properties: GridRowProperties,
    },
    Hyperlink(String),
    Padding,
    Placeholder {
        maximum_size: Size,
    },
    Row,
    SizedBox {
        desired_size: Size,
    },
    Text(String),
    TextButton(String),
    TextInput {
        text: String,
        width: f64,
    },
}
