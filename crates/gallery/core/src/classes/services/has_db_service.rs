use crate::services::db::db_service::DbService;

pub trait HasDbService {
    // Gets the DB service
    fn db_service(&self) -> &dyn DbService;
}
