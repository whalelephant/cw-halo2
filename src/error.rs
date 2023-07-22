#[derive(Debug, PartialEq)]
pub enum CWError {
    FilesystemError,
    HexInvalid,
    InputParseError,
}
