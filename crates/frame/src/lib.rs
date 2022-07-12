pub mod id;
pub mod status;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Commands {
    pub opcmd: String,
    pub status: String,
    pub datas: String,
}