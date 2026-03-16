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

    /// Resolve an identifier to a [`Zone`].
    ///
    /// Match priority:
    /// 1. Exact `zone_id` match
    /// 2. `zone_id` prefix match (error if ambiguous)
    /// 3. Case-insensitive `zone_name` match (error if ambiguous)
    ///
    /// Returns a descriptive error when no match is found or the match is ambiguous.
    pub fn resolve_zone(&self, identifier: &str) -> Result<Zone> {
        let zones = self.get_zones()?;

        // 1. Exact zone_id match
        if let Some(z) = zones.iter().find(|z| z.zone_id == identifier) {
            return Ok(z.clone());
        }

        // 2. zone_id prefix match
        let prefix_matches: Vec<&Zone> = zones
            .iter()
            .filter(|z| z.zone_id.starts_with(identifier))
            .collect();

        match prefix_matches.len() {
            1 => return Ok(prefix_matches[0].clone()),
            n if n > 1 => {
                let ids: Vec<&str> = prefix_matches.iter().map(|z| z.zone_id.as_str()).collect();
                anyhow::bail!(
                    "ambiguous zone prefix {:?}: matches {} zones ({})",
                    identifier,
                    n,
                    ids.join(", ")
                );
            }
            _ => {}
        }

        // 3. Case-insensitive zone_name match
        let ident_lower = identifier.to_lowercase();
        let name_matches: Vec<&Zone> = zones
            .iter()
            .filter(|z| {
                z.zone_name
                    .as_deref()
                    .map(|n| n.to_lowercase() == ident_lower)
                    .unwrap_or(false)
            })
            .collect();

        match name_matches.len() {
            1 => Ok(name_matches[0].clone()),
            n if n > 1 => {
                let ids: Vec<&str> = name_matches.iter().map(|z| z.zone_id.as_str()).collect();
                anyhow::bail!(
                    "ambiguous zone name {:?}: matches {} zones ({})",
                    identifier,
                    n,
                    ids.join(", ")
                );
            }
            _ => anyhow::bail!(
                "zone {:?} not found (tried exact ID, ID prefix, and name match)",
                identifier
            ),
        }
    }
}
