use crate::model::Progress;

pub fn load_curriculum() -> Progress {
    const DATA: &str = include_str!("../data/curriculum.json");
    serde_json::from_str(DATA).expect("curriculum.json is valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Book, Status};

    #[test]
    fn load_curriculum_returns_chapters() {
        let curriculum = load_curriculum();
        assert_eq!(curriculum.chapters.len(), 32);
    }

    #[test]
    fn all_chapters_start_not_started() {
        let curriculum = load_curriculum();
        for ch in &curriculum.chapters {
            assert_eq!(ch.status(), Status::NotStarted);
        }
    }

    #[test]
    fn curriculum_has_both_books() {
        let curriculum = load_curriculum();
        let has_trpl = curriculum.chapters.iter().any(|ch| ch.book == Book::Trpl);
        let has_rfr = curriculum
            .chapters
            .iter()
            .any(|ch| ch.book == Book::RustForRustaceans);
        assert!(has_trpl);
        assert!(has_rfr);
    }

    #[test]
    fn every_chapter_has_at_least_one_exercise() {
        let curriculum = load_curriculum();
        for ch in &curriculum.chapters {
            assert!(
                !ch.exercises.is_empty(),
                "Chapter {} ({}) has no exercises",
                ch.number,
                ch.title
            );
        }
    }
}
