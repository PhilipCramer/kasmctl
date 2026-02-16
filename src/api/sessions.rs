use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::session::{CreateSessionResponse, Session};

use super::KasmClient;

impl KasmClient {
    pub fn get_kasms(&self) -> Result<Vec<Session>> {
        #[derive(Serialize)]
        struct Req {}

        #[derive(Deserialize)]
        struct Resp {
            kasms: Vec<Session>,
        }

        let resp: Resp = self.post("public/get_kasms", &Req {})?;
        Ok(resp.kasms)
    }

    pub fn get_kasm_status(&self, kasm_id: &str, user_id: &str) -> Result<Session> {
        #[derive(Serialize)]
        struct Req<'a> {
            kasm_id: &'a str,
            user_id: &'a str,
        }

        #[derive(Deserialize)]
        struct Resp {
            kasm: Session,
        }

        let resp: Resp = self.post("public/get_kasm_status", &Req { kasm_id, user_id })?;
        Ok(resp.kasm)
    }

    pub fn request_kasm(
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

        self.post("public/request_kasm", &Req { image_id, user_id })
    }

    pub fn destroy_kasm(&self, kasm_id: &str) -> Result<()> {
        #[derive(Serialize)]
        struct Req<'a> {
            kasm_id: &'a str,
        }

        #[derive(Deserialize)]
        struct Resp {}

        let _: Resp = self.post("public/destroy_kasm", &Req { kasm_id })?;
        Ok(())
    }

    pub fn stop_kasm(&self, kasm_id: &str) -> Result<()> {
        #[derive(Serialize)]
        struct Req<'a> {
            kasm_id: &'a str,
        }

        #[derive(Deserialize)]
        struct Resp {}

        let _: Resp = self.post("stop_kasm", &Req { kasm_id })?;
        Ok(())
    }

    pub fn pause_kasm(&self, kasm_id: &str) -> Result<()> {
        #[derive(Serialize)]
        struct Req<'a> {
            kasm_id: &'a str,
        }

        #[derive(Deserialize)]
        struct Resp {}

        let _: Resp = self.post("pause_kasm", &Req { kasm_id })?;
        Ok(())
    }

    pub fn resume_kasm(&self, kasm_id: &str) -> Result<()> {
        #[derive(Serialize)]
        struct Req<'a> {
            kasm_id: &'a str,
        }

        #[derive(Deserialize)]
        struct Resp {}

        let _: Resp = self.post("resume_kasm", &Req { kasm_id })?;
        Ok(())
    }
}
