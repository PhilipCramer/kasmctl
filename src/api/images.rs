use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::image::Image;

use super::KasmClient;

impl KasmClient {
    pub fn get_images(&self) -> Result<Vec<Image>> {
        #[derive(Serialize)]
        struct Req {}

        #[derive(Deserialize)]
        struct Resp {
            images: Vec<Image>,
        }

        let resp: Resp = self.post("public/get_images", &Req {})?;
        Ok(resp.images)
    }
}
