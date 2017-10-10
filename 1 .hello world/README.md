# Hello World

Rust hat ein fantastischen Paketmanager: Cargo. Dieser beherscht alle Automatisierungs- und Dependency-Management-Details für Sie, so dass Sie keine Sorgen mehr darüber machen müssen, wenn Sie ein Projekt erstellen, bauen oder aktualisieren.

## Ausführen von Rust Programmen
1. Führen Sie **cargo build** und/oder **cargo run** aus.
`Cargo.lock` wird erstell durch **cargo build**.
`Cargo.toml` wird dazu benötigt.

## Git
2. Im Root Ihres Repositories ist eine `.gitignore` Datei enthalten.
   > **cargo build** erstellt die nötigen Bibliotheken und Binaries im Unterverezeichnis `target/`. Die dafür aus dem Internet geladenen Programmversionen werden in `Cargo.lock` gespeichert. Das `target/` Verzeichnis benötigen wir nicht im git Repository, da dies jederzeit wieder per **cargo build** erstellt werden kann. Mit der `.gitignore` Datei können Sie **git** anweisen, bestimmte Dateien/Verzeichnisse zu ignorieren.

3. Fügen Sie nun über **git add** und **git commit** neue Dateien bzw. geänderte Dateien in Ihr Repository.

## Cargo doc
4. Dokumentieren Sie in Englisch, um über **cargo doc** die Dokumentation erstellen zu lassen. Damit Ihre Funktion auch in der Dokumentation auftaucht ist sie in obiger Notation als public (*pub*) markiert.

5. Der Aufruf von **cargo doc** erstellt nun den Ordner `target/doc/`. Darin liegen Ihre Domunetationsdateien sowie die Dokumentation von externen Modulen (crates). Schauen Sie sich Ihre Dokumentation im Browser an. Öffnen Sie dazu die Datei `index.html` im Ordner Ihres crates `task1`.

### Abgabe task

1. Erstellen Sie einen Branch.

2. Wenn Sie mit Ihrer Bearbeitung zufrieden sind pushen Sie ihren lokalen branch nach github und erstellen Sie auf github einen Pull-Request(PR) dafür.
