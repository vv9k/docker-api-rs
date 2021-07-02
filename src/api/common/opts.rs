impl_url_opts_builder!(Logs);

impl LogsOptsBuilder {
    impl_url_bool_field!(follow => "follow");

    impl_url_bool_field!(stdout => "stdout");

    impl_url_bool_field!(stderr => "stderr");

    impl_url_bool_field!(timestamps => "timestamps");

    impl_url_str_field!(tail: N => "tail");

    #[cfg(feature = "chrono")]
    pub fn since<Tz>(&mut self, timestamp: &chrono::DateTime<Tz>) -> &mut Self
    where
        Tz: chrono::TimeZone,
    {
        self.params
            .insert("since", timestamp.timestamp().to_string());
        self
    }

    #[cfg(not(feature = "chrono"))]
    pub fn since(&mut self, timestamp: i64) -> &mut Self {
        self.params.insert("since", timestamp.to_string());
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
            .tail("all")
            .since(&since)
            .build();

        let serialized = options.serialize().unwrap();

        assert!(serialized.contains("follow=true"));
        assert!(serialized.contains("stdout=true"));
        assert!(serialized.contains("stderr=true"));
        assert!(serialized.contains("timestamps=true"));
        assert!(serialized.contains("tail=all"));
        assert!(serialized.contains("since=2147483647"));
    }

    #[cfg(not(feature = "chrono"))]
    #[test]
    fn logs_Opts() {
        let options = LogsOptsBuilder::default()
            .follow(true)
            .stdout(true)
            .stderr(true)
            .timestamps(true)
            .tail("all")
            .since(2_147_483_647)
            .build();

        let serialized = options.serialize().unwrap();

        assert!(serialized.contains("follow=true"));
        assert!(serialized.contains("stdout=true"));
        assert!(serialized.contains("stderr=true"));
        assert!(serialized.contains("timestamps=true"));
        assert!(serialized.contains("tail=all"));
        assert!(serialized.contains("since=2147483647"));
    }
}
