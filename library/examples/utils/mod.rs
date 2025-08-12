#![allow(unused)]

use {
    anstream::println,
    kutil::cli::debug::*,
    read_url::*,
    std::result::Result,
    tokio::{io::*, *},
};

pub fn heading(heading: &str, first: bool) {
    let theme = Theme::default();
    if !first {
        println!();
    }
    println!("{}:", theme.heading(heading));
}

pub fn about(url: &UrlRef) {
    let theme = Theme::default();

    println!("  {:12}{}", theme.meta("URL:"), url);

    if let Some(query) = url.query() {
        println!("    {:10}{:?}", theme.meta("Query:"), query);
    }

    if let Some(fragment) = url.fragment() {
        println!("    {:10}{}", theme.meta("Fragment:"), fragment);
    }

    if let Some(base) = url.base() {
        println!("    {:10}{}", theme.meta("Base:"), base);
    }

    if let Some(format) = url.format() {
        println!("    {:10}{}", theme.meta("Format:"), format);
    }
}

pub fn dump(url: &UrlRef) -> Result<(), UrlError> {
    about(url);

    let theme = Theme::default();

    let mut reader = url.open()?; // io::Read
    let mut string = String::default();
    reader.read_to_string(&mut string)?;
    println!("    {:10}{:?}", theme.meta("Content:"), string);

    Ok(())
}

pub async fn dump_async(url: &UrlRef) -> Result<(), UrlError> {
    about(url);

    let theme = Theme::default();

    let mut reader = url.open_async()?.await?; // tokio::io::AsyncRead
    let mut string = String::default();
    reader.read_to_string(&mut string).await?;
    println!("    {:10}{:?}", theme.meta("Content:"), string);

    Ok(())
}
