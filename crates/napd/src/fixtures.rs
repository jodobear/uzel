use nmp_native_runtime_ffi::{ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource};

pub const MAXIMUM_ACTIVE_FIXTURES: usize = 4;

pub struct FixtureDefinition {
    pub name: &'static str,
    pub title: &'static str,
    pub author: &'static str,
    pub d_tag: &'static str,
    pub aggregate_hash: &'static str,
    pub index_digest: &'static str,
    pub artifact_base_url: &'static str,
    pub event: &'static [u8],
    pub index: &'static [u8],
    pub domains: &'static [&'static str],
}

const GOOD_MORNING: FixtureDefinition = FixtureDefinition {
    name: "good-morning",
    title: "Good Morning Protocol",
    author: "266815e0c9210dfa324c6cba3573b14bee49da4209a9456f9484e5106cd408a5",
    d_tag: "good-morning",
    aggregate_hash: "828a6df02afd56782ea20f805084acce65c53f7c37554948c1e0a64aa5a2b0a8",
    index_digest: "ffd35eea5c84d03cdda74c23e1bbb2c40500f503833503aa688036faa52f3808",
    artifact_base_url: "nmp-artifact://828a6df0-2afd-4678-a20f-805084acce65/",
    event: include_bytes!("../../../fixtures/good-morning/event.json"),
    index: include_bytes!("../../../fixtures/good-morning/index.html"),
    domains: &["shell", "identity", "inc", "outbox", "resource"],
};

const FOLLOW_LIST: FixtureDefinition = FixtureDefinition {
    name: "follow-list",
    title: "Direct follows",
    author: "8dd2fc14f5d41e5b87d3de9baf3129398602a3d257a3d0c415f0841f0550a155",
    d_tag: "follow-list",
    aggregate_hash: "522e97dccfbf47d24074625e8ed0be83833d2f797a56ec8215d08ff9294cbb4d",
    index_digest: "ecf5118afcb9d1dd218288445397ae596cf85d2d6ad7de57bb7097c8faf217ce",
    artifact_base_url: "nmp-artifact://522e97dc-cfbf-47d2-8074-625e8ed0be83/",
    event: include_bytes!("../../../fixtures/follow-list/event.json"),
    index: include_bytes!("../../../fixtures/follow-list/index.html"),
    domains: &["shell", "identity", "inc"],
};

const PROFILE_CARD: FixtureDefinition = FixtureDefinition {
    name: "profile-card",
    title: "Profile card",
    author: "8dd2fc14f5d41e5b87d3de9baf3129398602a3d257a3d0c415f0841f0550a155",
    d_tag: "profile-card",
    aggregate_hash: "fa03cba781c3c61ccb0fce6c7ac69eaf4c903b1fadddb7ed44464393197cd873",
    index_digest: "868a771fc87eeba3a501c356d7c170d54a1df3e89268919b6d72add239e28526",
    artifact_base_url: "nmp-artifact://fa03cba7-81c3-461c-8b0f-ce6c7ac69eaf/",
    event: include_bytes!("../../../fixtures/profile-card/event.json"),
    index: include_bytes!("../../../fixtures/profile-card/index.html"),
    domains: &["shell", "identity", "inc", "outbox", "resource"],
};

const HOSTILE_EGRESS: FixtureDefinition = FixtureDefinition {
    name: "hostile-egress",
    title: "Hostile egress probe",
    author: "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
    d_tag: "egress-probe",
    aggregate_hash: "d29a7660cd37118f9d619a16854617b0d44b20d16b3f6a45b9f8e28ce5187a16",
    index_digest: "749d4742bde8d42a85f0719f12248203a86ebb9f7f0ace408951f08eb8e15285",
    artifact_base_url: "nmp-artifact://d29a7660-cd37-4118-9d61-9a16854617b0/",
    event: include_bytes!("../../../fixtures/hostile-egress/event.json"),
    index: include_bytes!("../../../fixtures/hostile-egress/index.html"),
    domains: &["shell", "config"],
};

const FIXTURES: [&FixtureDefinition; MAXIMUM_ACTIVE_FIXTURES] =
    [&GOOD_MORNING, &FOLLOW_LIST, &PROFILE_CARD, &HOSTILE_EGRESS];

pub fn fixture_by_name(name: &str) -> Option<&'static FixtureDefinition> {
    FIXTURES
        .iter()
        .copied()
        .find(|fixture| fixture.name == name)
}

#[derive(Debug)]
pub struct ExactFixtureSource;

impl ArtifactSource for ExactFixtureSource {
    fn fetch(&self, request: ArtifactFetchRequest) -> ArtifactFetchResponse {
        let fixture = FIXTURES
            .iter()
            .copied()
            .find(|fixture| fixture.index_digest == request.expected_sha256);
        let accepted = request.logical_path == "/index.html"
            && fixture.is_some_and(|fixture| request.maximum_bytes >= fixture.index.len() as u64)
            && !request.redirects_allowed;
        ArtifactFetchResponse::Body {
            source_url: request.candidate_urls.first().cloned().unwrap_or_default(),
            http_status: if accepted { 200 } else { 404 },
            bytes: if accepted {
                fixture.map_or_else(Vec::new, |fixture| fixture.index.to_vec())
            } else {
                Vec::new()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trusted_shell_artifact_url(value: &str) -> bool {
        let Some(uuid) = value
            .strip_prefix("nmp-artifact://")
            .and_then(|value| value.strip_suffix('/'))
        else {
            return false;
        };
        let bytes = uuid.as_bytes();
        bytes.len() == 36
            && bytes[8] == b'-'
            && bytes[13] == b'-'
            && bytes[18] == b'-'
            && bytes[23] == b'-'
            && bytes[14] == b'4'
            && matches!(bytes[19], b'8' | b'9' | b'a' | b'b')
            && bytes
                .iter()
                .enumerate()
                .all(|(index, byte)| matches!(index, 8 | 13 | 18 | 23) || byte.is_ascii_hexdigit())
    }

    #[test]
    fn fixture_artifact_urls_match_the_trusted_shell_contract() {
        for fixture in FIXTURES {
            assert!(
                trusted_shell_artifact_url(fixture.artifact_base_url),
                "{} has an invalid artifact base URL",
                fixture.name
            );
        }
    }
}
