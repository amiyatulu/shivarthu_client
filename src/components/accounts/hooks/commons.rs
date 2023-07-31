#[derive(PartialEq, Clone)]
pub enum TransactionReturnKind {
    Error,
    Finalized,
    Processing,
    InBlock,
}

#[derive(PartialEq, Clone)]
pub struct TransactionReturn {
    pub kind: TransactionReturnKind,
    pub value: String,
    pub dispatch_error: Option<String>,
}
