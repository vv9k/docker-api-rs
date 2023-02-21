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

            calculated_doc!{
            #[doc = concat!("Interface for accessing and manipulating Docker ", stringify!($name), ".\n", $($docs,)* "\n", api_url!($name))]
            #[derive(Debug)]
            pub struct [< $name >] {
                docker: crate::Docker,
                $name_field: crate::Id,
            }
            }
            impl [< $name >] {
                // TODO: this is possible on nightly, figure out what to do
                calculated_doc!{
                #[doc = concat!("Exports an interface exposing operations against a ", stringify!($name), " instance.")]
                pub fn new(docker: crate::Docker, $name_field: impl Into<crate::Id>) -> Self
                {
                    [< $name >] {
                        docker,
                        $name_field: $name_field.into(),
                    }
                }
                }

                calculated_doc!{
                #[doc = concat!("A getter for ", stringify!($name), " ", stringify!($name_field))]
                pub fn $name_field(&self) -> &crate::Id {
                    &self.$name_field
                }
                }


            }


            calculated_doc!{
            #[doc = concat!("Interface for Docker ", stringify!($name), "s.", stringify!($name), ">")]
            #[derive(Debug)]
            pub struct [< $name s >] {
                docker: crate::Docker,
            }
            }

            impl [< $name s >] {
                calculated_doc!{
                #[doc = concat!("Exports an interface for interacting with Docker ", stringify!($name), "s.")]
                pub fn new(docker: crate::Docker) -> Self {
                    [< $name s >] { docker }
                }
                }

                calculated_doc!{
                #[doc = concat!("Returns a reference to a set of operations available to a specific ", stringify!($name), ".")]
                pub fn get(&self, $name_field: impl Into<crate::Id>) -> [< $name >]
                {
                    [< $name >]::new(self.docker.clone(), $name_field)
                }
                }
            }

        }
    }
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
            $op:ident -> $ep:expr, $ret:expr $(,$extra:expr)*
        )*
    ) => {
        $(
        impl_api_ep! {$op $it: $base -> $resp $ep, $ret $(,$extra)* }
        )*
    };
    (
        Inspect $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:expr $(,$extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => Inspect
        |
        #[doc = concat!("Inspect this ", stringify!($base), ".")]
        pub async fn inspect(&self) -> Result<$ret> {
            let $it = self;
            self.docker.get_json($ep).await
        }}
        }
    };
    (
        ForceDelete $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:expr $(,$extra:expr)*
    ) => {

        paste::item! {
        async fn _delete(&self, force: bool) -> Result<$ret> {
            let query = if force {
                Some(containers_api::url::encoded_pair("force", force))
            } else {
                None
            };

            let $it = self;
            let ep = containers_api::url::construct_ep($ep, query);

            self.docker
                .delete_json(ep.as_ref())
                .await
        }
        }
        paste::item! {
        api_doc! { $base => Delete
        |
        #[doc = concat!("Delete this ", stringify!($base), ".")]
        pub async fn force_delete(&self) -> Result<$ret> {
            self._delete(true).await
        }}
        }
        paste::item! {
        api_doc! { $base => Delete
        |
        #[doc = concat!("Delete this ", stringify!($base), ".")]
        pub async fn delete(&self) -> Result<$ret> {
            self._delete(false).await
        }}
        }
    };
    (
        Delete $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:expr $(,$extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => Delete
        |
        #[doc = concat!("Delete this ", stringify!($base), ".")]
        pub async fn delete(&self) -> Result<()> {
            let $it = self;
            self.docker.delete($ep).await.map(|_| ())
        }}
        }
    };
    (
        DeleteWithOpts $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:expr $(,$extra:expr)*
    ) => {
        impl_api_ep! { DeleteWithOpts $it: $base -> $resp $ep, $ret => $($extra)* }
    };
    (
        DeleteWithOpts $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:expr => $fn:expr
    ) => {
        paste::item! {
        api_doc! { $base => Delete
        |
        #[doc = concat!("Delete this ", stringify!($base), ".")]
        #[doc = concat!("Use [`delete`](", stringify!($base), "::delete) to delete without options.")]
        pub async fn remove(&self, opts: &[< $base RemoveOpts >]) -> Result<$ret> {
            let $it = self;
            let ep = containers_api::url::construct_ep($ep, opts.serialize());
            self.docker.$fn(ep.as_ref()).await
        }}
        }
        paste::item! {
        api_doc! { $base => Delete
        |
        #[doc = concat!("Delete this ", stringify!($base), ".")]
        #[doc = concat!("Use [`remove`](", stringify!($base), "::remove) to customize options.")]
        pub async fn delete(&self) -> Result<[< $ret >]> {
            let $it = self;
            self.docker.$fn($ep).await
        }}
        }
    };
    (
        List $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:expr $(, $extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => List
        |
        #[doc = concat!("List available ", stringify!($base), "s.")]
        pub async fn list(&self, opts: &[< $base ListOpts >]) -> Result<Vec<$ret>> {
            let ep = containers_api::url::construct_ep($ep, opts.serialize());
            self.docker.get_json(&ep).await
        }}
        }
    };
    (
        Create $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:expr $(, $extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => Create
        |
        #[doc = concat!("Create a new ", stringify!($base), ".")]
        pub async fn create(&self, opts: &[< $base CreateOpts >]) -> Result<[< $base >]> {
            self.docker.post_json(&$ep, Payload::Json(opts.serialize_vec()?), Headers::none()).await
            .map(|$resp: [< $ret >]| [< $base >]::new(self.docker.clone(), $($extra)*))
        }}
        }
    };
    (
        Prune $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:expr  $(, $extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => Prune
        |
        #[doc = concat!("Delete stopped/unused ", stringify!($base), "s.")]
        pub async fn prune(&self, opts: &[< $base PruneOpts >]) -> Result<$ret> {
            self.docker
                .post_json(
                    &containers_api::url::construct_ep($ep, opts.serialize()),
                    crate::conn::Payload::empty(),
                    crate::conn::Headers::none(),
                ).await
        }}
        }
    };
    (
        Logs $it:ident: $base:ident -> $resp:ident $ep:expr, $ret:expr $(, $extra:expr)*
    ) => {
        paste::item! {
        api_doc! { $base => Logs
        |
        #[doc = concat!("Returns a stream of logs from a ", stringify!($base), ".")]
        pub fn logs<'docker>(
            &'docker self,
            opts: &crate::opts::LogsOpts
        ) -> impl futures_util::Stream<Item = crate::Result<containers_api::conn::TtyChunk>> + Unpin + 'docker {
            use containers_api::conn::tty;
            use futures_util::TryStreamExt;
            let $it = self;
            let ep = containers_api::url::construct_ep($ep, opts.serialize());

            let stream = Box::pin(self.docker.get_stream(ep).map_err(|e| containers_api::conn::Error::Any(Box::new(e))));

            Box::pin(tty::decode(stream).map_err(crate::Error::Error))
        }
        }}
    };
}
