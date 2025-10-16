#![allow(unused)]

use super::super::errors::*;

use {
    kutil::std::error::*,
    problemo::{common::*, *},
    relative_path::*,
    std::{io, path::*},
};

/// Format from path.
///
/// We canonicalize common variations into a single representation.
pub fn get_format_from_path<PathT>(path: PathT) -> Option<String>
where
    PathT: Into<RelativePathBuf>,
{
    let path = path.into();

    let path_string = path.as_str();
    if path_string.ends_with(".txt") {
        return Some("text".into());
    } else if path_string.ends_with(".htm") {
        return Some("html".into());
    } else if path_string.ends_with(".jpg") {
        return Some("jpeg".into());
    } else if path_string.ends_with(".mpg") {
        return Some("mpeg".into());
    } else if path_string.ends_with(".yml") {
        return Some("yaml".into());
    } else if path_string.ends_with(".tar.gz") || path_string.ends_with(".tgz") {
        return Some("tar.gz".into());
    } else if path_string.ends_with(".tar.zst") || path_string.ends_with(".tar.zstd") {
        return Some("tar.zst".into());
    }

    path.extension().map(|extension| extension.into())
}

/// Path parent with trailing slash.
pub fn get_relative_path_parent<PathT>(path: PathT) -> Option<RelativePathBuf>
where
    PathT: Into<RelativePathBuf>,
{
    let path: RelativePathBuf = path.into();
    path.parent().map(|path| {
        let mut parent_path = path.to_relative_path_buf();

        // Avoid double trailing slashes (for root path)
        if !parent_path.as_str().ends_with("/") {
            // Add trailing slash
            parent_path.push("/");
        }

        parent_path
    })
}

/// Parse `archive!entry` URL.
pub fn parse_archive_entry_url_representation(
    url_representation: &str,
    required_scheme: &str,
) -> Result<(String, String), Problem> {
    let prefix = required_scheme.to_string() + ":";
    if !url_representation.starts_with(&prefix) {
        return Err(MalformedError::from(format!("scheme is not \"{}\": {}", required_scheme, url_representation))
            .into_problem()
            .via(UrlError)
            .with(UrlAttachment::new(url_representation)));
    }

    let mut archive_url_representation = None;
    let mut path = None;
    for (index, segment) in url_representation[prefix.len()..].rsplitn(2, "!").enumerate() {
        match index {
            0 => path = Some(segment),
            1 => archive_url_representation = Some(segment),
            _ => panic!("rsplitn(2): {} items?", index),
        }
    }

    if let Some(archive_url_representation) = archive_url_representation
        && let Some(path) = path
    {
        return Ok((archive_url_representation.into(), path.into()));
    }

    Err(MalformedError::from(format!("\"{}\" URL does not have a \"!\": {}", prefix, url_representation))
        .into_problem()
        .via(UrlError)
        .with(UrlAttachment::new(url_representation)))
}

/// Conform file path.
pub fn conform_file_path(path: &PathBuf) -> io::Result<PathBuf> {
    // We assume the archive URL has already been conformed

    let path = path.canonicalize().with_path(&path)?;

    if path.is_dir() {
        let mut path = path.into_os_string();
        path.push(MAIN_SEPARATOR_STR);
        return Ok(path.into());
    }

    Ok(path.into())
}
