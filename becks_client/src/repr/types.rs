pub trait Repr: Sized + 'static {
    fn repr(&self) -> &'static str;

    fn all() -> &'static [Self];

    fn all_repred() -> Vec<&'static str> {
        Self::all().iter().map(|value| value.repr()).collect()
    }

    fn unrepr(s: &str) -> &'static Self {
        for value in Self::all().iter() {
            if value.repr() == s {
                return value;
            }
        }
        Self::all().first().expect("all should not be empty")
    }
}

pub trait ClientRepr: Repr {
    fn from_server(s: &str) -> Self;
    fn to_server(&self) -> String;
}
