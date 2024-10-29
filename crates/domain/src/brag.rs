#[derive(Debug, PartialEq, Eq, strum::VariantArray, Clone)]
pub enum Type {
    Project,
    CollaborationAndMembership,
    DesignAndDocumentation,
    CompanyBuilding,
    Learning,
    OutsideOfWork,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Impact {
    Trivial,
    Ordinary,
    Notable,
    Remarkable,
    Extraordinary,
}
