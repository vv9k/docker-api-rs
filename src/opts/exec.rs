use containers_api::{impl_field, impl_opts_builder, impl_str_field, impl_vec_field};

impl_opts_builder!(json => ExecContainer);

impl ExecContainerOptsBuilder {
    impl_vec_field!(
        /// Command to run, as an array of strings.
        cmd => "Cmd"
    );

    impl_vec_field!(
        /// A list of environment variables in the form 'VAR=value'.
        env => "Env"
    );

    impl_field!(
        /// Attach to stdout of the exec command.
        attach_stdout: bool => "AttachStdout"
    );

    impl_field!(
        /// Attach to stderr of the exec command.
        attach_stderr: bool => "AttachStderr"
    );

    impl_str_field!(
        /// Override the key sequence for detaching a container. Format is a single
        /// character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _.
        detach_keys => "DetachKeys"
    );

    impl_field!(
        /// Allocate a pseudo-TTY.
        tty: bool => "Tty"
    );

    impl_field!(
        /// Runs the exec process with extended privileges. (Default: `false`)
        privileged: bool => "Privileged"
    );

    impl_str_field!(
        /// The user, and optionally, group to run the exec process inside the container.
        /// Format is one of: user, user:group, uid, or uid:gid.
        user => "User"
    );

    impl_str_field!(
        /// The working directory for the exec process inside the container.
        working_dir => "WorkingDir"
    );
}

impl_opts_builder!(json => ExecResize);

impl ExecResizeOptsBuilder {
    impl_field!(height: u64 => "Height");
    impl_field!(width: u64 => "Width");
}
