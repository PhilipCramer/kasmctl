use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::session::{CreateSessionResponse, Session};

use super::KasmClient;

impl KasmClient {
    pub async fn get_kasms(&self) -> Result<Vec<Session>> {
        #[derive(Serialize)]
        struct Req {}

        #[derive(Deserialize)]
        struct Resp {
            kasms: Vec<Session>,
        }

        let resp: Resp = self.post("get_kasms", &Req {}).await?;
        Ok(resp.kasms)
    }

    pub async fn get_kasm_status(&self, kasm_id: &str) -> Result<Session> {
        #[derive(Serialize)]
        struct Req<'a> {
            kasm_id: &'a str,
        }

        #[derive(Deserialize)]
        struct Resp {
            kasm: Session,
        }

        let resp: Resp = self.post("get_kasm_status", &Req { kasm_id }).await?;
        Ok(resp.kasm)
    }

    pub async fn request_kasm(
        &self,
        image_id: &str,
        user_id: Option<&str>,
    ) -> Result<CreateSessionResponse> {
        #[derive(Serialize)]
        struct Req<'a> {
            image_id: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            user_id: Option<&'a str>,
        }

        self.post("request_kasm", &Req { image_id, user_id }).await
    }

    pub async fn destroy_kasm(&self, kasm_id: &str) -> Result<()> {
        #[derive(Serialize)]
        struct Req<'a> {
            kasm_id: &'a str,
        }

        #[derive(Deserialize)]
        struct Resp {}

        let _: Resp = self.post("destroy_kasm", &Req { kasm_id }).await?;
        Ok(())
    }
}
