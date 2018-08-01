use managers::Manager;
use shamrock::{Format, Identifiable, Layout};
use void::Void;

pub struct PlaceholderFormat;

impl Format for PlaceholderFormat {
    type Value = ();

    fn as_value(self) -> Self::Value {
        ()
    }
}

pub struct PlaceholderLayout;

impl Identifiable for PlaceholderLayout {
    fn id() -> &'static str {
        ""
    }
}

impl Layout for PlaceholderLayout {
    type Error = Void;
    type Format = PlaceholderFormat;
}