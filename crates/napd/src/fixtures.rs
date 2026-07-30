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
    author: "0133afdd76a777f7d5d4d576a87bb9ce5c03ee9198efaee60e7f84ab5755f108",
    d_tag: "follow-list",
    aggregate_hash: "10126de003137c1a8711a3c48f6aab6b3994cbef8d47d0f400c3490d60c58417",
    index_digest: "e7227ac1df0eb0f5533d923242c13bc819b689f1317f6db3c901e3344711aa08",
    artifact_base_url: "nmp-artifact://10126de0-0313-4c1a-8711-a3c48f6aab6b/",
    event: include_bytes!("../../../fixtures/follow-list/event.json"),
    index: include_bytes!("../../../fixtures/follow-list/index.html"),
    domains: &["shell", "identity", "inc"],
};

const PROFILE_CARD: FixtureDefinition = FixtureDefinition {
    name: "profile-card",
    title: "Profile card",
    author: "0133afdd76a777f7d5d4d576a87bb9ce5c03ee9198efaee60e7f84ab5755f108",
    d_tag: "profile-card",
    aggregate_hash: "e537754146e4c659157c9d5da878d7d8d22dd2222794e7c13b03be46000bf9e1",
    index_digest: "9c347b659caa43982218cf4b38996e5e7f17b7bc708667ce05e4a5fcdd5b021c",
    artifact_base_url: "nmp-artifact://e5377541-46e4-4659-957c-9d5da878d7d8/",
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
