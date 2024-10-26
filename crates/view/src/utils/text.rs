pub trait Txt: PartialEq + Eq {
    fn text(&self) -> String;
}
