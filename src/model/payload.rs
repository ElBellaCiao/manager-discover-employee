use crate::model::Assignment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub assignments: Vec<Assignment>,
}
