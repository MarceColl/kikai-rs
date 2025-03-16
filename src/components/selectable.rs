use bevy::prelude::*;

pub enum SelectableType {
    Unit,
    Building,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Selectable {
    ty: SelectableType,
    selected: bool,
}

impl Selectable {
    pub fn new() -> Self {
        Selectable {
            ty: SelectableType::Unit,
            selected: false,
        }
    }

    pub fn select(&mut self, _commands: &mut Commands) {
        self.selected = true;
    }
}
