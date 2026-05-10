use serde::{Deserialize, Serialize};

/// Which textbook a chapter belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Book {
    /// The Rust Programming Language
    Trpl,
    /// Rust for Rustaceans
    RustForRustaceans,
}

/// The kind of work an exercise asks for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExerciseType {
    Reading,
    Coding,
    Quiz,
}

/// Completion state of a chapter or exercise.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    InProgress,
    Complete,
}

/// A single exercise within a chapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
    pub description: String,
    pub exercise_type: ExerciseType,
    pub status: Status,
}

/// A chapter from one of the tracked textbooks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chapter {
    pub book: Book,
    pub number: u32,
    pub title: String,
    pub exercises: Vec<Exercise>,
}

/// Top-level progress tracker across all chapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress {
    pub chapters: Vec<Chapter>,
}

// ── Display implementations ─────────────────────────────────────────

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Book::Trpl => write!(f, "The Rust Programming Language"),
            Book::RustForRustaceans => write!(f, "Rust for Rustaceans"),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::NotStarted => write!(f, "Not Started"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Complete => write!(f, "Complete"),
        }
    }
}

impl std::fmt::Display for ExerciseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExerciseType::Reading => write!(f, "Reading"),
            ExerciseType::Coding => write!(f, "Coding"),
            ExerciseType::Quiz => write!(f, "Quiz"),
        }
    }
}

// ── Convenience constructors ────────────────────────────────────────

impl Exercise {
    pub fn new(description: String, exercise_type: ExerciseType) -> Self {
        Self {
            description,
            exercise_type,
            status: Status::NotStarted,
        }
    }
}

impl Chapter {
    pub fn new(book: Book, number: u32, title: String) -> Self {
        Self {
            book,
            number,
            title,
            exercises: Vec::new(),
        }
    }

    pub fn status(&self) -> Status {
        if self.exercises.is_empty() {
            return Status::NotStarted;
        }

        let all_complete = self.exercises.iter().all(|e| e.status == Status::Complete);
        if all_complete {
            return Status::Complete;
        }

        let any_started = self
            .exercises
            .iter()
            .any(|e| e.status != Status::NotStarted);
        if any_started {
            return Status::InProgress;
        }

        Status::NotStarted
    }
}

impl Progress {
    pub fn new() -> Self {
        Self {
            chapters: Vec::new(),
        }
    }

    /// Count how many chapters have the given status.
    pub fn count_by_status(&self, status: &Status) -> usize {
        self.chapters
            .iter()
            .filter(|ch| ch.status() == *status)
            .count()
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::NotStarted
    }
}


// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chapter_status_no_exercises() {
        let ch = Chapter::new(Book::Trpl, 1, String::from("Getting Started"));
        assert_eq!(ch.status(), Status::NotStarted);
    }

    #[test]
    fn chapter_status_all_not_started() {
        let mut ch = Chapter::new(Book::Trpl, 3, String::from("Common Concepts"));
        ch.exercises
            .push(Exercise::new(String::from("Read the chapter"), ExerciseType::Reading));
        ch.exercises.push(Exercise::new(
            String::from("Write a temp converter"),
            ExerciseType::Coding,
        ));
        assert_eq!(ch.status(), Status::NotStarted);
    }

    #[test]
    fn chapter_status_in_progress() {
        let mut ch = Chapter::new(Book::RustForRustaceans, 1, String::from("Foundations"));
        ch.exercises
            .push(Exercise::new(String::from("Read the chapter"), ExerciseType::Reading));
        ch.exercises.push(Exercise {
            description: String::from("Quiz on ownership"),
            exercise_type: ExerciseType::Quiz,
            status: Status::Complete,
        });
        assert_eq!(ch.status(), Status::InProgress);
    }

    #[test]
    fn chapter_status_all_complete() {
        let mut ch = Chapter::new(Book::Trpl, 5, String::from("Using Structs"));
        ch.exercises.push(Exercise {
            description: String::from("Read the chapter"),
            exercise_type: ExerciseType::Reading,
            status: Status::Complete,
        });
        ch.exercises.push(Exercise {
            description: String::from("Build a rectangle struct"),
            exercise_type: ExerciseType::Coding,
            status: Status::Complete,
        });
        assert_eq!(ch.status(), Status::Complete);
    }

    #[test]
    fn progress_count_by_status() {
        let mut progress = Progress::new();

        let mut ch1 = Chapter::new(Book::Trpl, 1, String::from("Getting Started"));
        ch1.exercises.push(Exercise {
            description: String::from("Install Rust"),
            exercise_type: ExerciseType::Coding,
            status: Status::Complete,
        });

        let ch2 = Chapter::new(Book::Trpl, 2, String::from("Guessing Game"));

        progress.chapters.push(ch1);
        progress.chapters.push(ch2);

        assert_eq!(progress.count_by_status(&Status::Complete), 1);
        assert_eq!(progress.count_by_status(&Status::NotStarted), 1);
        assert_eq!(progress.count_by_status(&Status::InProgress), 0);
    }

    #[test]
    fn display_impls() {
        assert_eq!(Book::Trpl.to_string(), "The Rust Programming Language");
        assert_eq!(Book::RustForRustaceans.to_string(), "Rust for Rustaceans");
        assert_eq!(Status::InProgress.to_string(), "In Progress");
        assert_eq!(ExerciseType::Coding.to_string(), "Coding");
    }

    #[test]
    fn exercise_new_defaults_to_not_started() {
        let ex = Exercise::new(String::from("Do the thing"), ExerciseType::Quiz);
        assert_eq!(ex.status, Status::NotStarted);
        assert_eq!(ex.description, "Do the thing");
        assert_eq!(ex.exercise_type, ExerciseType::Quiz);
    }

    #[test]
    fn serde_round_trip() {
        let mut progress = Progress::new();
        let mut ch = Chapter::new(Book::Trpl, 1, String::from("Getting Started"));
        ch.exercises
            .push(Exercise::new(String::from("Hello world"), ExerciseType::Coding));
        progress.chapters.push(ch);

        let json = serde_json::to_string(&progress).unwrap();
        let deserialized: Progress = serde_json::from_str(&json).unwrap();
        assert_eq!(progress, deserialized);
    }
}
