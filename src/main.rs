use lorchestre::track::MusicCollectionIndexer;

fn main() {
    let mut indexer = MusicCollectionIndexer::default();
    indexer.index("/home/luxluth/Music/".into());
    println!("{:#?}", indexer.collection);

    let config = bincode::config::standard();
    let encoded: Vec<u8> = bincode::encode_to_vec(&indexer.collection, config).unwrap();
    eprintln!("{} bytes", encoded.len());
}
