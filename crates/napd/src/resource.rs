use std::{
    collections::BTreeSet,
    future::Future,
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::{Duration, Instant},
};

use nmp_native_nap_bridge::Provider;
use nmp_native_provider_resource::{
    NoopResourceActivity, PinnedHttpsRequest, RasterizedSvg, RawHttpsResponse, ResolveRequest,
    ResourceClock, ResourceDeadline, ResourceNetwork, ResourceNetworkError, ResourceProvider,
    ResourceProviderLimits, SvgRasterError, SvgRasterRequest, SvgRasterizer,
};
use nmp_native_runtime_core::Cancellation;
use reqwest::{Client, redirect::Policy};
use tokio::runtime::{Builder, Runtime};

const CANCELLATION_POLL: Duration = Duration::from_millis(20);
const RESOURCE_USER_AGENT: &str = "uzel-linux-poc/0.0.0";

#[derive(Debug)]
struct MonotonicResourceClock {
    origin: Instant,
}

impl MonotonicResourceClock {
    fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl ResourceClock for MonotonicResourceClock {
    fn monotonic_millis(&self) -> u64 {
        u64::try_from(self.origin.elapsed().as_millis()).unwrap_or(u64::MAX)
    }
}

#[derive(Debug)]
struct LinuxResourceNetwork {
    runtime: Runtime,
    clock: Arc<MonotonicResourceClock>,
}

impl LinuxResourceNetwork {
    fn new(clock: Arc<MonotonicResourceClock>) -> Result<Self, std::io::Error> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("uzel-resource")
            .enable_all()
            .build()?;
        Ok(Self { runtime, clock })
    }

    fn remaining(&self, deadline: ResourceDeadline) -> Result<Duration, ResourceNetworkError> {
        let remaining = deadline
            .monotonic_millis
            .saturating_sub(self.clock.monotonic_millis());
        if remaining == 0 {
            Err(ResourceNetworkError::Timeout)
        } else {
            Ok(Duration::from_millis(remaining))
        }
    }

    fn run_guarded<T>(
        &self,
        deadline: ResourceDeadline,
        cancellation: &Cancellation,
        future: impl Future<Output = Result<T, ResourceNetworkError>>,
    ) -> Result<T, ResourceNetworkError> {
        if cancellation.is_cancelled() {
            return Err(ResourceNetworkError::Cancelled);
        }
        let remaining = self.remaining(deadline)?;
        self.runtime.block_on(async {
            tokio::select! {
                biased;
                () = cancellation_signal(cancellation) => Err(ResourceNetworkError::Cancelled),
                () = tokio::time::sleep(remaining) => Err(ResourceNetworkError::Timeout),
                result = future => result,
            }
        })
    }

    fn pinned_client(
        request: &PinnedHttpsRequest,
        timeout: Duration,
    ) -> Result<Client, ResourceNetworkError> {
        if request.approved_addresses.is_empty() {
            return Err(ResourceNetworkError::Failed);
        }
        let sockets = request
            .approved_addresses
            .iter()
            .map(|address| SocketAddr::new(*address, request.port))
            .collect::<Vec<_>>();
        Client::builder()
            .redirect(Policy::none())
            .no_proxy()
            .https_only(true)
            .connect_timeout(timeout)
            .timeout(timeout)
            .user_agent(RESOURCE_USER_AGENT)
            .resolve_to_addrs(&request.host, &sockets)
            .build()
            .map_err(|_| ResourceNetworkError::Failed)
    }
}

impl ResourceNetwork for LinuxResourceNetwork {
    fn resolve(
        &self,
        request: &ResolveRequest,
        cancellation: &Cancellation,
    ) -> Result<Vec<IpAddr>, ResourceNetworkError> {
        let endpoint = (request.host.to_string(), request.port);
        self.run_guarded(request.deadline, cancellation, async move {
            let addresses = tokio::net::lookup_host(endpoint)
                .await
                .map_err(|_| ResourceNetworkError::Failed)?
                .map(|address| address.ip())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            if addresses.is_empty() {
                Err(ResourceNetworkError::NotFound)
            } else {
                Ok(addresses)
            }
        })
    }

    fn get(
        &self,
        request: &PinnedHttpsRequest,
        cancellation: &Cancellation,
    ) -> Result<RawHttpsResponse, ResourceNetworkError> {
        let remaining = self.remaining(request.deadline)?;
        let client = Self::pinned_client(request, remaining)?;
        let url = request.url.to_string();
        let maximum_body_bytes = request.maximum_body_bytes;
        self.run_guarded(request.deadline, cancellation, async move {
            let mut response = client.get(url).send().await.map_err(map_reqwest_error)?;
            if response
                .content_length()
                .is_some_and(|length| length > maximum_body_bytes as u64)
            {
                return Err(ResourceNetworkError::TooLarge);
            }
            let status = response.status().as_u16();
            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|value| value.to_str().ok())
                .map(Arc::<str>::from);
            let mut body = Vec::new();
            while let Some(chunk) = response.chunk().await.map_err(map_reqwest_error)? {
                if body.len().saturating_add(chunk.len()) > maximum_body_bytes {
                    return Err(ResourceNetworkError::TooLarge);
                }
                body.extend_from_slice(&chunk);
            }
            Ok(RawHttpsResponse {
                status,
                location,
                body,
            })
        })
    }
}

fn map_reqwest_error(error: reqwest::Error) -> ResourceNetworkError {
    if error.is_timeout() {
        ResourceNetworkError::Timeout
    } else {
        ResourceNetworkError::Failed
    }
}

async fn cancellation_signal(cancellation: &Cancellation) {
    loop {
        if cancellation.is_cancelled() {
            return;
        }
        tokio::time::sleep(CANCELLATION_POLL).await;
    }
}

#[derive(Debug)]
struct UnsupportedSvgRasterizer {
    clock: Arc<MonotonicResourceClock>,
}

impl SvgRasterizer for UnsupportedSvgRasterizer {
    fn rasterize(
        &self,
        request: &SvgRasterRequest,
        cancellation: &Cancellation,
    ) -> Result<RasterizedSvg, SvgRasterError> {
        if cancellation.is_cancelled() {
            return Err(SvgRasterError::Cancelled);
        }
        if self.clock.monotonic_millis() >= request.deadline.monotonic_millis {
            return Err(SvgRasterError::Timeout);
        }
        // This Linux POC deliberately exposes no file/network-capable SVG
        // renderer. Raster images still cross the hardened resource port.
        Err(SvgRasterError::DecodeFailed)
    }
}

pub(crate) fn linux_resource_provider() -> Result<Arc<dyn Provider>, String> {
    let clock = Arc::new(MonotonicResourceClock::new());
    let network = Arc::new(
        LinuxResourceNetwork::new(Arc::clone(&clock))
            .map_err(|error| format!("Linux resource runtime could not open: {error}"))?,
    );
    let limits = ResourceProviderLimits {
        fetch_timeout_millis: 12_000,
        ..ResourceProviderLimits::default()
    };
    let provider = ResourceProvider::new(
        network,
        Arc::new(UnsupportedSvgRasterizer {
            clock: Arc::clone(&clock),
        }),
        clock,
        Arc::new(NoopResourceActivity),
        limits,
        [
            Arc::<str>::from("https://cdn.hzrd149.com/"),
            Arc::<str>::from("https://blossom.ditto.pub/"),
        ],
    )
    .map_err(|error| format!("Linux resource provider could not open: {error}"))?;
    Ok(Arc::new(provider))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancelled_resolution_returns_without_network_work() {
        let clock = Arc::new(MonotonicResourceClock::new());
        let network = LinuxResourceNetwork::new(Arc::clone(&clock)).unwrap();
        let cancellation = Cancellation::new();
        cancellation.cancel();
        assert_eq!(
            network.resolve(
                &ResolveRequest {
                    host: Arc::from("does-not-resolve.invalid"),
                    port: 443,
                    deadline: ResourceDeadline {
                        monotonic_millis: clock.monotonic_millis() + 10_000,
                    },
                },
                &cancellation,
            ),
            Err(ResourceNetworkError::Cancelled)
        );
    }

    #[test]
    fn pinned_transport_refuses_an_empty_address_set() {
        let request = PinnedHttpsRequest {
            url: Arc::from("https://example.com/image.png"),
            host: Arc::from("example.com"),
            port: 443,
            approved_addresses: Arc::from([]),
            maximum_body_bytes: 1024,
            deadline: ResourceDeadline {
                monotonic_millis: 10_000,
            },
        };
        assert!(matches!(
            LinuxResourceNetwork::pinned_client(&request, Duration::from_secs(1)),
            Err(ResourceNetworkError::Failed)
        ));
    }
}
