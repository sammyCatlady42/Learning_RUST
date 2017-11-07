# Hello World

Rust hat einen fantastischen Paketmanager: Cargo. Dieser beherrscht alle Automatisierungs- und Dependency-Management-Details, so dass man keine Sorgen mehr darüber machen muss, wenn ein Projekt erstellt, gebaut oder aktualisiert wird.

## Ausführen von Rust Programmen
**cargo build** erstellt `Cargo.lock`.
`Cargo.toml` wird dazu benötigt.
Wenn das Programm Warnings oder Errors hat werden diese angezeigt.
Wenn keine Fehler vorhanden sind und die Warnungen nicht relevant sind **cargo run** ausführen.
Auch legitim ist **cargo run** auszuführen, ohne **cargo build**.
Denn es macht beides.

## Cargo doc
**cargo doc** erstellt eine Dokumentation. Damit Funktionen auch in der Dokumentation auftauchen sollten sie als public (*pub*) markiert werden.

Der Aufruf von **cargo doc** erstellt nun den Ordner `target/doc/`. Darin liegen die Domunetationsdateien sowie die Dokumentation von externen Modulen (crates). Um die Dokumentation im Browser zu sehen, einfach die Datei `index.html` im Ordner des crates öffnen.


ENDE :D
