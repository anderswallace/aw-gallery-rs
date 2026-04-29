use crate::io::db::db_io::DbIO;

pub trait HasDbIO {
    // Gets the DB IO
    fn db_io(&self) -> &dyn DbIO;
}
