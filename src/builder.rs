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
            pub struct [< $name Options >] {
                params: HashMap<&'static str, String>
            }

            impl [< $name  Options >] {
                /// return a new instance of a builder for options
                pub fn builder() -> [< $name  OptionsBuilder >] {
                  [< $name  OptionsBuilder >]::default()
                }

                /// serialize options as a string. returns None if no options are defined
                pub fn serialize(&self) -> Option<String> {
                    if self.params.is_empty() {
                        None
                    } else {
                        Some(
                            form_urlencoded::Serializer::new(String::new())
                                .extend_pairs(&self.params)
                                .finish(),
                        )
                    }
                }
            }

            #[derive(Default, Debug)]
            pub struct [< $name  OptionsBuilder >] {
                params: HashMap<&'static str, String>
            }

            impl [< $name  OptionsBuilder >] {
                pub fn build(&self) -> [< $name  Options >] {
                    [< $name Options >] {
                        params: self.params.clone(),
                    }
                }
            }
        }
    };
}