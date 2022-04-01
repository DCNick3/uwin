#[derive(PartialEq, Debug)]
pub enum SignatureKind {
    Query,
    QueryOptional,
    // ResultValue,
    ResultVoid,
    // ReturnStruct,
    ReturnVoid,
    PreserveSig,
}
