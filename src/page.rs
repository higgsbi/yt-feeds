use std::cmp::{max, min};

use crossterm::terminal;

pub struct Page {
    pub current_index: usize,
    pub count_per_page: usize,
    pub count_total: usize,
}

impl Page {
    pub fn new(count_total: usize, lines_per_element: usize) -> Page {
        Page {
            current_index: 0,
            // Avoids terminal scrolling
            // Take smallest between count and the terminal divided up by each element's space
            count_per_page: min(
                min(count_total, 10),
                terminal::size().unwrap().1 as usize / lines_per_element - 4,
            ),
            count_total,
        }
    }

    pub fn current_page<'a, T>(&self, videos: &'a Vec<T>) -> &'a [T] {
        &videos[self.current_index..(self.current_index + self.count_per_page)]
    }

    pub fn next_page(&mut self) {
        self.current_index = min(
            self.current_index + self.count_per_page,
            self.count_total - self.count_per_page,
        )
    }

    pub fn prev_page(&mut self) {
        self.current_index = max(
            self.current_index as i32 - self.count_per_page as i32,
            0 as i32,
        ) as usize;
    }

    pub fn item_at_index<'a, T>(&self, videos: &'a [T], index: usize) -> Option<&'a T> {
        if self.item_is_at_index(index) {
            videos.get(index)
        } else {
            None
        }
    }

    pub fn item_is_at_index(&self, index: usize) -> bool {
        index <= (self.current_index + self.count_per_page - 1) && index >= self.current_index
    }
}
