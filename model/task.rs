use serde::{Serialize,Deserialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;


#[derive(Debug, Clone, Serialize, Deserialize,ToSchema)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,ToSchema)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub due_date: String,  // Can use chrono for DateTime
    pub status: String,    // Enum can be used for stricter type safety
}

#[derive(Validate, Serialize, Deserialize,ToSchema)]
pub struct AddTodo{
    #[validate(length(min= 1, message="todo required"))]
    pub todo: String,
    
}
// pub mod todo;