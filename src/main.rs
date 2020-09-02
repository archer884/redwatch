mod mailgun;
mod reddit;

use hashbrown::HashSet;
use mailgun::MailgunSender;
use reddit::{Post, Response};
use reqwest::blocking::Client;
use std::{borrow::Borrow, time::Duration};
use std::{str, thread};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
struct Opt {
    /// Subreddits to be watched
    pub subreddit: String,
    /// Comma-delimited list of email addresses
    pub email: String,
    /// Comma-delimited list of keywords
    pub whitelist: String,
}

impl Opt {
    fn emails(&self) -> String {
        let mut emails = self.email.split(',');
        let mut buf = String::new();

        if let Some(email) = emails.next() {
            buf.push_str(email.trim());
        }

        for email in emails {
            buf.push_str(", ");
            buf.push_str(email);
        }

        buf
    }

    fn whitelist(&self) -> str::Split<char> {
        self.whitelist.split(',')
    }
}

fn main() -> reqwest::Result<()> {
    const WAIT_DURATION: Duration = Duration::from_secs(5 * 60);

    let opt = Opt::from_args();
    let client = build_client();
    let url = format_url(&opt.subreddit);
    let emails = opt.emails();
    let whitelist: HashSet<_> = opt.whitelist().collect();
    let mailgun = MailgunSender::new(&client);

    let mut last_set = HashSet::new();

    loop {
        let response: Response = client.get(&url).send()?.json()?;
        let mut keywords = Vec::new();
        let posts: Vec<_> = response
            .posts()
            .filter(|&post| !last_set.contains(&post.id))
            .filter(|&post| {
                post.keywords().any(|x| {
                    if let Some(&keyword) = whitelist.get(&*x) {
                        keywords.push(keyword);
                        true
                    } else {
                        false
                    }
                })
            })
            .collect();

        notify(&mailgun, &emails, &keywords, posts)?;

        last_set = response.ids.into_iter().collect();
        thread::sleep(WAIT_DURATION);
    }
}

fn notify<T: Borrow<Post>>(
    mailgun: &MailgunSender,
    emails: &str,
    keywords: &[&str],
    posts: impl IntoIterator<Item = T>,
) -> reqwest::Result<()> {
    fn build_subject(keywords: &[&str]) -> String {
        keywords.join(", ")
    }

    fn build_text<T: Borrow<Post>>(posts: impl IntoIterator<Item = T>) -> String {
        use std::fmt::Write;

        let mut links = String::new();
        for post in posts {
            write!(links, "{}\n", post.borrow().permalink).unwrap();
        }
        links
    }

    let subject = build_subject(keywords);
    let text = build_text(posts);

    mailgun.send(emails, &subject, &text)?;
    Ok(())
}

fn format_url(subreddit: &str) -> String {
    format!(
        "https://gateway.reddit.com/desktopapi/v1/subreddits/{}?rtj=only\
        &redditWebClient=web2x\
        &app=web2x-client-production\
        &allow_over18=1\
        &allow_quarantined=true\
        &include=identity\
        &dist=25&layout=card&sort=new&geo_filter=GLOBAL",
        subreddit
    )
}

// This code is not strictly necessary.
// fn add_after(url: &mut String, after: &str) {
//     use std::fmt::Write;
//     write!(url, "&after={}", after).unwrap();
// }

fn build_client() -> Client {
    static USER_AGENT: &str =
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:80.0) Gecko/20100101 Firefox/80.0";

    Client::builder().user_agent(USER_AGENT).build().unwrap()
}
