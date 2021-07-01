macro_rules! impl_vec_field {
    ($($docs:literal)* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name  >]<[< $ty >], S>(&mut self, $name: $ty)-> &mut Self
            where
                $ty: IntoIterator<Item = S>,
                S: AsRef<str> + Serialize
            {
                self.params.insert($docker_name, json!($name.into_iter().collect::<Vec<_>>()));
                self
            }
        }
    };
}

macro_rules! impl_field {
    ($($docs:literal)* $name:ident: $ty:ty => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](&mut self, $name: $ty)-> &mut Self
            {
                self.params.insert($docker_name, json!($name));
                self
            }
        }
    };
}

macro_rules! impl_str_field {
    ($($docs:literal)* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >]<[< $ty >]>(&mut self, $name: $ty)-> &mut Self
            where
                $ty: AsRef<str> + Serialize,
            {
                self.params.insert($docker_name, json!($name.as_ref()));
                self
            }
        }
    };
}
macro_rules! impl_map_field {
    ($($docs:literal)* $name:ident: $ty:tt => $docker_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name  >]<[< $ty >], K, V>(&mut self, $name: $ty)-> &mut Self
            where
                $ty: IntoIterator<Item = (K, V)>,
                K: AsRef<str> + Serialize + Eq + Hash,
                V: AsRef<str> + Serialize
            {
                self.params.insert($docker_name, json!($name.into_iter().collect::<HashMap<_, _>>()));
                self
            }
        }
    };
}

macro_rules! impl_url_opts_builder {
    ($($docs:literal)* $name:ident) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            #[derive(Default, Debug)]
            pub struct [< $name Opts >] {
                params: HashMap<&'static str, String>
            }

            impl [< $name  Opts >] {
                /// return a new instance of a builder for Opts
                pub fn builder() -> [< $name  OptsBuilder >] {
                  [< $name  OptsBuilder >]::default()
                }

                /// serialize Opts as a string. returns None if no Opts are defined
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

            #[derive(Default, Debug)]
            pub struct [< $name  OptsBuilder >] {
                params: HashMap<&'static str, String>
            }

            impl [< $name  OptsBuilder >] {
                pub fn build(&self) -> [< $name  Opts >] {
                    [< $name Opts >] {
                        params: self.params.clone(),
                    }
                }
            }
        }
    };
}

macro_rules! impl_api_ty {
    ($($docs:literal)* $name:ident => $name_field:ident : $name_field_tt:tt) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            #[derive(Debug)]
            pub struct [< $name >]<'docker> {
                docker: &'docker Docker,
                $name_field: String,
            }
            impl<'docker> [< $name >]<'docker> {
                // TODO: this is possible on nightly, figure out what to do
                //#[doc = concat!("Exports an interface exposing operations against a ", stringify!($name), "instance")]
                pub fn new<$name_field_tt>(docker: &'docker Docker, $name_field: $name_field_tt) -> Self
                where
                    $name_field_tt: Into<String>,
                {
                    [< $name >] {
                        docker,
                        $name_field: $name_field.into(),
                    }
                }

                //#[doc = concat!("A getter for ", $name, " ", $name_field)]
                pub fn $name_field(&self) -> &str {
                    &self.$name_field
                }


            }


            $(
                #[doc= $docs]
            )*
            #[derive(Debug)]
            pub struct [< $name s >]<'docker> {
                docker: &'docker Docker,
            }

            impl<'docker> [< $name s >]<'docker> {
                //#[doc = concat!("Exports an interface for interacting with Docker ", stringify!($name), "s.")]
                pub fn new(docker: &'docker Docker) -> Self {
                    [< $name s >] { docker }
                }

                /// Returns a reference to a set of operations available to a specific container instance
                pub fn get<$name_field_tt>(&self, $name_field: $name_field_tt) -> [< $name >]<'docker>
                where
                    $name_field_tt: Into<String>,
                {
                    [< $name >]::new(self.docker, $name_field)
                }
            }

        }
    }
}
