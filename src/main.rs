mod element;
mod model;

use raylib::{ffi::Vector2, prelude::*};

use crate::element::{
    BulletListElement, Drawable, Element, ExpandableSectionElement, ExperienceListElement,
    SkillListElement, TitleElement,
};
use crate::model::{
    Candidate, EducationExperience, Experience, ExperienceKind, PersonalInformation, Skill,
    WorkExperience,
};

fn main() {
    let (mut handle, thread) = raylib::init()
        .size(800, 600)
        .title("Candidate Viewer")
        .build();

    let candidate = Candidate {
        personal_info: PersonalInformation {
            first_name: String::from("Max"),
            last_name: String::from("Musterman"),
            job_title: String::from("Engineer"),
        },
        experience: vec![
            Experience {
                to_year: String::from("2012"),
                from_year: String::from("2013"),
                kind: ExperienceKind::Education(EducationExperience {
                    grade: 2.0,
                    school: String::from("University 1"),
                }),
            },
            Experience {
                from_year: String::from("2013"),
                to_year: String::from("2015"),
                kind: ExperienceKind::Work(WorkExperience {
                    company: String::from("Company 1"),
                }),
            },
        ],
        skills: vec![
            Skill {
                name: String::from("CSS"),
                rating: 5,
            },
            Skill {
                name: String::from("HTML"),
                rating: 9,
            },
            Skill {
                name: String::from("Excel"),
                rating: 1,
            },
        ],
        notes: vec![String::from("Note 1"), String::from("Note 2")],
    };

    let mut elements = build_elements(candidate);

    while !handle.window_should_close() {
        let mut d = handle.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        let mouse_position = d.get_mouse_position();
        let mouse_pressed = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        let mut offset_y: f32 = 0.0;

        for element in &mut elements {
            if let Element::ExpandableSection(section) = element {
                if section.is_inside(mouse_position.into()) && mouse_pressed {
                    section.toggle();
                }
            }

            element.set_position(Vector2 {
                x: 20.0,
                y: offset_y,
            });
            element.draw(&mut d);
            offset_y += element.size().height + 25.0;
        }
    }
}

fn build_elements(candidate: Candidate) -> Vec<Element> {
    let title = TitleElement {
        position: Vector2 { x: 0.0, y: 0.0 },
        info: candidate.personal_info,
    };

    let experience_list = ExperienceListElement {
        position: Vector2 { x: 0.0, y: 0.0 },
        experiences: candidate.experience,
    };

    let experience_section = ExpandableSectionElement::new(
        "Experience",
        20.0,
        100.0,
        Element::ExperienceList(experience_list),
    );

    let skill_list = SkillListElement {
        position: Vector2 { x: 0.0, y: 0.0 },
        skills: candidate.skills,
    };

    let skill_section =
        ExpandableSectionElement::new("Skills", 20.0, 200.0, Element::SkillList(skill_list));

    let note_list = BulletListElement {
        position: Vector2 { x: 0.0, y: 0.0 },
        items: candidate.notes,
    };

    let note_section =
        ExpandableSectionElement::new("Notes", 20.0, 200.0, Element::BulletList(note_list));

    vec![
        Element::Title(title),
        Element::ExpandableSection(experience_section),
        Element::ExpandableSection(skill_section),
        Element::ExpandableSection(note_section),
    ]
}
