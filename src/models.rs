use crate::fields::*;

pub type TitleField = CharField<255>;

#[derive(Debug, Default)]
pub enum CardActiveState {
    #[default]
    Todo,
    Doing,
    Done,
    Archived,
}

#[derive(Debug)]
pub struct Board {
    pub id: Id,
    pub title: TitleField,
    pub description: TextField,
}

#[derive(Debug)]
pub struct Column {
    pub id: Id,
    pub board_id: Id,
    pub ownder_id: Id,

    pub title: TitleField,
    pub description: TextField,
    pub color: ColorField,
}

#[derive(Debug)]
pub struct Card {
    pub id: Id,
    pub owner_id: Id,
    pub column_id: Id,

    pub title: TitleField,
    pub description: TextField,
    pub active_state: CardActiveState,
}
