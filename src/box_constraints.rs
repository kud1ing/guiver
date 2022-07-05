///´
pub struct BoxConstraints {
    pub maximum_height: f64,
    pub maximum_width: f64,
    pub minimum_height: f64,
    pub minimum_width: f64,
}

impl BoxConstraints {
    ///
    pub fn new(
        minimum_width: f64,
        maximum_width: f64,
        minimum_height: f64,
        maximum_height: f64,
    ) -> Self {
        BoxConstraints {
            maximum_height,
            maximum_width,
            minimum_height,
            minimum_width,
        }
    }
}
