use frunk::LabelledGeneric;

#[derive(Debug, LabelledGeneric)]
pub struct Health {
    pub status: String,
}
