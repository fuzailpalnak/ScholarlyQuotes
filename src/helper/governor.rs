use actix_governor::governor::clock::QuantaInstant;
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};

use crate::utils::constants::RequestLimit;

pub fn create_governor_config() -> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>
{
    GovernorConfigBuilder::default()
        .requests_per_second(RequestLimit::RPS.as_u64())
        .burst_size(RequestLimit::BurstSize.as_u64() as u32)
        .finish()
        .unwrap()
}

pub fn create_governor<K, M>(governor_conf: &GovernorConfig<K, M>) -> Governor<K, M>
where
    K: actix_governor::KeyExtractor,
    M: actix_governor::governor::middleware::RateLimitingMiddleware<
        actix_governor::governor::clock::QuantaInstant,
    >,
{
    Governor::new(governor_conf)
}
