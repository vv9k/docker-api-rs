impl_url_opts_builder!(Logs);

impl LogsOptsBuilder {
    impl_url_bool_field!(
        /// Keep connection after returning logs.
        follow => "follow"
    );

    impl_url_bool_field!(
        /// Return logs from `stdout`.
        stdout => "stdout"
    );

    impl_url_bool_field!(
        /// Return logs from `stderr`.
        stderr => "stderr"
    );

    impl_url_bool_field!(
        /// Add timestamps to every log line.
        timestamps => "timestamps"
    );

    impl_url_field!(
        /// Only return this number of log lines from the end of logs
        n_lines: usize => "tail"
    );

    /// Return all log lines.
    pub fn all(&mut self) -> &mut Self {
        self.params.insert("tail", "all".into());
        self
    }

    #[cfg(feature = "chrono")]
    /// Only return logs since this time.
    pub fn since<Tz>(&mut self, timestamp: &chrono::DateTime<Tz>) -> &mut Self
    where
        Tz: chrono::TimeZone,
    {
        self.params
            .insert("since", timestamp.timestamp().to_string());
        self
    }

    #[cfg(not(feature = "chrono"))]
    /// Only return logs since this time, as a UNIX timestamp.
    pub fn since(&mut self, timestamp: i64) -> &mut Self {
        self.params.insert("since", timestamp.to_string());
        self
    }

    #[cfg(feature = "chrono")]
    /// Only return logs before this time.
    pub fn until<Tz>(&mut self, timestamp: &chrono::DateTime<Tz>) -> &mut Self
    where
        Tz: chrono::TimeZone,
    {
        self.params
            .insert("until", timestamp.timestamp().to_string());
        self
    }

    #[cfg(not(feature = "chrono"))]
    /// Only return logs before this time, as a UNIX timestamp.
    pub fn until(&mut self, timestamp: i64) -> &mut Self {
        self.params.insert("until", timestamp.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "chrono")]
    #[test]
    fn logs_options() {
        let timestamp = chrono::NaiveDateTime::from_timestamp(2_147_483_647, 0);
        let since = chrono::DateTime::<chrono::Utc>::from_utc(timestamp, chrono::Utc);

        let options = LogsOptsBuilder::default()
            .follow(true)
            .stdout(true)
            .stderr(true)
            .timestamps(true)
            .all()
            .since(&since)
            .build();

        let serialized = options.serialize().unwrap();

        assert!(serialized.contains("follow=true"));
        assert!(serialized.contains("stdout=true"));
        assert!(serialized.contains("stderr=true"));
        assert!(serialized.contains("timestamps=true"));
        assert!(serialized.contains("tail=all"));
        assert!(serialized.contains("since=2147483647"));

        let options = LogsOptsBuilder::default().n_lines(5).until(&since).build();

        let serialized = options.serialize().unwrap();

        assert!(serialized.contains("tail=5"));
        assert!(serialized.contains("until=2147483647"));
    }

    #[cfg(not(feature = "chrono"))]
    #[test]
    fn logs_options() {
        let options = LogsOptsBuilder::default()
            .follow(true)
            .stdout(true)
            .stderr(true)
            .timestamps(true)
            .all()
            .since(2_147_483_647)
            .until(2_147_600_000)
            .build();

        let serialized = options.serialize().unwrap();

        assert!(serialized.contains("follow=true"));
        assert!(serialized.contains("stdout=true"));
        assert!(serialized.contains("stderr=true"));
        assert!(serialized.contains("timestamps=true"));
        assert!(serialized.contains("tail=all"));
        assert!(serialized.contains("since=2147483647"));
        assert!(serialized.contains("until=2147600000"));
    }
}
