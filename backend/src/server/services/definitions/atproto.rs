use crate::server::hosts::r#impl::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ATProto;

impl ServiceDefinition for ATProto {
    fn name(&self) -> &'static str {
        "AT Protocol Personal Data Server"
    }
    fn description(&self) -> &'static str {
        "Distributed Social Networking Server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::SocialNetwork
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortBase::Http3000,
            "/",
            "This is an AT Protocol Personal Data Server",
            None,
        )
    }
    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/bluesky.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<ATProto>));
