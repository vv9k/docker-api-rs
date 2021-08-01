pub mod url {
    use std::{borrow::Borrow, string::ToString};
    use url::form_urlencoded;

    pub fn construct_ep<E, Q>(ep: E, query: Option<Q>) -> String
    where
        E: Into<String>,
        Q: AsRef<str>,
    {
        let mut ep = ep.into();
        if let Some(query) = query {
            append_query(&mut ep, query);
        }
        ep
    }

    pub fn append_query<Q>(ep: &mut String, query: Q)
    where
        Q: AsRef<str>,
    {
        ep.push('?');
        ep.push_str(query.as_ref());
    }

    pub fn encoded_pair<K, V>(key: K, val: V) -> String
    where
        K: AsRef<str> + 'static,
        V: ToString,
    {
        form_urlencoded::Serializer::new(String::new())
            .append_pair(key.as_ref(), &val.to_string())
            .finish()
    }

    pub fn encoded_pairs<I, K, V>(iter: I) -> String
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        iter.into_iter()
            .fold(
                form_urlencoded::Serializer::new(String::new()),
                |mut acc, v| {
                    let &(ref k, ref v) = v.borrow();
                    let k = k.as_ref();
                    let v = v.as_ref();
                    if v.is_empty() {
                        acc.append_key_only(k);
                    } else {
                        acc.append_pair(k, v);
                    }
                    acc
                },
            )
            .finish()
    }
}

#[cfg(feature = "chrono")]
pub mod datetime {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;

    pub(crate) fn datetime_from_unix_timestamp<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let timestamp = chrono::NaiveDateTime::from_timestamp(i64::deserialize(deserializer)?, 0);
        Ok(DateTime::<Utc>::from_utc(timestamp, Utc))
    }

    pub(crate) fn datetime_from_nano_timestamp<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let timestamp_nano = u64::deserialize(deserializer)?;
        let timestamp = chrono::NaiveDateTime::from_timestamp(
            (timestamp_nano / 1_000_000_000) as i64,
            (timestamp_nano % 1_000_000_000) as u32,
        );
        Ok(DateTime::<Utc>::from_utc(timestamp, Utc))
    }
}

pub mod tarball {
    use flate2::{write::GzEncoder, Compression};
    use std::{
        fs::{self, File},
        io::{self, Write},
        path::{Path, MAIN_SEPARATOR},
    };
    use tar::Builder;

    /// Writes a gunzip encoded tarball to `buf` from entries found in `path`.
    pub fn dir<W, P>(buf: W, path: P) -> io::Result<()>
    where
        W: Write,
        P: AsRef<Path>,
    {
        let mut archive = Builder::new(GzEncoder::new(buf, Compression::best()));
        fn bundle<F>(dir: &Path, f: &mut F, bundle_dir: bool) -> io::Result<()>
        where
            F: FnMut(&Path) -> io::Result<()>,
        {
            if fs::metadata(dir)?.is_dir() {
                if bundle_dir {
                    f(dir)?;
                }
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    if fs::metadata(entry.path())?.is_dir() {
                        bundle(&entry.path(), f, true)?;
                    } else {
                        f(entry.path().as_path())?;
                    }
                }
            }
            Ok(())
        }

        {
            let path = path.as_ref();
            let base_path = path.canonicalize()?;
            let mut base_path_str = base_path
                .to_str()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid base path"))?
                .to_owned();
            if let Some(last) = base_path_str.chars().last() {
                if last != MAIN_SEPARATOR {
                    base_path_str.push(MAIN_SEPARATOR)
                }
            }

            let mut append = |path: &Path| {
                let canonical = path.canonicalize()?;
                let relativized = canonical
                    .to_str()
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "invalid canonicalized path")
                    })?
                    .trim_start_matches(&base_path_str[..]);
                if path.is_dir() {
                    archive.append_dir(Path::new(relativized), &canonical)?
                } else {
                    archive.append_file(Path::new(relativized), &mut File::open(&canonical)?)?
                }
                Ok(())
            };
            bundle(path, &mut append, false)?;
        }
        archive.finish()?;

        Ok(())
    }
}
