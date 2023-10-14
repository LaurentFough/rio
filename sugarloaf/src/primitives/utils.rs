use crate::primitives::Sugar;
use crate::primitives::style::{SugarStyle, SugarDecoration};

#[derive(Debug)]
pub struct RepeatedSugar {
    pub content: Option<char>,
    pub content_str: String,
    pub foreground_color: [f32; 4],
    pub background_color: [f32; 4],
    pub style: Option<SugarStyle>,
    pub decoration: Option<SugarDecoration>,
    pub quantity: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub reset_on_next: bool,
}

impl RepeatedSugar {
    pub fn new(quantity: usize) -> RepeatedSugar {
        RepeatedSugar {
            content: None,
            content_str: String::from(""),
            foreground_color: [0.0, 0.0, 0.0, 0.0],
            background_color: [0.0, 0.0, 0.0, 0.0],
            style: None,
            decoration: None,
            quantity,
            pos_x: 0.0,
            pos_y: 0.0,
            reset_on_next: false,
        }
    }

    #[inline]
    pub fn set_reset_on_next(&mut self) {
        self.reset_on_next = true;
    }

    #[inline]
    pub fn reset_on_next(&self) -> bool {
        self.reset_on_next
    }

    #[inline]
    pub fn reset(&mut self) {
        self.content = None;
        self.content_str = String::from("");
        self.foreground_color = [0.0, 0.0, 0.0, 0.0];
        self.background_color = [0.0, 0.0, 0.0, 0.0];
        self.quantity = 0;
        self.reset_on_next = false;
    }

    #[inline]
    pub fn set(&mut self, sugar: &Sugar, pos_x: f32, pos_y: f32) {
        self.content = Some(sugar.content);
        self.content_str += &sugar.content.to_string();
        self.foreground_color = sugar.foreground_color;
        self.background_color = sugar.background_color;
        if self.quantity == 0 {
            self.pos_x = pos_x;
            self.pos_y = pos_y;
            self.content_str += &sugar.content.to_string();
        }
        self.quantity += 1;
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.quantity
    }
}