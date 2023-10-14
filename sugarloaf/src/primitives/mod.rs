pub mod utils;
pub mod style;
pub mod media;

use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Sugar {
    pub content: char,
    pub foreground_color: [f32; 4],
    pub background_color: [f32; 4],
    pub style: Option<style::SugarStyle>,
    pub decoration: Option<style::SugarDecoration>,
}

#[derive(Default, Debug)]
pub struct SugarStack {
    pub inner: Vec<Sugar>,
    pub id: String,
}

impl SugarStack {
    pub fn new() -> SugarStack {
        SugarStack {
            inner: vec![],
            id: "".to_string(),
        }
    }

    #[inline]
    pub fn add(&mut self, sugar: Sugar) {
    	// TODO: Improve the id creation
    	self.id += &(sugar.content as u32).to_string();
    	self.id += &sugar.background_color[0].to_string();
    	self.id += &sugar.background_color[1].to_string();
    	self.id += &sugar.background_color[2].to_string();
    	self.id += &sugar.background_color[3].to_string();
    	self.id += &sugar.foreground_color[0].to_string();
    	self.id += &sugar.foreground_color[1].to_string();
    	self.id += &sugar.foreground_color[2].to_string();
    	self.id += &sugar.foreground_color[3].to_string();
        self.inner.push(sugar);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Index<usize> for SugarStack {
    type Output = Sugar;

    fn index(&self, idx: usize) -> &Sugar {
        &self.inner[idx]
    }
}

impl IndexMut<usize> for SugarStack {
    fn index_mut(&mut self, idx: usize) -> &mut Sugar {
        &mut self.inner[idx]
    }
}