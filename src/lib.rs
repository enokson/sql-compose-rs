
pub mod actions;
pub mod commands;
pub mod compose;
pub mod conditions;
pub mod constraints;
pub mod data_types;
pub mod fns;
pub mod joins;
pub mod misc;
pub mod modifiers;
pub mod operators;

pub use crate::commands::*;
pub use crate::compose::*;
pub use crate::conditions::*;
pub use crate::constraints::*;
pub use crate::data_types::*;
pub use crate::fns::*;
pub use crate::joins::*;
pub use crate::misc::*;
pub use crate::modifiers::*;
pub use crate::operators::*;

#[cfg(test)]
mod tests {
    // use super::{*, column};

/*     #[test]
    fn create_table() {
        let sql = compose!(
            create_table!(if_not_exists, "users"),
            columns!(
                column!("id", int, auto_increment, not_null),
                column!("username", varchar!(255), not_null),
                column!("chars", char_type!(11)),
                column!("chars_2", char_type),
                column!("binaries", binary),
                column!("tiny_blobs", tiny_blob),
                column!("tiny_texts", tiny_text),
                column!("texts", text),
                column!("blobs", blob),
                column!("medium_sized_texts", medium_text),
                column!("medium_blobs", medium_blob),
                column!("long_texts", long_text),
                column!("long_blobs", long_blob),
                column!("order_status", enum_type!("open", "in_progess", "closed")),
                column!("friends", set_type!("jimbob", "sally", "jose")),
                column!("date_created", timestamp),
                primary_key!("id")
            )
        );
        assert_eq!("create table if not exists users ()", sql)
    } */

/*     #[test]
    fn selecting() {
        let where_name_equals = compose!("name", eq);
        let where_row_has_requested_name = compose!(where_name_equals, escape!("mc'donald"));
        let sql = compose!(
            select!("id", "username"),
            from!("users"),
            where_clause!(
                or!(
                    and!(gt!("id", 1), lte!("id", 100)),
                    and!(gte!("id", 1000), lt!("id", 30)),
                    eq!("id", 0),
                    is_null!("id"),
                    is_not_null!("username"),
                    between!("userid", "1", "2"),
                    not_between!("userid", "4", "6")
                ),
                or!(
                    eq!("name", escape!("o'brian")),
                    compose!(where_name_equals, escape!("o'connor")),
                    where_row_has_requested_name
                )
            ),
            order_by!(desc!("username"), asc!("id"))
        );
        println!("{}", sql);
        assert_eq!("SELECT (id, username) FROM users".to_string(), sql);
    } */

/*     #[test]
    fn inserting() {
        let sql = compose!(
            insert_into!("users"),
            columns!("username", "first_name"),
            values!(escape!("myusername"), escape!("myfirstname"))
        );
        let sql2 = compose!(
            insert_into!("users"),
            insert_key_values!(
                key_value!("username", escape!("myusername")),
                key_value!("first_name", escape!("myfirstname"))
            )
        );
        assert_eq!(
            "INSERT INTO users (username, first_name) VALUES ('myusername', 'myfirstname')",
            sql
        );
        assert_eq!(
            "INSERT INTO users (username, first_name) VALUES ('myusername', 'myfirstname')",
            sql2
        );
    } */

/*     #[test]
    fn updating() {
        let sql = compose!(
            update!("users"),
            set!(
                eq!("username", escape!("new_username")),
                eq!("first_name", escape!("new_first_name"))
            ),
            where_clause!(eq!("id", "1"))
        );
        assert_eq!("UPDATE users SET username, first_name VALUES ('new_username', new_first_name') WHERE id = 1", sql)
    } */

/*     #[test]
    fn deleting() {
        let sql = compose!(delete_from!("users"), where_clause!(eq!("id", "1")));
        assert_eq!("DELETE FROM users WHERE id = 1", sql);
    } */

/*     #[test]
    fn joins() {
        let sql = compose!(
            select!(
                "Orders.OrderID",
                "Customers.CustomerName",
                "Shippers.ShipperName"
            ),
            from!("Orders"),
            inner!(
                "Customers",
                eq!("Orders.CustomerID", "Customers.CustomerID")
            ),
            inner!("Shippers", eq!("Orders.ShipperID", "Shippers.ShipperID"))
        );
        assert_eq!("SELECT Orders.OrderID, Customers.CustomerName, Shippers.ShipperName FROM Orders INNER JOIN Customers ON Orders.CustomerID = Customers.CustomerID INNER JOIN Shippers ON Orders.ShipperID = Shippers.ShipperID", sql);
    } */
}
