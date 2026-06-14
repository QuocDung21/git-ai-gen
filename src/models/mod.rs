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
pub enum DiffLineKind {
    Added,
    Removed,
    Hunk,
    Header,
    Normal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiffViewLine {
    pub text: String,
    pub kind: DiffLineKind,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommitLogEntry {
    pub hash: String,
    pub short_hash: String,
    pub author: String,
    pub author_email: String,
    pub time: String,
    pub subject: String,
    pub parents: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageStat {
    pub name: String,
    pub bytes: u64,
    pub percentage: f64,
    pub color_code: (u8, u8, u8),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StashEntry {
    pub index: usize,
    pub branch: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RemoteEntry {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FeatureGroup {
    pub name: String,
    pub files: Vec<String>,
    pub file_count: usize,
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
pub struct GithubTreeEntry {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
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
    CommitDiff(String),   // commit hash
    MergeConfirm(String), // Branch name to merge into current branch
    NewBranchInput,       // Text input modal to create and checkout a new branch
    ThemeSelect,          // Interactive theme selection modal
    WorkspaceHistory,     // Workspace history selector modal
    ViewPrompt,
    ManualCommit,
    GitMenu,
    CommitTree,
    FeatureCommit,
    GithubDownloadUrlInput,
    GithubDownloadTree,
    GithubDownloadTargetInput,
    GithubQuickView { path: String, name: String },
    GithubBranchSelect,
    BranchDeleteConfirm(String),
    Settings,
    EditorSelect,
    WorkspacePathInput,
    ProjectLanguages,
    HandleTest,
}
