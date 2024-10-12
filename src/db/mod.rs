use diesel::PgConnection;

pub mod pg;
pub type Connection = PgConnection;
