use crate::api::SwarmSpec;

impl_json_opts_builder!(SwarmJoin);

impl SwarmJoinOptsBuilder {
    impl_str_field!(listen_addr: A => "ListenAddr");

    impl_str_field!(advertise_addr: A => "AdvertiseAddr");

    impl_str_field!(data_path_addr: A => "DataPathAddr");

    impl_vec_field!(remote_addrs: A => "RemoteAddrs");

    impl_str_field!(join_token: T => "JoinToken");
}

impl_json_opts_builder!(SwarmInit);

impl SwarmInitOptsBuilder {
    impl_str_field!(listen_addr: A => "ListenAddr");

    impl_str_field!(advertise_addr: A => "AdvertiseAddr");

    impl_str_field!(data_path_addr: A => "DataPathAddr");

    impl_field!(data_path_port: u32 => "DataPathPort");

    impl_vec_field!(default_addr_pool: A => "DefaultAddrPool");

    impl_field!(force_new_cluster: bool => "ForceNewCluster");

    impl_field!(subnet_size: u32 => "SubnetSize");

    impl_field!(spec: SwarmSpec => "Spec");
}
