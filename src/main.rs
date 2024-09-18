mod element;
mod model;

use raylib::{
    ffi::{Camera2D, Vector2},
    prelude::*,
};

use crate::element::{
    BulletListElement, Drawable, Element, ExpandableSectionElement, ExperienceListElement,
    TitleElement,
};
use crate::model::{
    EducationExperience, Experience, ExperienceKind, PersonalInformation, WorkExperience,
};

fn main() {
    let (mut handle, thread) = raylib::init()
        .size(800, 600)
        .title("Candidate Viewer")
        .build();

    let camera = Camera2D {
        offset: Vector2 { x: 0.0, y: 0.0 },
        target: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };

    let title = TitleElement {
        position: Vector2 { x: 10.0, y: 10.0 },
        info: PersonalInformation {
            first_name: String::from("Max"),
            last_name: String::from("Musterman"),
            job_title: String::from("Engineer"),
        },
    };

    let experience_list = ExperienceListElement {
        position: Vector2 { x: 0.0, y: 0.0 },
        experiences: vec![
            Experience {
                from_year: String::from("2012"),
                to_year: String::from("2013"),
                kind: ExperienceKind::Education(EducationExperience {
                    school: String::from("University 1"),
                    grade: 2.0,
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
    };

    let mut experience_section = ExpandableSectionElement::new(
        "Experience",
        20.0,
        100.0,
        Element::ExperienceList(experience_list),
    );

    let bullet_list = BulletListElement {
        position: Vector2 { x: 0.0, y: 0.0 },
        items: vec![String::from("Skill 1"), String::from("Skill 2")],
    };

    let mut skill_section =
        ExpandableSectionElement::new("Skills", 20.0, 200.0, Element::BulletList(bullet_list));

    while !handle.window_should_close() {
        let mut d = handle.begin_drawing(&thread);

        let mut d = d.begin_mode2D(camera);

        d.clear_background(Color::WHITE);

        let mouse_position = d.get_mouse_position();
        let mouse_pressed = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        // Handle events
        if experience_section.is_inside(mouse_position.into()) && mouse_pressed {
            experience_section.toggle();
        }

        if skill_section.is_inside(mouse_position.into()) && mouse_pressed {
            skill_section.toggle();
        }

        // Draw
        let mut offset_y: f32 = 0.0;

        title.draw(&mut d);
        offset_y += title.size().height + 25.0;

        experience_section.set_position(Vector2 {
            x: 20.0,
            y: offset_y,
        });
        experience_section.draw(&mut d);
        offset_y += experience_section.size().height + 25.0;

        skill_section.set_position(Vector2 {
            x: 20.0,
            y: offset_y,
        });
        skill_section.draw(&mut d);
    }
}
