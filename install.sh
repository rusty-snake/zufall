#!/bin/bash
install -v -Dm644 data/icon.svg.png "$DESTDIR"/usr/share/pixmaps/zufall.png
install -v -Dm644 data/zufall.desktop "$DESTDIR"/usr/share/applications/zufall.desktop
install -v -Dm755 target/release/zufall "$DESTDIR"/usr/bin/zufall
install -v -Dm644 LICENSE "$DESTDIR"/usr/share/licenses/zufall/LICENSE
