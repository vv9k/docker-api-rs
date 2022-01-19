macro_rules! impl_vec_field {
    ($(#[doc = $docs:expr])* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name  >]<[< $ty >], S>(mut self, $name: $ty)-> Self
            where
                $ty: IntoIterator<Item = S>,
                S: AsRef<str> + serde::Serialize
            {
                self.params.insert($docker_name, serde_json::json!($name.into_iter().collect::<Vec<_>>()));
                self
            }
        }
    };
}

macro_rules! impl_field {
    ($(#[doc = $docs:expr])* $name:ident: $ty:ty => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: $ty)-> Self
            {
                self.params.insert($docker_name, serde_json::json!($name));
                self
            }
        }
    };
}

macro_rules! impl_str_field {
    ($(#[doc = $docs:expr])* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >]<[< $ty >]>(mut self, $name: $ty)-> Self
            where
                $ty: AsRef<str> + serde::Serialize,
            {
                self.params.insert($docker_name, serde_json::json!($name.as_ref()));
                self
            }
        }
    };
}

macro_rules! impl_url_str_field {
    ($(#[doc = $docs:expr])* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >]<[< $ty >]>(mut self, $name: $ty)-> Self
            where
                $ty: Into<String>,
            {
                self.params.insert($docker_name, $name.into());
                self
            }
        }
    };
}

macro_rules! impl_url_field {
    ($(#[doc = $docs:expr])* $name:ident : $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: $ty)-> Self {
                self.params.insert($docker_name, $name.to_string());
                self
            }
        }
    };
}

macro_rules! impl_url_bool_field {
    ($(#[doc = $docs:expr])* $name:ident => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: bool)-> Self {
                self.params.insert($docker_name, $name.to_string());
                self
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_str_enum_field {
    ($(#[doc = $docs:expr])* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: $ty)-> Self
            {
                self.params.insert($docker_name, serde_json::json!($name.as_ref()));
                self
            }
        }
    };
}

macro_rules! impl_map_field {
    (url $(#[doc = $docs:expr])* $name:ident: $ty:tt => $docker_name:literal) => {
        impl_map_field! { $(#[doc = $docs])* $name: $ty => $docker_name => serde_json::to_string(&$name.into_iter().collect::<std::collections::HashMap<_, _>>()).unwrap_or_default() }
    };
    (json $(#[doc = $docs:expr])* $name:ident: $ty:tt => $docker_name:literal) => {
        impl_map_field! { $(#[doc = $docs])* $name: $ty => $docker_name => serde_json::json!($name.into_iter().collect::<std::collections::HashMap<_, _>>()) }
    };
    ($(#[doc = $docs:expr])* $name:ident: $ty:tt => $docker_name:literal => $ret:expr) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name  >]<[< $ty >], K, V>(mut self, $name: $ty)-> Self
            where
                $ty: IntoIterator<Item = (K, V)>,
                K: AsRef<str> + serde::Serialize + Eq + std::hash::Hash,
                V: AsRef<str> + serde::Serialize
            {
                self.params.insert($docker_name, $ret);
                self
            }
        }
    };
}

macro_rules! impl_opts_builder {
    ($(#[doc = $docs:expr])* $name:ident $ty:expr) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            #[derive(serde::Serialize, Debug, Default)]
            pub struct [< $name Opts >] {
                params: std::collections::HashMap<&'static str, $ty>,
            }
            impl [< $name Opts >] {
                calculated_doc!{
                #[doc = concat!("Returns a new instance of a builder for ", stringify!($name), "Opts.")]
                pub fn builder() -> [< $name OptsBuilder >] {
                    [< $name OptsBuilder >]::default()
                }
                }
            }

            calculated_doc!{
            #[doc = concat!("A builder struct for ", stringify!($name), "Opts.")]
            #[derive(Default, Debug)]
            pub struct [< $name OptsBuilder >] {
                params: std::collections::HashMap<&'static str, $ty>,
            }
            }

            impl [< $name OptsBuilder >] {
                calculated_doc!{
                #[doc = concat!("Finish building ", stringify!($name), "Opts.")]
                pub fn build(&self) -> [< $name Opts >] {
                    [< $name Opts >] {
                        params: self.params.clone(),
                    }
                }
                }
            }
       }
    };
    (json => $(#[doc = $docs:expr])* $name:ident) => {
        paste::item! {
            impl_opts_builder!($(#[doc = $docs])* $name serde_json::Value);

            impl [< $name Opts >] {
                /// Serialize options as a JSON String. Returns an error if the options will fail
                /// to serialize.
                pub fn serialize(&self) -> crate::Result<String> {
                    serde_json::to_string(&self.params).map_err(crate::Error::from)
                }
            }
        }
    };
    (url => $(#[doc = $docs:expr])* $name:ident) => {
        paste::item! {
            impl_opts_builder!($(#[doc = $docs])* $name String);

            impl [< $name  Opts >] {
                /// Serialize options as a URL query String. Returns None if no options are defined.
                pub fn serialize(&self) -> Option<String> {
                    if self.params.is_empty() {
                        None
                    } else {
                        Some(
                            crate::util::url::encoded_pairs(&self.params)
                        )
                    }
                }
            }
        }
    };
}

/// Necessary to work around https://github.com/rust-lang/rust/issues/52607.
macro_rules! calculated_doc {
    (
        $(
            #[doc = $doc:expr]
            $thing:item
        )*
    ) => {
        $(
            #[doc = $doc]
            $thing
        )*
    };
}

macro_rules! impl_api_ty {
    ($(#[doc = $docs:expr])* $name:ident => $name_field:ident) => {
        paste::item! {
            pub type [< $name Id >] = String;
            pub type [< $name IdRef >]<'a> = &'a str;

            calculated_doc!{
            #[doc = concat!("Interface for accessing and manipulating Docker ", stringify!($name), ".\n", $($docs,)* "\n", api_url!($name))]
            #[derive(Debug)]
            pub struct [< $name >]<'docker> {
                docker: &'docker crate::Docker,
                $name_field: [< $name Id >],
            }
            }
            impl<'docker> [< $name >]<'docker> {
                // TODO: this is possible on nightly, figure out what to do
                calculated_doc!{
                #[doc = concat!("Exports an interface exposing operations against a ", stringify!($name), " instance.")]
                pub fn new<ID>(docker: &'docker crate::Docker, $name_field: ID) -> Self
                where
                    ID: Into<[< $name Id>]>,
                {
                    [< $name >] {
                        docker,
                        $name_field: $name_field.into(),
                    }
                }
                }

                calculated_doc!{
                #[doc = concat!("A getter for ", stringify!($name), " ", stringify!($name_field))]
                pub fn $name_field(&self) -> [< $name IdRef >] {
                    &self.$name_field
                }
                }


            }


            calculated_doc!{
            #[doc = concat!("Interface for Docker ", stringify!($name), "s.", stringify!($name), ">")]
            #[derive(Debug)]
            pub struct [< $name s >]<'docker> {
                docker: &'docker crate::Docker,
            }
            }

            impl<'docker> [< $name s >]<'docker> {
                calculated_doc!{
                #[doc = concat!("Exports an interface for interacting with Docker ", stringify!($name), "s.")]
                pub fn new(docker: &'docker crate::Docker) -> Self {
                    [< $name s >] { docker }
                }
                }

                calculated_doc!{
                #[doc = concat!("Returns a reference to a set of operations available to a specific ", stringify!($name), ".")]
                pub fn get<ID>(&self, $name_field: ID) -> [< $name >]<'docker>
                where
                    ID: Into<[< $name Id >]>,
                {
                    [< $name >]::new(self.docker, $name_field)
                }
                }
            }

        }
    }
}

macro_rules! impl_filter_func {
    ($(#[doc = $doc:expr])* $filter_ty:ident) => {
        $(
            #[doc = $doc]
        )*
        pub fn filter<F>(mut self, filters: F) -> Self
        where
            F: IntoIterator<Item = $filter_ty>,
        {
            let mut param = std::collections::HashMap::new();
            for (key, val) in filters.into_iter().map(|f| f.query_key_val()) {
                param.insert(key, vec![val]);
            }
            // structure is a a json encoded object mapping string keys to a list
            // of string values
            self.params
                .insert("filters", serde_json::to_string(&param).unwrap_or_default());
            self
        }
    };
}

macro_rules! api_url {
    () => {
        concat!("https://docs.docker.com/engine/api/", version!())
    };
    (operation $ep:expr) => {
        concat!("\n[Api Reference](", api_url!(), "/#operation/", $ep, ")")
    };
    (tag $ep:expr) => {
        concat!("\n[Api Reference](", api_url!(), "/#tag/", $ep, ")")
    };
    ($base:ident) => {
        api_url!(tag stringify!($base))
    };
    ($base:ident => $op:ident) => {
        api_url!(operation concat!(stringify!($base), stringify!($op)))
    };
}

macro_rules! api_doc {
    (
        $base:ident => $op:ident
        $(#[doc = $doc:expr])*
        |
        $it:item
    ) => {
        calculated_doc!{
            #[doc = concat!(api_url!($base => $op))]
            #[doc = "\n"]
            $(
                #[doc = $doc]
            )*
            $it
        }
    };
    (
        $base:ident
        $(#[doc = $doc:expr])*
        |
        $it:item
    ) => {
        calculated_doc!{
            #[doc = concat!(api_url!($base))]
            #[doc = "\n"]
            $(
                #[doc = $doc]
            )*
            $it
        }
    };
}

macro_rules! impl_api_ep {
    (
        $it:ident: $base:ident, $resp:ident
        $(
            $op:ident -> $ep:expr $(,$extra:expr)*
        )*
    ) => {
        $(
        impl_api_ep! {$op $it: $base -> $resp $ep $(,$extra)* }
        )*
    };
    (
        Inspect $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:tt $(,$extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => Inspect
        #[doc = concat!("Inspect this ", stringify!($base), ".")]
        |
        pub async fn inspect(&self) -> Result<[< $base $ret >]> {
            let $it = self;
            self.docker.get_json($ep).await
        }}
        }
    };
    (
        Inspect $it:ident: $base:ident -> $resp:ident $ep:expr $(,$extra:expr)*
    ) => {
        impl_api_ep! { Inspect $it: $base -> $resp $ep, Info }
    };
    (
        ForceDelete $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:tt $(,$extra:expr)*
    ) => {

        paste::item! {
        async fn _delete(&self, force: bool) -> Result<[< $ret >]> {
            let query = if force {
                Some(crate::util::url::encoded_pair("force", force))
            } else {
                None
            };

            let $it = self;
            let ep = crate::util::url::construct_ep($ep, query);

            self.docker
                .delete_json(ep.as_ref())
                .await
        }
        }
        paste::item! {
        api_doc! { $base => Delete
        #[doc = concat!("Delete this ", stringify!($base), ".")]
        |
        pub async fn force_delete(&self) -> Result<[< $ret >]> {
            self._delete(true).await
        }}
        }
        paste::item! {
        api_doc! { $base => Delete
        #[doc = concat!("Delete this ", stringify!($base), ".")]
        |
        pub async fn delete(&self) -> Result<[< $ret >]> {
            self._delete(false).await
        }}
        }
    };
    (
        Delete $it:ident: $base:ident -> $resp:ident $ep:expr $(,$extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => Delete
        #[doc = concat!("Delete this ", stringify!($base), ".")]
        |
        pub async fn delete(&self) -> Result<()> {
            let $it = self;
            self.docker.delete($ep).await.map(|_| ())
        }}
        }
    };
    (
        DeleteWithOpts $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:tt $(,$extra:expr)*
    ) => {
        impl_api_ep! { DeleteWithOpts $it: $base -> $resp $ep, $ret => $($extra)* }
    };
    (
        DeleteWithOpts $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:tt => $fn:expr
    ) => {
        paste::item! {
        api_doc! { $base => Delete
        #[doc = concat!("Delete this ", stringify!($base), ".")]
        #[doc = concat!("Use [`delete`](", stringify!($base), "::delete) to delete without options.")]
        |
        pub async fn remove(&self, opts: &[< Rm $base Opts >]) -> Result<[< $ret >]> {
            let $it = self;
            let ep = crate::util::url::construct_ep($ep, opts.serialize());
            self.docker.$fn(ep.as_ref()).await
        }}
        }
        paste::item! {
        api_doc! { $base => Delete
        #[doc = concat!("Delete this ", stringify!($base), ".")]
        #[doc = concat!("Use [`remove`](", stringify!($base), "::remove) to customize options.")]
        |
        pub async fn delete(&self) -> Result<[< $ret >]> {
            let $it = self;
            self.docker.$fn($ep).await
        }}
        }
    };
    (
        List $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:tt $(, $extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => List
        #[doc = concat!("List available ", stringify!($base), "s.")]
        |
        pub async fn list(&self, opts: &[< $base ListOpts >]) -> Result<$ret> {
            let ep = crate::util::url::construct_ep($ep, opts.serialize());
            self.docker.get_json(&ep).await
        }}
        }
    };
    (
        List $it:ident: $base:ident -> $resp:ident $ep:expr $(, $extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => List
        #[doc = concat!("List available ", stringify!($base), "s.")]
        |
        pub async fn list(&self, opts: &[< $base ListOpts >]) -> Result<Vec<[< $base Info >]>> {
            let ep = crate::util::url::construct_ep($ep, opts.serialize());
            self.docker.get_json(&ep).await
        }}
        }
    };
    (
        Create $it:ident: $base:ident -> $resp:ident $ep:expr $(, $extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => Create
        #[doc = concat!("Create a new ", stringify!($base), ".")]
        |
        pub async fn create(&self, opts: &[< $base CreateOpts >]) -> Result<[< $base >]<'docker>> {
            self.docker.post_json(&$ep, Payload::Json(opts.serialize()?)).await
            .map(|$resp: [< $base CreateInfo >]| [< $base >]::new(self.docker, $($extra)*))
        }}
        }
    };
    (
        Prune $it:ident: $base:ident -> $resp:ident $ep:expr $(, $extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => Prune
        #[doc = concat!("Delete stopped/unused ", stringify!($base), "s.")]
        |
        pub async fn prune(&self, opts: &[< $base PruneOpts >]) -> Result<[< $base sPruneInfo >]> {
            self.docker
                .post_json(
                    &crate::util::url::construct_ep($ep, opts.serialize()),
                    crate::conn::Payload::empty()
                ).await
        }}
        }
    };
    (
        Logs $it:ident: $base:ident -> $resp:ident $ep:expr $(, $extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => Logs
        #[doc = concat!("Returns a stream of logs from a ", stringify!($base), ".")]
        |
        pub fn logs(
            &self,
            opts: &crate::api::LogsOpts
        ) -> impl futures_util::Stream<Item = crate::Result<bytes::Bytes>> + Unpin + 'docker {
            let $it = self;
            let ep = crate::util::url::construct_ep($ep, opts.serialize());

            Box::pin(self.docker.stream_get(ep))
        }
        }}
    };
}
