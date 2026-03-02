use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::report::{AgentResourceReport, ScalarReportData};

use super::KasmClient;

impl KasmClient {
    /// Retrieve a scalar report by name, with optional time delta and resolution.
    pub fn get_report(
        &self,
        name: &str,
        delta: Option<i64>,
        resolution: Option<&str>,
    ) -> Result<ScalarReportData> {
        #[derive(Serialize)]
        struct Req<'a> {
            name: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            delta: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            resolution: Option<&'a str>,
        }

        let resp: ScalarReportData = self.post(
            "admin/get_report",
            &Req {
                name,
                delta,
                resolution,
            },
        )?;
        Ok(resp)
    }

    /// Retrieve per-agent resource utilization.
    pub fn get_agent_report(&self) -> Result<Vec<AgentResourceReport>> {
        #[derive(Serialize)]
        struct Req {}

        #[derive(Deserialize)]
        struct Resp {
            agents: Vec<AgentResourceReport>,
        }

        let resp: Resp = self.post("admin/get_agent_report", &Req {})?;
        Ok(resp.agents)
    }
}
