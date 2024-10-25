#[derive(Debug, PartialEq, Eq)]
pub enum Impact {
    Trivial,
    Ordinary,
    Notable,
    Remarkable,
    Extraordinary,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Project,
    CollaborationAndMembership,
    DesignAndDocumentation,
    CompanyBuilding,
    Learning,
    OutsideOfWork,
}
