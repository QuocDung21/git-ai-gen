#[derive(Clone, Debug, PartialEq)]
pub struct BranchEntry {
    pub name: String,
    pub is_remote: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChangedFile {
    pub status: String,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommitLogEntry {
    pub hash: String,
    pub author: String,
    pub time: String,
    pub subject: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StashEntry {
    pub index: usize,
    pub branch: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StashAction {
    Pop,
    Apply,
    Drop,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StashStep {
    List,
    Confirm(usize, StashAction),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GoStep {
    Confirm,      // Step 1: confirm
    Pushing,      // Step 2: pushing
    Done(String), // Step 3: result msg
}

#[derive(Clone, Debug, PartialEq)]
pub enum AmendStep {
    Edit,
    Pushing,
    Done(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActiveModal {
    None,
    RevertConfirm(String), // Path of the file to revert
    LanguageSelect,
    Help,
    GitLog,
    BranchSelect,
    DiffResult, // Display diff AI prompt copy preview
    GoConfirm,  // Multi-step commit & push modal
    StashList,
    RemoteInfo,
    AmendCommit,
    CommitDiff(String), // commit hash
    MergeConfirm(String), // Branch name to merge into current branch
}
