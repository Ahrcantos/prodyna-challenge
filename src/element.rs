use raylib::{
    color::Color,
    drawing::RaylibDraw,
    ffi::{Rectangle, Vector2},
};

use crate::model::{Experience, ExperienceKind, PersonalInformation, Skill};

pub trait Drawable {
    fn draw(&self, draw: &mut impl RaylibDraw);
    fn size(&self) -> Rectangle;
    fn set_position(&mut self, position: Vector2);
}

pub enum Element {
    Title(TitleElement),
    BulletList(BulletListElement),
    ExperienceList(ExperienceListElement),
    ExpandableSection(ExpandableSectionElement),
    SkillList(SkillListElement),
}

impl Drawable for Element {
    fn draw(&self, draw: &mut impl RaylibDraw) {
        match self {
            Element::Title(el) => el.draw(draw),
            Element::BulletList(el) => el.draw(draw),
            Element::ExperienceList(el) => el.draw(draw),
            Element::ExpandableSection(el) => el.draw(draw),
            Element::SkillList(el) => el.draw(draw),
        }
    }

    fn size(&self) -> Rectangle {
        match self {
            Element::Title(el) => el.size(),
            Element::BulletList(el) => el.size(),
            Element::ExperienceList(el) => el.size(),
            Element::ExpandableSection(el) => el.size(),
            Element::SkillList(el) => el.size(),
        }
    }

    fn set_position(&mut self, position: Vector2) {
        match self {
            Element::Title(el) => el.set_position(position),
            Element::BulletList(el) => el.set_position(position),
            Element::ExperienceList(el) => el.set_position(position),
            Element::ExpandableSection(el) => el.set_position(position),
            Element::SkillList(el) => el.set_position(position),
        }
    }
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
            width: 600.0,
            height: 54.0,
        }
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
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
                (self.position.y + offset_y + 7.0) as i32,
                10,
                5,
                Color::BLACK,
            );

            draw.draw_text(
                item,
                (self.position.x + 20.0) as i32,
                (self.position.y + offset_y) as i32,
                20,
                Color::BLACK,
            );

            offset_y += 20.0;
        }
    }

    fn size(&self) -> Rectangle {
        let height = (self.items.len() as f32) * 20.0;

        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: 600.0,
            height,
        }
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
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
                        "{} ({:.1})",
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
        let height = (self.experiences.len() as f32) * 20.0;

        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: 600.0,
            height,
        }
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }
}

pub struct ExpandableSectionElement {
    position: Vector2,
    title: String,
    is_open: bool,
    content: Box<Element>,
}

impl ExpandableSectionElement {
    pub fn new(title: &'static str, x: f32, y: f32, content: Element) -> Self {
        let mut content = Box::new(content);
        content.set_position(Vector2 {
            x: x + 10.0,
            y: y + 30.0,
        });

        Self {
            position: Vector2 { x, y },
            title: String::from(title),
            is_open: false,
            content,
        }
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

impl Drawable for ExpandableSectionElement {
    fn draw(&self, draw: &mut impl RaylibDraw) {
        draw.draw_rectangle(
            self.position.x as i32,
            self.position.y as i32,
            760,
            24,
            Color::GRAY,
        );

        draw.draw_text(
            &self.title,
            self.position.x as i32,
            self.position.y as i32,
            24,
            Color::BLACK,
        );

        if self.is_open {
            draw.draw_rectangle(
                self.position.x as i32,
                (self.position.y + 24.0) as i32,
                760,
                (self.content.size().height + 10.0) as i32,
                Color::LIGHTGRAY,
            );

            self.content.draw(draw);
        }
    }

    fn size(&self) -> Rectangle {
        if self.is_open {
            Rectangle {
                x: self.position.x,
                y: self.position.y,
                width: 760.0,
                height: self.content.size().height + 10.0 + 24.0,
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

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
        self.content.set_position(Vector2 {
            x: position.x + 10.0,
            y: position.y + 30.0,
        });
    }
}

pub struct SkillListElement {
    pub position: Vector2,
    pub skills: Vec<Skill>,
}

impl Drawable for SkillListElement {
    fn draw(&self, draw: &mut impl RaylibDraw) {
        let mut offset_y: f32 = 0.0;

        for skill in &self.skills {
            draw.draw_text(
                &skill.name,
                self.position.x as i32,
                (self.position.y + offset_y) as i32,
                20,
                Color::BLACK,
            );

            let color = match skill.rating {
                0..=3 => Color::RED,
                4..=7 => Color::YELLOW,
                _ => Color::GREEN,
            };

            for i in (0..skill.rating).take(10) {
                let offset_x = (i as f32) * 22.0;

                draw.draw_rectangle(
                    (self.position.x + 200.0 + offset_x) as i32,
                    (self.position.y + offset_y) as i32,
                    20,
                    20,
                    color,
                );
            }

            offset_y += 22.0;
        }
    }

    fn size(&self) -> Rectangle {
        let height = (self.skills.len() as f32) * 20.0;

        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: 600.0,
            height,
        }
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }
}
