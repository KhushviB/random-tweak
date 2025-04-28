use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateRecord {
    pub Category: String,
    pub Subcategory: String,
    pub Content: String,
}
