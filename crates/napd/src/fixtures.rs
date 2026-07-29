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
    domains: &["shell", "identity", "inc", "outbox"],
};

const FOLLOW_LIST: FixtureDefinition = FixtureDefinition {
    name: "follow-list",
    title: "Direct follows",
    author: "5ffaf74a636594d5995750526f67a0db34b1c49db9433844ecfb981af7ba69b2",
    d_tag: "follow-list",
    aggregate_hash: "eaf4e565642e5cd055c8f69bea832d39701d04d3a820f5a5753f39bb3651ea9a",
    index_digest: "3ae0e253b192fff4aa36a86c0ddc48f20e86551058490b2893b52fa8d3d0edf4",
    artifact_base_url: "nmp-artifact://eaf4e565-642e-4cd0-95c8-f69bea832d39/",
    event: include_bytes!("../../../fixtures/follow-list/event.json"),
    index: include_bytes!("../../../fixtures/follow-list/index.html"),
    domains: &["shell", "identity", "inc"],
};

const PROFILE_CARD: FixtureDefinition = FixtureDefinition {
    name: "profile-card",
    title: "Profile card",
    author: "5ffaf74a636594d5995750526f67a0db34b1c49db9433844ecfb981af7ba69b2",
    d_tag: "profile-card",
    aggregate_hash: "9ee2d7bfebcd1c56f9c8c0e4641402e2d9ab7bed8c97c5d480cc77c04d5690cc",
    index_digest: "eeb037774dcc43faf6e0e13a9cf67aae8684b34c9c52921bcbd511739c46fa63",
    artifact_base_url: "nmp-artifact://9ee2d7bf-ebcd-4c56-9c8c-0e4641402e2d/",
    event: include_bytes!("../../../fixtures/profile-card/event.json"),
    index: include_bytes!("../../../fixtures/profile-card/index.html"),
    domains: &["shell", "inc", "outbox"],
};

const HOSTILE_EGRESS: FixtureDefinition = FixtureDefinition {
    name: "hostile-egress",
    title: "Hostile egress probe",
    author: "be1a049c1b9da66d504a808cbb1141ab37f03b1505eefa43f49894eff379c73f",
    d_tag: "egress-probe",
    aggregate_hash: "4f69e62d242a6f0d1d13ff7721325906940491037c79fe4c2f0bd61c0f1e1022",
    index_digest: "1843ffc7ee9710c207c7097cab5d2a376bb3b94e96ed4b7872db8403c815a828",
    artifact_base_url: "nmp-artifact://4f69e62d-242a-4f0d-9d13-ff7721325906/",
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
