//! Types which represent a SQL data type.
//!
//! The structs in this module are *only* used as markers to represent a SQL type.
//! They should never be used in your structs.
//! If you'd like to know the rust types which can be used for a given SQL type,
//! see the documentation for that SQL type.
//!
//! Any backend specific types are re-exported through this module
/// The PostgreSQL [Point](https://www.postgresql.org/docs/current/static/datatype-geometric.html) type.
///
/// ### [`ToSql`](::diesel::serialize::ToSql) impls
///
/// - [`PgPoint`](::pg::data_types::PgPoint)
///
/// ### [`FromSql`](::diesel::deserialize::FromSql) impls
///
/// - [`PgPoint`](::pg::data_types::PgPoint)
///
///
/// # Examples
///
/// ```rust
/// # #![allow(dead_code)]
/// # #[macro_use] extern crate diesel;
/// # extern crate diesel_geometry;
/// # include!("../../doctest_setup.rs");
/// use diesel_geometry::data_types::PgPoint;
///
///
/// table! {
///     use diesel::sql_types::*;
///     use diesel_geometry::sql_types::Point;
///     items {
///         id -> Integer,
///         name -> VarChar,
///         location -> Point,
///     }
/// }
///
/// # fn main() {
/// #     use diesel::insert_into;
/// #     use items::dsl::*;
/// #     let connection = connection_no_data();
/// #     connection.execute("CREATE TABLE items (
/// #         id SERIAL PRIMARY KEY,
/// #         name VARCHAR NOT NULL,
/// #         location POINT NOT NULL
/// #     )").unwrap();
/// let inserted_location = insert_into(items)
///     .values((name.eq("Shiny Thing"), location.eq(PgPoint(3.1, 9.4))))
///     .returning(location)
///     .get_result(&connection);
/// assert_eq!(Ok(PgPoint(3.1, 9.4)), inserted_location);
/// # }
/// ```
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[postgres(oid = "600", array_oid = "1017")]
#[mysql_type = "Blob"]
pub struct Point;

#[cfg(feature = "mysql")]
pub use mysql::types::*;

#[cfg(feature = "postgres")]
pub use pg::types::sql_types::*;
