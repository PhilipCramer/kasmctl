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

    /// Look up the user_id for a session by scanning all active sessions.
    pub fn resolve_user_id(&self, kasm_id: &str) -> Result<String> {
        let sessions = self.get_kasms()?;
        let session = sessions
            .iter()
            .find(|s| s.kasm_id == kasm_id)
            .ok_or_else(|| anyhow::anyhow!("session {kasm_id:?} not found"))?;
        session
            .user_id
            .clone()
            .ok_or_else(|| anyhow::anyhow!("session {kasm_id:?} has no user_id"))
    }

    /// Execute a command inside a running session.
    pub fn exec_command_kasm(
        &self,
        kasm_id: &str,
        user_id: &str,
        cmd: &str,
        workdir: Option<&str>,
        privileged: bool,
        exec_user: Option<&str>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct ExecConfig<'a> {
            cmd: &'a str,
            environment: std::collections::HashMap<String, String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            workdir: Option<&'a str>,
        }

        #[derive(Serialize)]
        struct Req<'a> {
            kasm_id: &'a str,
            user_id: &'a str,
            exec_config: ExecConfig<'a>,
            privileged: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            exec_user: Option<&'a str>,
        }

        let req = Req {
            kasm_id,
            user_id,
            exec_config: ExecConfig {
                cmd,
                environment: std::collections::HashMap::new(),
                workdir,
            },
            privileged,
            exec_user,
        };

        let _: serde_json::Value = self.post("public/exec_command_kasm", &req)?;
        Ok(())
    }
}
