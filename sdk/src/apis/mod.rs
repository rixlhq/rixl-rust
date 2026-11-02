use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod api_keys_api;
pub mod access_policies_api;
pub mod audio_tracks_api;
pub mod blog_api;
pub mod chapters_api;
pub mod client_credentials_api;
pub mod custom_domains_api;
pub mod dashboards_api;
pub mod email_api;
pub mod events_api;
pub mod feed_analytics_api;
pub mod feeds_api;
pub mod funnels_api;
pub mod heatmaps_api;
pub mod image_analytics_api;
pub mod image_conversion_api;
pub mod images_api;
pub mod invoices_api;
pub mod languages_api;
pub mod memberships_api;
pub mod one_time_passcodes_api;
pub mod passkeys_api;
pub mod payments_api;
pub mod plans_api;
pub mod platform_auth_api;
pub mod post_analytics_api;
pub mod posts_api;
pub mod projects_api;
pub mod realtime_api;
pub mod sales_api;
pub mod sessions_api;
pub mod social_providers_api;
pub mod subscriptions_api;
pub mod subtitles_api;
pub mod support_chat_api;
pub mod support_tickets_api;
pub mod usage_api;
pub mod users_api;
pub mod video_analytics_api;
pub mod video_conversion_api;
pub mod videos_api;
pub mod webhooks_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn api_keys_api(&self) -> &dyn api_keys_api::ApiKeysApi;
    fn access_policies_api(&self) -> &dyn access_policies_api::AccessPoliciesApi;
    fn audio_tracks_api(&self) -> &dyn audio_tracks_api::AudioTracksApi;
    fn blog_api(&self) -> &dyn blog_api::BlogApi;
    fn chapters_api(&self) -> &dyn chapters_api::ChaptersApi;
    fn client_credentials_api(&self) -> &dyn client_credentials_api::ClientCredentialsApi;
    fn custom_domains_api(&self) -> &dyn custom_domains_api::CustomDomainsApi;
    fn dashboards_api(&self) -> &dyn dashboards_api::DashboardsApi;
    fn email_api(&self) -> &dyn email_api::EmailApi;
    fn events_api(&self) -> &dyn events_api::EventsApi;
    fn feed_analytics_api(&self) -> &dyn feed_analytics_api::FeedAnalyticsApi;
    fn feeds_api(&self) -> &dyn feeds_api::FeedsApi;
    fn funnels_api(&self) -> &dyn funnels_api::FunnelsApi;
    fn heatmaps_api(&self) -> &dyn heatmaps_api::HeatmapsApi;
    fn image_analytics_api(&self) -> &dyn image_analytics_api::ImageAnalyticsApi;
    fn image_conversion_api(&self) -> &dyn image_conversion_api::ImageConversionApi;
    fn images_api(&self) -> &dyn images_api::ImagesApi;
    fn invoices_api(&self) -> &dyn invoices_api::InvoicesApi;
    fn languages_api(&self) -> &dyn languages_api::LanguagesApi;
    fn memberships_api(&self) -> &dyn memberships_api::MembershipsApi;
    fn one_time_passcodes_api(&self) -> &dyn one_time_passcodes_api::OneTimePasscodesApi;
    fn passkeys_api(&self) -> &dyn passkeys_api::PasskeysApi;
    fn payments_api(&self) -> &dyn payments_api::PaymentsApi;
    fn plans_api(&self) -> &dyn plans_api::PlansApi;
    fn platform_auth_api(&self) -> &dyn platform_auth_api::PlatformAuthApi;
    fn post_analytics_api(&self) -> &dyn post_analytics_api::PostAnalyticsApi;
    fn posts_api(&self) -> &dyn posts_api::PostsApi;
    fn projects_api(&self) -> &dyn projects_api::ProjectsApi;
    fn realtime_api(&self) -> &dyn realtime_api::RealtimeApi;
    fn sales_api(&self) -> &dyn sales_api::SalesApi;
    fn sessions_api(&self) -> &dyn sessions_api::SessionsApi;
    fn social_providers_api(&self) -> &dyn social_providers_api::SocialProvidersApi;
    fn subscriptions_api(&self) -> &dyn subscriptions_api::SubscriptionsApi;
    fn subtitles_api(&self) -> &dyn subtitles_api::SubtitlesApi;
    fn support_chat_api(&self) -> &dyn support_chat_api::SupportChatApi;
    fn support_tickets_api(&self) -> &dyn support_tickets_api::SupportTicketsApi;
    fn usage_api(&self) -> &dyn usage_api::UsageApi;
    fn users_api(&self) -> &dyn users_api::UsersApi;
    fn video_analytics_api(&self) -> &dyn video_analytics_api::VideoAnalyticsApi;
    fn video_conversion_api(&self) -> &dyn video_conversion_api::VideoConversionApi;
    fn videos_api(&self) -> &dyn videos_api::VideosApi;
    fn webhooks_api(&self) -> &dyn webhooks_api::WebhooksApi;
}

pub struct ApiClient {
    api_keys_api: Box<dyn api_keys_api::ApiKeysApi>,
    access_policies_api: Box<dyn access_policies_api::AccessPoliciesApi>,
    audio_tracks_api: Box<dyn audio_tracks_api::AudioTracksApi>,
    blog_api: Box<dyn blog_api::BlogApi>,
    chapters_api: Box<dyn chapters_api::ChaptersApi>,
    client_credentials_api: Box<dyn client_credentials_api::ClientCredentialsApi>,
    custom_domains_api: Box<dyn custom_domains_api::CustomDomainsApi>,
    dashboards_api: Box<dyn dashboards_api::DashboardsApi>,
    email_api: Box<dyn email_api::EmailApi>,
    events_api: Box<dyn events_api::EventsApi>,
    feed_analytics_api: Box<dyn feed_analytics_api::FeedAnalyticsApi>,
    feeds_api: Box<dyn feeds_api::FeedsApi>,
    funnels_api: Box<dyn funnels_api::FunnelsApi>,
    heatmaps_api: Box<dyn heatmaps_api::HeatmapsApi>,
    image_analytics_api: Box<dyn image_analytics_api::ImageAnalyticsApi>,
    image_conversion_api: Box<dyn image_conversion_api::ImageConversionApi>,
    images_api: Box<dyn images_api::ImagesApi>,
    invoices_api: Box<dyn invoices_api::InvoicesApi>,
    languages_api: Box<dyn languages_api::LanguagesApi>,
    memberships_api: Box<dyn memberships_api::MembershipsApi>,
    one_time_passcodes_api: Box<dyn one_time_passcodes_api::OneTimePasscodesApi>,
    passkeys_api: Box<dyn passkeys_api::PasskeysApi>,
    payments_api: Box<dyn payments_api::PaymentsApi>,
    plans_api: Box<dyn plans_api::PlansApi>,
    platform_auth_api: Box<dyn platform_auth_api::PlatformAuthApi>,
    post_analytics_api: Box<dyn post_analytics_api::PostAnalyticsApi>,
    posts_api: Box<dyn posts_api::PostsApi>,
    projects_api: Box<dyn projects_api::ProjectsApi>,
    realtime_api: Box<dyn realtime_api::RealtimeApi>,
    sales_api: Box<dyn sales_api::SalesApi>,
    sessions_api: Box<dyn sessions_api::SessionsApi>,
    social_providers_api: Box<dyn social_providers_api::SocialProvidersApi>,
    subscriptions_api: Box<dyn subscriptions_api::SubscriptionsApi>,
    subtitles_api: Box<dyn subtitles_api::SubtitlesApi>,
    support_chat_api: Box<dyn support_chat_api::SupportChatApi>,
    support_tickets_api: Box<dyn support_tickets_api::SupportTicketsApi>,
    usage_api: Box<dyn usage_api::UsageApi>,
    users_api: Box<dyn users_api::UsersApi>,
    video_analytics_api: Box<dyn video_analytics_api::VideoAnalyticsApi>,
    video_conversion_api: Box<dyn video_conversion_api::VideoConversionApi>,
    videos_api: Box<dyn videos_api::VideosApi>,
    webhooks_api: Box<dyn webhooks_api::WebhooksApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            api_keys_api: Box::new(api_keys_api::ApiKeysApiClient::new(configuration.clone())),
            access_policies_api: Box::new(access_policies_api::AccessPoliciesApiClient::new(configuration.clone())),
            audio_tracks_api: Box::new(audio_tracks_api::AudioTracksApiClient::new(configuration.clone())),
            blog_api: Box::new(blog_api::BlogApiClient::new(configuration.clone())),
            chapters_api: Box::new(chapters_api::ChaptersApiClient::new(configuration.clone())),
            client_credentials_api: Box::new(client_credentials_api::ClientCredentialsApiClient::new(configuration.clone())),
            custom_domains_api: Box::new(custom_domains_api::CustomDomainsApiClient::new(configuration.clone())),
            dashboards_api: Box::new(dashboards_api::DashboardsApiClient::new(configuration.clone())),
            email_api: Box::new(email_api::EmailApiClient::new(configuration.clone())),
            events_api: Box::new(events_api::EventsApiClient::new(configuration.clone())),
            feed_analytics_api: Box::new(feed_analytics_api::FeedAnalyticsApiClient::new(configuration.clone())),
            feeds_api: Box::new(feeds_api::FeedsApiClient::new(configuration.clone())),
            funnels_api: Box::new(funnels_api::FunnelsApiClient::new(configuration.clone())),
            heatmaps_api: Box::new(heatmaps_api::HeatmapsApiClient::new(configuration.clone())),
            image_analytics_api: Box::new(image_analytics_api::ImageAnalyticsApiClient::new(configuration.clone())),
            image_conversion_api: Box::new(image_conversion_api::ImageConversionApiClient::new(configuration.clone())),
            images_api: Box::new(images_api::ImagesApiClient::new(configuration.clone())),
            invoices_api: Box::new(invoices_api::InvoicesApiClient::new(configuration.clone())),
            languages_api: Box::new(languages_api::LanguagesApiClient::new(configuration.clone())),
            memberships_api: Box::new(memberships_api::MembershipsApiClient::new(configuration.clone())),
            one_time_passcodes_api: Box::new(one_time_passcodes_api::OneTimePasscodesApiClient::new(configuration.clone())),
            passkeys_api: Box::new(passkeys_api::PasskeysApiClient::new(configuration.clone())),
            payments_api: Box::new(payments_api::PaymentsApiClient::new(configuration.clone())),
            plans_api: Box::new(plans_api::PlansApiClient::new(configuration.clone())),
            platform_auth_api: Box::new(platform_auth_api::PlatformAuthApiClient::new(configuration.clone())),
            post_analytics_api: Box::new(post_analytics_api::PostAnalyticsApiClient::new(configuration.clone())),
            posts_api: Box::new(posts_api::PostsApiClient::new(configuration.clone())),
            projects_api: Box::new(projects_api::ProjectsApiClient::new(configuration.clone())),
            realtime_api: Box::new(realtime_api::RealtimeApiClient::new(configuration.clone())),
            sales_api: Box::new(sales_api::SalesApiClient::new(configuration.clone())),
            sessions_api: Box::new(sessions_api::SessionsApiClient::new(configuration.clone())),
            social_providers_api: Box::new(social_providers_api::SocialProvidersApiClient::new(configuration.clone())),
            subscriptions_api: Box::new(subscriptions_api::SubscriptionsApiClient::new(configuration.clone())),
            subtitles_api: Box::new(subtitles_api::SubtitlesApiClient::new(configuration.clone())),
            support_chat_api: Box::new(support_chat_api::SupportChatApiClient::new(configuration.clone())),
            support_tickets_api: Box::new(support_tickets_api::SupportTicketsApiClient::new(configuration.clone())),
            usage_api: Box::new(usage_api::UsageApiClient::new(configuration.clone())),
            users_api: Box::new(users_api::UsersApiClient::new(configuration.clone())),
            video_analytics_api: Box::new(video_analytics_api::VideoAnalyticsApiClient::new(configuration.clone())),
            video_conversion_api: Box::new(video_conversion_api::VideoConversionApiClient::new(configuration.clone())),
            videos_api: Box::new(videos_api::VideosApiClient::new(configuration.clone())),
            webhooks_api: Box::new(webhooks_api::WebhooksApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn api_keys_api(&self) -> &dyn api_keys_api::ApiKeysApi {
        self.api_keys_api.as_ref()
    }
    fn access_policies_api(&self) -> &dyn access_policies_api::AccessPoliciesApi {
        self.access_policies_api.as_ref()
    }
    fn audio_tracks_api(&self) -> &dyn audio_tracks_api::AudioTracksApi {
        self.audio_tracks_api.as_ref()
    }
    fn blog_api(&self) -> &dyn blog_api::BlogApi {
        self.blog_api.as_ref()
    }
    fn chapters_api(&self) -> &dyn chapters_api::ChaptersApi {
        self.chapters_api.as_ref()
    }
    fn client_credentials_api(&self) -> &dyn client_credentials_api::ClientCredentialsApi {
        self.client_credentials_api.as_ref()
    }
    fn custom_domains_api(&self) -> &dyn custom_domains_api::CustomDomainsApi {
        self.custom_domains_api.as_ref()
    }
    fn dashboards_api(&self) -> &dyn dashboards_api::DashboardsApi {
        self.dashboards_api.as_ref()
    }
    fn email_api(&self) -> &dyn email_api::EmailApi {
        self.email_api.as_ref()
    }
    fn events_api(&self) -> &dyn events_api::EventsApi {
        self.events_api.as_ref()
    }
    fn feed_analytics_api(&self) -> &dyn feed_analytics_api::FeedAnalyticsApi {
        self.feed_analytics_api.as_ref()
    }
    fn feeds_api(&self) -> &dyn feeds_api::FeedsApi {
        self.feeds_api.as_ref()
    }
    fn funnels_api(&self) -> &dyn funnels_api::FunnelsApi {
        self.funnels_api.as_ref()
    }
    fn heatmaps_api(&self) -> &dyn heatmaps_api::HeatmapsApi {
        self.heatmaps_api.as_ref()
    }
    fn image_analytics_api(&self) -> &dyn image_analytics_api::ImageAnalyticsApi {
        self.image_analytics_api.as_ref()
    }
    fn image_conversion_api(&self) -> &dyn image_conversion_api::ImageConversionApi {
        self.image_conversion_api.as_ref()
    }
    fn images_api(&self) -> &dyn images_api::ImagesApi {
        self.images_api.as_ref()
    }
    fn invoices_api(&self) -> &dyn invoices_api::InvoicesApi {
        self.invoices_api.as_ref()
    }
    fn languages_api(&self) -> &dyn languages_api::LanguagesApi {
        self.languages_api.as_ref()
    }
    fn memberships_api(&self) -> &dyn memberships_api::MembershipsApi {
        self.memberships_api.as_ref()
    }
    fn one_time_passcodes_api(&self) -> &dyn one_time_passcodes_api::OneTimePasscodesApi {
        self.one_time_passcodes_api.as_ref()
    }
    fn passkeys_api(&self) -> &dyn passkeys_api::PasskeysApi {
        self.passkeys_api.as_ref()
    }
    fn payments_api(&self) -> &dyn payments_api::PaymentsApi {
        self.payments_api.as_ref()
    }
    fn plans_api(&self) -> &dyn plans_api::PlansApi {
        self.plans_api.as_ref()
    }
    fn platform_auth_api(&self) -> &dyn platform_auth_api::PlatformAuthApi {
        self.platform_auth_api.as_ref()
    }
    fn post_analytics_api(&self) -> &dyn post_analytics_api::PostAnalyticsApi {
        self.post_analytics_api.as_ref()
    }
    fn posts_api(&self) -> &dyn posts_api::PostsApi {
        self.posts_api.as_ref()
    }
    fn projects_api(&self) -> &dyn projects_api::ProjectsApi {
        self.projects_api.as_ref()
    }
    fn realtime_api(&self) -> &dyn realtime_api::RealtimeApi {
        self.realtime_api.as_ref()
    }
    fn sales_api(&self) -> &dyn sales_api::SalesApi {
        self.sales_api.as_ref()
    }
    fn sessions_api(&self) -> &dyn sessions_api::SessionsApi {
        self.sessions_api.as_ref()
    }
    fn social_providers_api(&self) -> &dyn social_providers_api::SocialProvidersApi {
        self.social_providers_api.as_ref()
    }
    fn subscriptions_api(&self) -> &dyn subscriptions_api::SubscriptionsApi {
        self.subscriptions_api.as_ref()
    }
    fn subtitles_api(&self) -> &dyn subtitles_api::SubtitlesApi {
        self.subtitles_api.as_ref()
    }
    fn support_chat_api(&self) -> &dyn support_chat_api::SupportChatApi {
        self.support_chat_api.as_ref()
    }
    fn support_tickets_api(&self) -> &dyn support_tickets_api::SupportTicketsApi {
        self.support_tickets_api.as_ref()
    }
    fn usage_api(&self) -> &dyn usage_api::UsageApi {
        self.usage_api.as_ref()
    }
    fn users_api(&self) -> &dyn users_api::UsersApi {
        self.users_api.as_ref()
    }
    fn video_analytics_api(&self) -> &dyn video_analytics_api::VideoAnalyticsApi {
        self.video_analytics_api.as_ref()
    }
    fn video_conversion_api(&self) -> &dyn video_conversion_api::VideoConversionApi {
        self.video_conversion_api.as_ref()
    }
    fn videos_api(&self) -> &dyn videos_api::VideosApi {
        self.videos_api.as_ref()
    }
    fn webhooks_api(&self) -> &dyn webhooks_api::WebhooksApi {
        self.webhooks_api.as_ref()
    }
}


