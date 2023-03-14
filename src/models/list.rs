use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let state = ListState::default();
        StatefulList {
            state: state,
            items,
        }
    }

    pub fn next(&mut self) {
        if self.items.len() == 0 {
            self.unselect();
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.len() == 0 {
            self.unselect();
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    #[allow(dead_code)]
    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn selected_mut(&mut self) -> Option<&mut T> {
        self.state
            .selected()
            .map_or(None, |index| Some(&mut self.items[index]))
    }

    pub fn selected(&self) -> Option<&T> {
        self.state
            .selected()
            .map_or(None, |index| Some(&self.items[index]))
    }

    pub fn update_selected(&mut self, item: T) {
        if let Some(index) = self.state.selected() {
            self.items[index] = item;
        }
    }

    pub fn remove_at_index(&mut self, index: usize) {
        self.items.remove(index);
        self.previous();
    }
}
