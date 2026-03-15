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

    /// Resolve an identifier to an [`Image`].
    ///
    /// Match priority:
    /// 1. Exact `image_id` match
    /// 2. `image_id` prefix match (error if ambiguous)
    /// 3. Case-insensitive `friendly_name` match (error if ambiguous)
    ///
    /// Returns a descriptive error when no match is found or the match is ambiguous.
    pub fn resolve_image(&self, identifier: &str) -> Result<Image> {
        let images = self.get_images()?;

        // 1. Exact image_id match
        if let Some(img) = images.iter().find(|img| img.image_id == identifier) {
            return Ok(img.clone());
        }

        // 2. image_id prefix match
        let prefix_matches: Vec<&Image> = images
            .iter()
            .filter(|img| img.image_id.starts_with(identifier))
            .collect();

        match prefix_matches.len() {
            1 => return Ok(prefix_matches[0].clone()),
            n if n > 1 => {
                let ids: Vec<&str> = prefix_matches
                    .iter()
                    .map(|img| img.image_id.as_str())
                    .collect();
                anyhow::bail!(
                    "ambiguous image prefix {:?}: matches {} images ({})",
                    identifier,
                    n,
                    ids.join(", ")
                );
            }
            _ => {}
        }

        // 3. Case-insensitive friendly_name match
        let ident_lower = identifier.to_lowercase();
        let name_matches: Vec<&Image> = images
            .iter()
            .filter(|img| {
                img.friendly_name
                    .as_deref()
                    .map(|n| n.to_lowercase() == ident_lower)
                    .unwrap_or(false)
            })
            .collect();

        match name_matches.len() {
            1 => Ok(name_matches[0].clone()),
            n if n > 1 => {
                let ids: Vec<&str> = name_matches
                    .iter()
                    .map(|img| img.image_id.as_str())
                    .collect();
                anyhow::bail!(
                    "ambiguous image name {:?}: matches {} images ({})",
                    identifier,
                    n,
                    ids.join(", ")
                );
            }
            _ => anyhow::bail!(
                "image {:?} not found (tried exact ID, ID prefix, and name match)",
                identifier
            ),
        }
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
