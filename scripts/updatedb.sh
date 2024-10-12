# diesel_cli
# cargo add dsync

dsync -i src/schema.rs --once-connection-type --single-model-file --once-common-structs -o src/models --diesel-backend diesel::pg::Pg --connection-type crate::db::Connection
