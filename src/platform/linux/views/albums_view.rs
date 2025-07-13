use crate::platform::linux::views::album_widget::AlbumWidget;
use crate::track::MusicCollection;
use gtk::gio::ListStore;
use gtk::{ListItem, ScrolledWindow, prelude::*};
use gtk::{NoSelection, SignalListItemFactory};

use super::AlbumObject;

#[allow(non_snake_case)]
pub fn AlbumsView(collection: &MusicCollection) -> adw::Bin {
    let model = ListStore::new::<AlbumObject>();

    for (_, album) in &collection.albums {
        let cover_path = album
            .cover
            .as_ref()
            .and_then(|id| collection.covers.get(id))
            .map(|cover| cover.get_path().to_string_lossy().to_string());

        let artist_name = album
            .artist
            .as_ref()
            .and_then(|id| collection.artists.get(id))
            .map(|artist| artist.name.clone());
        eprintln!("{album:?}");

        model.append(&AlbumObject::new(album.clone(), cover_path, artist_name));
    }

    let item_factory = SignalListItemFactory::new();

    item_factory.connect_setup(|_, list_item| {
        let list_item = list_item
            .clone()
            .downcast::<ListItem>()
            .expect("Failed to downcast to ListItem");

        list_item.set_child(Some(&AlbumWidget::default()));
    });

    item_factory.connect_bind(move |_, list_item| {
        let list_item = list_item
            .clone()
            .downcast::<ListItem>()
            .expect("Failed to downcast to ListItem");

        let item = list_item
            .item()
            .unwrap()
            .downcast::<AlbumObject>()
            .expect("Cannot downcast to AlbumWidget");

        let child = list_item
            .child()
            .unwrap()
            .downcast::<AlbumWidget>()
            .unwrap();

        child.set_album(item);
    });

    let selection = NoSelection::new(Some(model));
    let list = gtk::GridView::builder()
        .model(&selection)
        .factory(&item_factory)
        .margin_bottom(20)
        .margin_start(10)
        .margin_top(10)
        .margin_end(10)
        .enable_rubberband(true)
        .build();

    let scrolled = ScrolledWindow::builder().child(&list).build();
    adw::Bin::builder()
        .child(&scrolled)
        .hexpand(true)
        .vexpand(true)
        .build()
}
