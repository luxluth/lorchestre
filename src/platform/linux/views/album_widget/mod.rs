use gtk::glib::object::ObjectExt;
use gtk::{glib, prelude::*, subclass::prelude::*};

use super::AlbumObject;
use crate::track::{Album, MusicCollection};

mod imp;

glib::wrapper! {
    pub struct AlbumWidget(ObjectSubclass<imp::AlbumWidget>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl AlbumWidget {
    pub fn new(collection: &MusicCollection, album: &Album) -> Self {
        let artist_name = album
            .artist
            .as_ref()
            .and_then(|id| collection.artists.get(id))
            .map(|artist| artist.name.clone());

        let cover_path = album
            .cover
            .as_ref()
            .and_then(|id| collection.covers.get(id))
            .map(|cover| cover.get_path().to_string_lossy().to_string());

        let widget: Self = glib::Object::builder()
            .property(
                "album",
                Some(AlbumObject::new(album.clone(), cover_path, artist_name)),
            )
            .build();

        widget
    }

    pub fn set_album(&self, album: AlbumObject) {
        self.set_property("album", Some(album));
    }
}

impl Default for AlbumWidget {
    fn default() -> Self {
        glib::Object::builder()
            .property("album", None::<AlbumObject>)
            .build()
    }
}
