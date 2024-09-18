mod element;
mod model;

use raylib::{
    ffi::{Camera2D, Vector2},
    prelude::*,
};

use crate::element::{BulletListElement, Drawable, ExperienceListElement, TitleElement};
use crate::model::{
    EducationExperience, Experience, ExperienceKind, PersonalInformation, WorkExperience,
};

struct ExpandableSection {
    position: Vector2,
    title: String,
    is_open: bool,
}

impl ExpandableSection {
    pub fn new(title: &'static str, x: f32, y: f32) -> Self {
        Self {
            position: Vector2 { x, y },
            title: String::from(title),
            is_open: false,
        }
    }

    pub fn size(&self) -> Rectangle {
        if self.is_open {
            Rectangle {
                x: self.position.x,
                y: self.position.y,
                width: 760.0,
                height: 224.0,
            }
        } else {
            Rectangle {
                x: self.position.x,
                y: self.position.y,
                width: 760.0,
                height: 24.0,
            }
        }
    }

    pub fn move_to(&mut self, vec: Vector2) {
        self.position = vec;
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open
    }

    pub fn is_inside(&self, position: Vector2) -> bool {
        let bounding = self.size();

        let inside_x = position.x >= bounding.x && position.x <= (bounding.x + bounding.width);
        let inside_y = position.y >= bounding.y && position.y <= (bounding.y + bounding.height);

        inside_x && inside_y
    }
}

fn main() {
    let (mut handle, thread) = raylib::init()
        .size(800, 600)
        .title("Candidate Viewer")
        .build();

    let mut sections = vec![
        ExpandableSection::new("Experience", 20.0, 100.0),
        ExpandableSection::new("Skills", 20.0, 200.0),
    ];

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

    while !handle.window_should_close() {
        let mut d = handle.begin_drawing(&thread);

        let mut d = d.begin_mode2D(camera);

        d.clear_background(Color::WHITE);

        title.draw(&mut d);

        let mouse_position = d.get_mouse_position();
        let mouse_pressed = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        let mut offset_y: f32 = 100.0;

        for section in &mut sections {
            section.move_to(Vector2 {
                x: 10.0,
                y: offset_y,
            });
            offset_y += section.size().height;
            offset_y += 25.0;
        }

        for section in &mut sections {
            if section.is_inside(mouse_position.into()) && mouse_pressed {
                section.toggle();
            }
        }

        for section in &sections {
            let font_size = 24;

            let position = d.get_world_to_screen2D(section.position, camera);
            d.draw_rectangle(
                position.x as i32,
                position.y as i32,
                760,
                font_size,
                Color::GRAY,
            );

            d.draw_text(
                &section.title,
                position.x as i32,
                position.y as i32,
                font_size,
                Color::BLACK,
            );

            if section.is_open {
                d.draw_rectangle(
                    position.x as i32,
                    position.y as i32 + 24,
                    760,
                    200,
                    Color::LIGHTGRAY,
                )
            }
        }

        // Experience section
        if sections[0].is_open {
            let section = &sections[0];
            let position = d.get_world_to_screen2D(section.position, camera);

            let element = ExperienceListElement {
                position: Vector2 {
                    x: position.x + 10.0,
                    y: position.y + 30.0,
                },
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

            element.draw(&mut d);
        }

        // Skills section
        if sections[1].is_open {
            let section = &sections[1];
            let position = d.get_world_to_screen2D(section.position, camera);

            let bullet = BulletListElement {
                position: Vector2 {
                    x: position.x + 10.0,
                    y: position.y + 10.0,
                },
                items: vec![String::from("Skill 1"), String::from("Skill 2")],
            };

            bullet.draw(&mut d);
        }
    }
}
