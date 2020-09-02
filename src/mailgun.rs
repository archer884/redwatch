use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
struct Email<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub subject: &'a str,
    pub text: &'a str,
}

pub struct MailgunSender<'a> {
    client: &'a Client,
    endpoint: String,
    from: String,
    api_key: String,
}

impl<'a> MailgunSender<'a> {
    pub fn new(client: &'a Client) -> Self {
        dotenv::dotenv().ok();

        let domain = dotenv::var("MAILGUN_DOMAIN").unwrap();
        let api_key = dotenv::var("MAILGUN_API_KEY").unwrap();

        MailgunSender {
            client,
            endpoint: format!("https://api.mailgun.net/v3/{}/messages", domain),
            from: format!("redwatch-notify@{}", domain),
            api_key,
        }
    }
}

impl MailgunSender<'_> {
    pub fn send(&self, to: &str, subject: &str, text: &str) -> reqwest::Result<()> {
        let email = Email {
            from: &self.from,
            to,
            subject,
            text,
        };

        self.client
            .post(&self.endpoint)
            .form(&email)
            .basic_auth("api", Some(&self.api_key))
            .send()?;

        Ok(())
    }
}
