use adw::subclass::prelude::ObjectSubclassExt;
use gtk::glib;

use crate::track::Album;
mod imp;

glib::wrapper! {
    pub struct AlbumObject(ObjectSubclass<imp::AlbumObject>);
}

impl AlbumObject {
    pub fn new(album: Album, cover_path: Option<String>, artist_name: Option<String>) -> Self {
        let obj: Self = glib::Object::new();
        imp::AlbumObject::from_obj(&obj).album.replace(Some(album));
        imp::AlbumObject::from_obj(&obj)
            .cover_path
            .replace(cover_path);

        imp::AlbumObject::from_obj(&obj)
            .artist_name
            .replace(artist_name);
        obj
    }

    pub fn album(&self) -> Option<Album> {
        imp::AlbumObject::from_obj(self).album.borrow().clone()
    }

    pub fn cover_path(&self) -> Option<String> {
        imp::AlbumObject::from_obj(self).cover_path.borrow().clone()
    }

    pub fn artist_name(&self) -> Option<String> {
        imp::AlbumObject::from_obj(self)
            .artist_name
            .borrow()
            .clone()
    }
}
