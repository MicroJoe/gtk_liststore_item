//! Automatic gtk::ListStore struct derive for Rust.
//!
//! # Example
//!
//! ```
//! use gtk::prelude::*;
//!
//! use gladis::Gladis;
//! use gladis_proc_macro::Gladis;
//! use gtk_liststore_item::ListStoreItem;
//! use gtk_liststore_item_derive::ListStoreItem;
//!
//! const GLADE_SRC: &str = r#"
//! <?xml version="1.0" encoding="UTF-8"?>
//! <!-- Generated with glade 3.22.2 -->
//! <interface>
//!   <requires lib="gtk+" version="3.20"/>
//!   <object class="GtkListStore" id="list_store">
//!     <columns>
//!       <!-- column-name name -->
//!       <column type="gchararray"/>
//!       <!-- column-name value -->
//!       <column type="guint"/>
//!     </columns>
//!   </object>
//! </interface>
//! "#;
//!
//! #[derive(Gladis)]
//! struct Glade {
//!     list_store: gtk::ListStore,
//! }
//!
//! #[derive(ListStoreItem)]
//! struct Item {
//!     name: String,
//!     value: u32,
//! }
//!
//! fn main() {
//!     gtk::init().unwrap();
//!
//!     let glade = Glade::from_string(GLADE_SRC);
//!
//!     let item = Item { name: "foobar".into(), value: 42 };
//!     item.insert_into_liststore(glade.list_store);
//! }
//! ```

use gtk::prelude::*;

/// A trait to ease interraction with a [ListStore](gtk::ListStore) using an intermediate struct.
///
/// If we want to interact with a table defined this way:
///
/// | number | name  | type         |
/// |--------|-------|--------------|
/// | 0      | name  | `gchararray` |
/// | 1      | value | `guint32`    |
///
/// We can create a struct that represent the columns in the order that they appear, and then
/// implement this trait to interact with the table's entries.
///
/// # Automatic implementation
/// ```
/// use gtk::prelude::*;
/// use gtk_liststore_item::ListStoreItem;
/// use gtk_liststore_item_derive::ListStoreItem;
///
/// #[derive(ListStoreItem)]
/// struct Item {
///     name: String,
///     value: u32,
/// }
/// ```
///
/// # Manual implementation
/// ```
/// use gtk::prelude::*;
/// use gtk_liststore_item::ListStoreItem;
///
/// struct Item {
///     name: String,
///     value: u32,
/// }
///
/// impl ListStoreItem for Item {
///     fn from_liststore_iter<S>(list_store: S, iter: &gtk::TreeIter) -> Option<Self>
///         where S: TreeModelExt
///     {
///         Some(Item {
///             name: list_store.get_value(&iter, 0).get::<String>().ok()??,
///             value: list_store.get_value(&iter, 1).get::<u32>().ok()??,
///         })
///     }
///
///     fn insert_into_liststore<S>(&self, list_store: S) -> gtk::TreeIter
///         where S: GtkListStoreExtManual
///     {
///         list_store.insert_with_values(
///             None,
///             &[0, 1],
///             &[&self.name, &self.value])
///     }
/// }
/// ```
pub trait ListStoreItem {
    fn from_liststore_iter<S>(list_store: S, iter: &gtk::TreeIter) -> Option<Self>
    where
        S: TreeModelExt,
        Self: std::marker::Sized;

    /// Construct an item from a `ListStore` and a `TreePath`.
    ///
    /// The `ListStore` is where the data is stored, and the `TreePath` is a pointer to the
    /// location of the data in the table.
    fn from_liststore_path<S>(list_store: S, tp: &gtk::TreePath) -> Option<Self>
    where
        S: TreeModelExt,
        Self: std::marker::Sized,
    {
        list_store
            .get_iter(tp)
            .and_then(|iter| ListStoreItem::from_liststore_iter(list_store, &iter))
    }

    /// Instert an item into a `ListStore` as a new entry.
    fn insert_into_liststore<S>(&self, list_store: S) -> gtk::TreeIter
    where
        S: GtkListStoreExtManual;
}
