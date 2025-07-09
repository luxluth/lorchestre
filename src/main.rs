use lorchestre::track::MusicCollection;

fn main() {
    let mut collection = MusicCollection::default();
    collection.index("/home/luxluth/Music/".into());
    eprintln!("{:#?}", collection.artists);
}
