use lorchestre::{Lorchestre, platform::Frontend, track::MusicCollectionIndexer};

fn main() -> std::process::ExitCode {
    let mut indexer = MusicCollectionIndexer::new();
    indexer.index("/home/luxluth/Music/".into());

    Lorchestre::init(indexer.collection).start()
}
