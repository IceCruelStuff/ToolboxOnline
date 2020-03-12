use crate::AppAuthenticator;
use reqwest::header::AUTHORIZATION;
use yup_oauth2::AccessToken;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::fmt::Display;
use serde::de;

pub struct AndroidPublisherApi {
    pub token: AccessToken
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionInfo {
    #[serde(deserialize_with = "from_str")]
    pub expiry_time_millis: u64
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

impl AndroidPublisherApi {

    pub fn new(token: AccessToken) -> AndroidPublisherApi {
        return AndroidPublisherApi { token }
    }

    pub async fn from_authenticator(authenticator: &AppAuthenticator) -> Result<AndroidPublisherApi, yup_oauth2::Error> {
        let token = authenticator.token(&["https://www.googleapis.com/auth/androidpublisher"]).await?;
        Ok(AndroidPublisherApi::new(token))
    }

    pub async fn get_subscription_info(self: &AndroidPublisherApi, app_id: &str, sub_id: &str, user_token: &str) -> Result<SubscriptionInfo, reqwest::Error> {
        reqwest::Client::new()
            .get(&["https://www.googleapis.com/androidpublisher/v3/applications/", app_id, "/purchases/subscriptions/", sub_id, "/tokens/", user_token].concat())
            .header(AUTHORIZATION, "Bearer ".to_owned() + self.token.as_str())
            .send()
            .await?
            .json()
            .await
    }

}