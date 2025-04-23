use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
pub struct School {
    students: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.entry(student.to_owned()).or_insert(grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students
            .values()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students: Vec<_> = self
            .students
            .iter()
            .filter(|(_, &g)| g == grade)
            .map(|(s, _)| s)
            .cloned()
            .collect();

        students.sort();
        students
    }
}
