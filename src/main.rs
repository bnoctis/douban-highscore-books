use fantoccini::{ Client as FClient, Locator as FLocator};
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut c = FClient::new("http://localhost:4444").await.expect("failed to connect to WebDriver");

    // first, go to the Wikipedia page for Foobar
    c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

    // click "Foo (disambiguation)"
    c.find(FLocator::Css(".mw-disambig")).await?.click().await?;

    // click "Foo Lake"
    c.find(FLocator::LinkText("Foo Lake")).await?.click().await?;

    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    c.close().await
}