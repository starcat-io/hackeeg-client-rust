#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode {
    Text,
    JsonLines,
    MsgPack,
    Unknown,
}
