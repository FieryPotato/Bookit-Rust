pub(crate) struct Book {
    pub(crate) title: String,
    pub(crate) author: String,
}
impl Book {
    pub(crate) fn to_string(&self) -> String {
        format!("{} — {}", self.title, self.author)
    }
}
