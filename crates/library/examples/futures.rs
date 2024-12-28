use {futures::*, read_url::*, tokio_util::compat::*};

// If you need to interact with Futures, you can use tokio_util::Compat

pub fn main() -> Result<(), UrlError> {
    // You *must* have a Tokio runtime to use read-url
    // (The tokio::main macro creates one automatically)
    let tokio = tokio::runtime::Builder::new_current_thread().enable_all().build()?;

    tokio.block_on(main_async())
}

pub async fn main_async() -> Result<(), UrlError> {
    let context = UrlContext::new();

    let url = context
        .url_async("https://raw.githubusercontent.com/tliron/rust-read-url/refs/heads/main/assets/files/two.txt")
        .await?;

    let reader = url.open_async()?.await?; // tokio::io::AsyncRead

    // Compat magic

    read(&mut reader.compat()).await?;

    Ok(())
}

// This is a Futures function that doesn't know anything about Tokio

pub async fn read<ReadT>(reader: &mut ReadT) -> io::Result<()>
where
    ReadT: io::AsyncRead + Unpin,
{
    let mut string = String::new();
    reader.read_to_string(&mut string).await?;
    println!("{:?}", string);

    Ok(())
}
