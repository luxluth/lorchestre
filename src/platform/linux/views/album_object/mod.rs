use adw::subclass::prelude::ObjectSubclassExt;
use gtk::glib;

use crate::track::Album;
mod imp;

glib::wrapper! {
    pub struct AlbumObject(ObjectSubclass<imp::AlbumObject>);
}

impl AlbumObject {
    pub fn new(album: Album) -> Self {
        let obj: Self = glib::Object::new();
        imp::AlbumObject::from_obj(&obj).album.replace(Some(album));
        obj
    }

    pub fn album(&self) -> Option<Album> {
        imp::AlbumObject::from_obj(self).album.borrow().clone()
    }
}
