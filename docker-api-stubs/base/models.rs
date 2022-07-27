#![allow(
    non_snake_case,
    clippy::redundant_field_names,
    clippy::new_without_default,
    clippy::too_many_arguments
)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;

use chrono::{DateTime, Utc};
