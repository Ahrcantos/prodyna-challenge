use raylib::{
    ffi::{Camera2D, Vector2},
    prelude::*,
};

struct Candidate {
    personal_info: PersonalInformation,
    experience: Vec<Experience>,
    skills: Vec<String>,
}

struct Experience {
    from_year: String,
    to_year: String,
}

enum ExperienceType {
    Education(EducationExperience),
    Work(WorkExperience),
}

struct EducationExperience {
    school: String,
    grade: f32,
}

struct WorkExperience {
    company: String,
}

struct PersonalInformation {
    first_name: String,
    last_name: String,
}

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

    pub fn title(&self) -> &str {
        &self.title
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
    let candidate = Candidate {
        personal_info: PersonalInformation {
            first_name: "Max".into(),
            last_name: "Mustermann".into(),
        },
        experience: vec![],
        skills: vec!["Skill 1".into(), "Skill 2".into(), "Skill 3".into()],
    };

    let (mut handle, thread) = raylib::init()
        .size(800, 600)
        .title("Candidate Viewer")
        .build();

    let mut sections = vec![
        ExpandableSection::new("Experience", 20.0, 100.0),
        ExpandableSection::new("Skills", 20.0, 200.0),
    ];

    let mut camera = Camera2D {
        offset: Vector2 { x: 0.0, y: 0.0 },
        target: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };

    while !handle.window_should_close() {
        let mut d = handle.begin_drawing(&thread);

        let mut d = d.begin_mode2D(camera);

        d.clear_background(Color::WHITE);

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

        d.draw_text("Max Musterman", 10, 10, 36, Color::BLACK);
        d.draw_text("Engineer", 10, 40, 24, Color::BLACK);

        d.draw_text(
            "This is a small liltte description for this person.",
            10,
            69,
            18,
            Color::BLACK,
        );

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

            d.draw_text(
                "2012-2013",
                (position.x + 10.0) as i32,
                (position.y + 30.0) as i32,
                20,
                Color::BLACK,
            );
            d.draw_text(
                "EDU",
                (position.x + 120.0) as i32,
                (position.y + 30.0) as i32,
                20,
                Color::GREEN,
            );
            d.draw_text(
                "University 1 (2.0)",
                (position.x + 190.0) as i32,
                (position.y + 30.0) as i32,
                20,
                Color::BLACK,
            );

            d.draw_text(
                "2013-2015",
                (position.x + 10.0) as i32,
                (position.y + 50.0) as i32,
                20,
                Color::BLACK,
            );
            d.draw_text(
                "WORK",
                (position.x + 120.0) as i32,
                (position.y + 50.0) as i32,
                20,
                Color::BLUE,
            );
            d.draw_text(
                "Workplace 1",
                (position.x + 190.0) as i32,
                (position.y + 50.0) as i32,
                20,
                Color::BLACK,
            );
        }

        // Skills section
        if sections[1].is_open {
            let section = &sections[1];
            let position = d.get_world_to_screen2D(section.position, camera);

            d.draw_rectangle(
                (position.x + 10.0) as i32,
                (position.y + 37.0) as i32,
                10,
                5,
                Color::BLACK,
            );

            d.draw_text(
                "Skill 1",
                (position.x + 30.0) as i32,
                (position.y + 30.0) as i32,
                20,
                Color::BLACK,
            );

            d.draw_rectangle(
                (position.x + 10.0) as i32,
                (position.y + 57.0) as i32,
                10,
                5,
                Color::BLACK,
            );

            d.draw_text(
                "Skill 2",
                (position.x + 30.0) as i32,
                (position.y + 50.0) as i32,
                20,
                Color::BLACK,
            );
        }
    }
}
