# redwatch

A reddit watching program. Redwatch sends email notifications to your inbox when it finds a keywords in a post in a watched subreddit.

```shell
$ redwatch --help
redwatch 0.1.0
Watches a subreddit for keywords. Requires the following environment variables: MAILGUN_DOMAIN, MAILGUN_API_KEY

USAGE:
    redwatch.exe <subreddit> <email> <whitelist>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <subreddit>    Subreddits to be watched
    <email>        Comma-delimited list of email addresses
    <whitelist>    Comma-delimited list of keywords
```

As mentioned above, the program requires both a sender domain and an API key for Mailgun, which is the service used to send emails.
