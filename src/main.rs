use lorchestre::{Lorch, Lorchestre, platform::Frontend, track::MusicCollectionIndexer};

fn main() -> std::process::ExitCode {
    let mut indexer = MusicCollectionIndexer::new();
    if Lorch::cache_path().exists() {
        indexer.load_from_cache();
    } else {
        indexer.index("/home/luxluth/Music/".into());
        indexer.save();
    }

    Lorchestre::init(indexer.collection).start()
}
