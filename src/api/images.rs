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

    pub fn delete_image(&self, image_id: &str) -> Result<()> {
        #[derive(Serialize)]
        struct TargetImage<'a> {
            image_id: &'a str,
        }

        #[derive(Serialize)]
        struct Req<'a> {
            target_image: TargetImage<'a>,
        }

        #[derive(Deserialize)]
        struct Resp {}

        let _: Resp = self.post(
            "admin/delete_image",
            &Req {
                target_image: TargetImage { image_id },
            },
        )?;
        Ok(())
    }
}
