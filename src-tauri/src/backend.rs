pub struct QueryResult<T> {
    results: Vec<T>,
}

/// T: QueryResult
/// U: Album
/// V: Song
pub trait Backend<T, U, V> {
    fn get_album(&self, id: String) -> Option<U>;
    fn get_song(&self, id: String) -> V;
    fn search(&self, q: String) -> QueryResult<T>;
}
