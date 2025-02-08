use crate::fields::{
    CardActiveStateField, CharField, ForeignKeyField, Id, RgbaColorField, TextField,
    TitleField,
};

#[derive(Debug)]
pub struct User {
    pub id: Id,
    pub username: CharField<255>,
}

#[derive(Debug)]
pub struct Board {
    pub id: Id,
    pub owner: ForeignKeyField<User, Id>,

    pub title: TitleField,
    pub description: TextField,
}

#[derive(Debug)]
pub struct Column {
    pub id: Id,
    pub owner: ForeignKeyField<User, Id>,
    pub board: ForeignKeyField<Board, Id>,

    pub title: TitleField,
    pub description: TextField,
    pub color: RgbaColorField,
}

#[derive(Debug)]
pub struct Card {
    pub id: Id,
    pub owner: ForeignKeyField<User, Id>,
    pub board: ForeignKeyField<Board, Id>,
    pub column: ForeignKeyField<Column, Id>,

    pub title: TitleField,
    pub description: TextField,
    pub active_state: CardActiveStateField,
}
