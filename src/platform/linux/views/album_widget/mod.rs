use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct AlbumWidget(ObjectSubclass<imp::AlbumWidget>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl AlbumWidget {
    // pub fn set_album(&self, album: AlbumObject) {
    //     self.set_property("album", Some(album));
    // }
}

impl Default for AlbumWidget {
    fn default() -> Self {
        glib::Object::new()
    }
}
