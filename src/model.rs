pub struct Candidate {
    pub personal_info: PersonalInformation,
    pub experience: Vec<Experience>,
    pub skills: Vec<Skill>,
    pub notes: Vec<String>,
}

pub struct Experience {
    pub from_year: String,
    pub to_year: String,
    pub kind: ExperienceKind,
}

pub enum ExperienceKind {
    Education(EducationExperience),
    Work(WorkExperience),
}

pub struct EducationExperience {
    pub school: String,
    pub grade: f32,
}

pub struct WorkExperience {
    pub company: String,
}

pub struct PersonalInformation {
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
}

pub struct Skill {
    pub name: String,
    pub rating: u8,
}
