use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::image::Image;

use super::KasmClient;

/// Parameters for creating a new workspace image.
#[derive(Serialize)]
pub struct CreateImageParams {
    pub name: String,
    pub friendly_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    pub enabled: bool,
    pub image_src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_registry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
}

/// Request body for updating a workspace image.
/// Only `image_id` is required; all other fields are optional
/// and only sent when set (via `#[serde(skip_serializing_if)]`).
#[derive(Serialize)]
pub struct UpdateImageRequest {
    pub image_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_src: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_registry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

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

    pub fn create_image(&self, params: &CreateImageParams) -> Result<Image> {
        #[derive(Serialize)]
        struct Req<'a> {
            target_image: &'a CreateImageParams,
        }

        #[derive(Deserialize)]
        struct Resp {
            image: Image,
        }

        let resp: Resp = self.post(
            "admin/create_image",
            &Req {
                target_image: params,
            },
        )?;
        Ok(resp.image)
    }

    pub fn update_image(&self, req: &UpdateImageRequest) -> Result<Image> {
        #[derive(Serialize)]
        struct Req<'a> {
            target_image: &'a UpdateImageRequest,
        }

        #[derive(Deserialize)]
        struct Resp {
            image: Image,
        }

        let resp: Resp = self.post("admin/update_image", &Req { target_image: req })?;
        Ok(resp.image)
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
