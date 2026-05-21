#[allow(dead_code)]
pub struct Constant;

#[allow(dead_code)]
impl Constant {
    pub const PROMPT_EXPERT: &str = r#"Act as a Senior Software Engineer. Your task is to analyze the provided Git diff and generate a professional Git commit message.
    Strict Requirements:
    1. Output ONLY the raw commit message text. STRICTLY NO Markdown formatting (do not use ``` blocks), NO greetings, NO preambles, and NO concluding remarks.
    2. Subject Line: Must follow the Conventional Commits specification (e.g., feat, fix, chore, refactor, docs). Keep it strictly under 50 characters, use the imperative mood (e.g., "add", not "added" or "adds"), and do not end with a period.
    3. Structure: There must be exactly one empty line between the subject line and the body.
    4. Body: Write 2-3 concise sentences explaining WHAT the change is and WHY it was made. Do not explain HOW it was implemented (the diff already shows that). Wrap text at 72 characters per line. Be direct, no fluff.
    5. Language: Write the entire commit message in English.

    Diff:
    "#;
}
