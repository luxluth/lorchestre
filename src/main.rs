use std::time::Instant;

use lorchestre::track::{IdKey, MusicCollection, MusicCollectionIndexer};

fn main() {
    let mut indexer = MusicCollectionIndexer::new();
    indexer.index("/home/luxluth/Music/".into());
    println!("{:#?}", indexer.collection.index.keys);

    let e = Instant::now();
    let res = indexer.collection.index.search("Kendrick Lamar", 1);
    eprintln!("{}ms", e.elapsed().as_millis());
    eprintln!("{:?}", res);

    for (key, _) in res {
        match key {
            IdKey::SongTitle(id) => {
                eprintln!("{:?}", indexer.collection.songs.get(id));
            }
            IdKey::ArtistName(id) => {
                eprintln!("{:?}", indexer.collection.artists.get(id));
            }
            IdKey::AlbumName(id) => {
                eprintln!("{:?}", indexer.collection.albums.get(id));
            }
            IdKey::Unknown => unreachable!(),
        }
    }
}
