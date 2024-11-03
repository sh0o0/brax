#[derive(Debug, PartialEq, Eq, strum::VariantArray, strum::EnumCount, Clone)]
pub enum Type {
    Project,
    CollaborationAndMembership,
    DesignAndDocumentation,
    CompanyBuilding,
    Learning,
    OutsideOfWork,
}

#[derive(Debug, PartialEq, Eq, strum::VariantArray, strum::EnumCount, Clone)]
pub enum Impact {
    Trivial,
    Ordinary,
    Notable,
    Remarkable,
    Extraordinary,
}
