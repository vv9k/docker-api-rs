use crate::{Error, Result};

use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiVersion {
    major: usize,
    minor: usize,
}

impl ApiVersion {
    pub const fn new(major: usize, minor: usize) -> Self {
        Self { major, minor }
    }

    pub fn make_endpoint(&self, ep: impl AsRef<str>) -> String {
        // As noted in [Versioning](https://docs.docker.com/engine/api/v1.41/#section/Versioning), all requests
        // should be prefixed with the API version as the ones without will stop being supported in future releases

        let ep = ep.as_ref();
        format!(
            "/v{}{}{}",
            self,
            if !ep.starts_with('/') { "/" } else { "" },
            ep
        )
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl From<(usize, usize)> for ApiVersion {
    fn from(v: (usize, usize)) -> Self {
        ApiVersion {
            major: v.0,
            minor: v.1,
        }
    }
}

impl FromStr for ApiVersion {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut elems = s.split('.');
        let major = if let Some(maj) = elems.next() {
            match maj.parse::<usize>() {
                Ok(maj) => maj,
                Err(e) => return Err(Error::MalformedVersion(e.to_string())),
            }
        } else {
            return Err(Error::MalformedVersion(s.to_string()));
        };
        let minor = if let Some(min) = elems.next() {
            match min.parse::<usize>() {
                Ok(min) => min,
                Err(e) => return Err(Error::MalformedVersion(e.to_string())),
            }
        } else {
            return Err(Error::MalformedVersion(
                "expected minor version".to_string(),
            ));
        };
        if elems.next().is_some() {
            return Err(Error::MalformedVersion(
                "unexpected extra tokens".to_string(),
            ));
        }
        Ok(Self { major, minor })
    }
}

#[cfg(test)]
mod tests {
    use super::ApiVersion;

    #[test]
    fn compares_versions() {
        assert_eq!(ApiVersion::new(1, 40), ApiVersion::new(1, 40));
        assert!(ApiVersion::new(1, 40) < ApiVersion::new(1, 41));
        assert!(ApiVersion::new(0, 41) < ApiVersion::new(1, 40));
        assert!(ApiVersion::new(2, 0) > ApiVersion::new(1, 41));
    }
}
