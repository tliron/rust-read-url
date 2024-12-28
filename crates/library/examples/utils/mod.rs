#![allow(unused)]

use {
    anstream::println,
    owo_colors::*,
    read_url::*,
    std::result::Result,
    tokio::{io::*, *},
};

pub fn heading(heading: &str) {
    println!("\n{}:", heading.green());
}

pub fn about(url: &UrlRef) {
    println!("  {:12}{}", "URL:".blue(), url);

    if let Some(query) = url.query() {
        println!("    {:10}{:?}", "Query:".blue(), query);
    }

    if let Some(fragment) = url.fragment() {
        println!("    {:10}{}", "Fragment:".blue(), fragment);
    }

    if let Some(base) = url.base() {
        println!("    {:10}{}", "Base:".blue(), base);
    }

    if let Some(format) = url.format() {
        println!("    {:10}{}", "Format:".blue(), format);
    }
}

pub fn dump(url: &UrlRef) -> Result<(), UrlError> {
    about(url);

    let mut reader = url.open()?; // io::Read
    let mut string = String::new();
    reader.read_to_string(&mut string)?;
    println!("    {:10}{:?}", "Content:".blue(), string);

    Ok(())
}

pub async fn dump_async(url: &UrlRef) -> Result<(), UrlError> {
    about(url);

    let mut reader = url.open_async()?.await?; // tokio::io::AsyncRead
    let mut string = String::new();
    reader.read_to_string(&mut string).await?;
    println!("    {:10}{:?}", "Content:".blue(), string);

    Ok(())
}
