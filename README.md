![banner](./assets/banner.svg)

_The only music player you'll ever need_

![Grande Vue](./assets/grande-vue.png)

## TODO

- [x] Use local files (music directory) as media source
- [ ] ~~Use Spotify as media source~~ (not planned)

## RANT

The description is **_The only music player you'll ever need_**.
I don't think this statement is true — or at least not true yet.
I'm still working on how to improve it.

What I have in mind can be summarized in a few points.

In terms of features:

- [ ] Audio file metadata editor
- [ ] Tag marks (for filtering)
- [ ] Automatic playlists
  - Based on genre, artists
  - _Composable_ logic

Technical additions/improvements:

- [ ] A better UI library
  - Tauri with WebKit is absolutely great to work with.
    It's easy and fast to implement new features. But frustrations arise with the growing
    complexity of the project. I feel like adding more features will slow down
    the overall experience.
    I want everything to be fast and snappy. Dropping the web side would also
    allow me to drop the web server that is spawned to serve audio files.
    The app reads local files — no need to add this much complexity.

## Windows support

> [!IMPORTANT]
> I tried to install the app on Windows and make it work, but with no success.
> I mainly develop my app for Unix systems. If you know how to fix the problem,
> feel free to submit a PR.
