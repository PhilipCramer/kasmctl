use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::zone::Zone;

use super::KasmClient;

impl KasmClient {
    pub fn get_zones(&self) -> Result<Vec<Zone>> {
        #[derive(Serialize)]
        struct Req {}

        #[derive(Deserialize)]
        struct Resp {
            zones: Vec<Zone>,
        }

        let resp: Resp = self.post("public/get_zones", &Req {})?;
        Ok(resp.zones)
    }
}
