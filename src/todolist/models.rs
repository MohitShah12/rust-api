use serde::Deserialize;

#[derive(Deserialize,Clone)]
pub struct CreateEntryData{
    pub title:String,
    pub description:String
}

#[derive(Deserialize,Clone)]
pub struct UpdateEnrtyData{
    pub title:String,
    pub description:String
}