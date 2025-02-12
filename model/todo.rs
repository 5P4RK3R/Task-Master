use serde::{Serialize,Deserialize};
use validator::Validate;
#[derive(Validate, Serialize, Deserialize)]
pub struct AddTodo{
    #[validate(length(min= 1, message="todo required"))]
    pub todo: String,
    
}