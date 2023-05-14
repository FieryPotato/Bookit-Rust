
#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub(crate) struct Book {
    pub(crate) title: String,
    pub(crate) author: String,
}

impl Book {
    pub(crate) fn file_name(&self) -> String {
        let path_title: String = self.title.replace(" ","_");
        let path_author: String = self.author.replace(" ","_");
        format!("{}-{}", path_title, path_author)
    }
}

impl Book {
    pub(crate) fn to_string(&self) -> String {
        format!("{} — {}", self.title, self.author)
    }
}
impl PartialOrd for Book {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let title_cmp = self.title.cmp(&other.title);
        let author_cmp = self.author.cmp(&other.author);
        if author_cmp != std::cmp::Ordering::Equal {
            Some(author_cmp)
        } else {
            Some(title_cmp)
        }
    }
}
