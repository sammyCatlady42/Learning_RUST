# Schaltjahr

Das Programm hat eine Methode, 
```rust
fn is_leap_year(year: i32) -> bool
```
die ein int (i32) Jahr einliest, überprüft, ob es ein Schaltjahr ist oder nicht und gibt dementsprechend ein true oder false zurück.
Die Vorraussetzungen für ein Schaltjahr ist :
```
on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
```
Durch die *main()* wird jeder Eintrag vom Array über die Funktion `is_leap_year` überprüft, ob es nun ein Schaltjahr ist oder nicht. Gleichzeitig gibt die Schleife die überprüften Werte aus. Gefundene Schaltjahre werden mit einem '*' zusätzlich ausgegeben. Dafür wird ein match benutzt.

ENDE :D
