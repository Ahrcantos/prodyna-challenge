use raylib::{
    color::Color,
    drawing::RaylibDraw,
    ffi::{Rectangle, Vector2},
};

use crate::model::{Experience, ExperienceKind, PersonalInformation};

pub trait Drawable {
    fn draw(&self, draw: &mut impl RaylibDraw);
    fn size(&self) -> Rectangle;
}

pub struct TitleElement {
    pub position: Vector2,
    pub info: PersonalInformation,
}

impl Drawable for TitleElement {
    fn draw(&self, draw: &mut impl RaylibDraw) {
        let full_name = format!("{} {}", &self.info.first_name, &self.info.last_name);

        draw.draw_text(
            &full_name,
            self.position.x as i32,
            self.position.y as i32,
            36,
            Color::BLACK,
        );

        draw.draw_text(
            &self.info.job_title,
            self.position.x as i32,
            (self.position.y + 30.0) as i32,
            24,
            Color::BLACK,
        );
    }

    fn size(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: 100.0,
            height: 100.0,
        }
    }
}

pub struct BulletListElement {
    pub position: Vector2,
    pub items: Vec<String>,
}

impl Drawable for BulletListElement {
    fn draw(&self, draw: &mut impl RaylibDraw) {
        let mut offset_y: f32 = 0.0;

        for item in &self.items {
            draw.draw_rectangle(
                self.position.x as i32,
                (self.position.y + offset_y + 27.0) as i32,
                10,
                5,
                Color::BLACK,
            );

            draw.draw_text(
                item,
                (self.position.x + 20.0) as i32,
                (self.position.y + offset_y + 20.0) as i32,
                20,
                Color::BLACK,
            );

            offset_y += 20.0;
        }
    }

    fn size(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: 100.0,
            height: 100.0,
        }
    }
}

pub struct ExperienceListElement {
    pub position: Vector2,
    pub experiences: Vec<Experience>,
}

impl Drawable for ExperienceListElement {
    fn draw(&self, draw: &mut impl RaylibDraw) {
        let mut offset_y: f32 = 0.0;

        for experience in &self.experiences {
            let time_frame = format!("{}-{}", experience.from_year, experience.to_year);

            draw.draw_text(
                &time_frame,
                self.position.x as i32,
                (self.position.y + offset_y) as i32,
                20,
                Color::BLACK,
            );

            match &experience.kind {
                ExperienceKind::Education(education_experience) => {
                    let text = format!(
                        "{} ({})",
                        education_experience.school, education_experience.grade
                    );

                    draw.draw_text(
                        "EDU",
                        (self.position.x + 110.0) as i32,
                        (self.position.y + offset_y) as i32,
                        20,
                        Color::GREEN,
                    );

                    draw.draw_text(
                        &text,
                        (self.position.x + 180.0) as i32,
                        (self.position.y + offset_y) as i32,
                        20,
                        Color::BLACK,
                    );
                }

                ExperienceKind::Work(work_experience) => {
                    draw.draw_text(
                        "WORK",
                        (self.position.x + 110.0) as i32,
                        (self.position.y + offset_y) as i32,
                        20,
                        Color::BLUE,
                    );

                    draw.draw_text(
                        &work_experience.company,
                        (self.position.x + 180.0) as i32,
                        (self.position.y + offset_y) as i32,
                        20,
                        Color::BLACK,
                    );
                }
            }

            offset_y += 20.0;
        }
    }

    fn size(&self) -> Rectangle {
        todo!()
    }
}
