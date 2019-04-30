#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[mysql_type = "Blob"]
pub struct Linestring;