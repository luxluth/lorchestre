use log::info;
use lorchestre::{Lorch, track::MusicCollectionIndexer};

use cstr::cstr;
use qmetaobject::prelude::*;

#[derive(QObject, Default)]
struct Lorchestre {
    base: qt_base_class!(trait QObject),
}

fn main() {
    qmetaobject::log::init_qt_to_rust();
    let _ = env_logger::try_init();

    let mut indexer = MusicCollectionIndexer::new();
    if Lorch::cache_path().exists() {
        indexer.load_from_cache();
    } else {
        indexer.index("/home/luxluth/Music/".into());
        indexer.save();
    }

    qml_register_type::<Lorchestre>(cstr!("Lorchestre"), 1, 0, cstr!("Lorchestre"));
    let mut engine = QmlEngine::new();
    let collection = indexer.collection.index.search("Risk", 10);
    println!("{collection:?}");

    engine.load_data(include_str!("./app.qml").into());

    engine.exec();
}
