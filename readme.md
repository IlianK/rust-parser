# rust-parser

###C++ vs Rust am Beispiel eines einfachen Parsers für arithmetische Ausdrücke

Der Rust-Parser basiert auf dem C++-Parser Projekt im Rahmen des Softwareprojekts 
im 2. Semester und dient dazu die Unterschiede zwischen der objektorientierten Umsetzung 
in C++ und der datenorientierten Gestaltung in Rust aufzuzeigen.
Dabei wird ein Hauptaugenmerk auf die speziellen Ownership-Rechte in Rust.

Als Einführung in die Programmiersprache-Rust habe ich 
folgende [Literatur-Quelle](https://doc.rust-lang.org/book/) benutzt, 
um mich mit den grundlegenden Besonderheiten und Unterschiede bekannt zu machen.

###Die Quelldateien:
+ **ast.rs:** Stellt einen abstrakten Syntax Baum zur Verfügung,der durch die pretty-printing Funktion in einen String umgewandelt wird
+ **tokenizer.rs:** Zerlegt den Eingabe-String in logisch zusammengehörige Einheiten, den Tokens
+ **parser.rs:** Nimmt als Input die Tokens vom Tokenizer und generiert den AST nach der definierten Grammatik
+ **main.rs:** Allgemeine Testklasse
---------
##Inhalte
1. Grundlagen
2. Implementierung
3. ...
____
##Grundlagen
Als erstes werde ich auf die grundlegenden Strukturen der Rust-Dateien, darüber hinaus auf das 
Package-Management-System Cargo eingehen und diese mit den in C++ zur Verfügung gestellten Funktionalitäten vergleichen.

###Header und Source Files
Der erste Unterschied zwischen C++ und Rust liegt in der Strukturierung der Dateien.
In C++, sowie C werden in den Header-Files (.h und .hpp) Klassen deklariert, definiert, 
Signaturen für Funktionen und auch Makros festgelegt:
```c++
typedef std::shared_ptr<Exp> EXP;

EXP newInt(int i);
EXP newPlus(EXP l, EXP r);
EXP newMult(EXP l, EXP r);
```

Die Quelldateien (.c, .cpp) können die für sie benötigten Deklarationen der Header-Files 
durch #include implementieren und dabei die Komplexität und Details durch private Felder vor dem Anwender verstecken.

Im Gegensatz dazu, können in Rust sowohl Structs (Vergleich: C++ Klassen), Funktionen 
und deren Implementierung in einer einzigen .rs Datei enthalten sein. 
Felder, Funktionen und Klassen sind standardmäßig privat, 
können jedoch durch die Modul-Strukturierung und dem expliziten Setzen des Schlüsselwortes ```pub``` öffentlich gemacht 
und dadurch von anderen .rs Files genutzt werden.
```Rust
pub struct Tokenizer{
    pos: usize,
    s: String,
    pub token: Token
}
```
Hier am Beispiel das äquivalent zur Tokenizer Klasse in C++. 
Die Felder pos und s sind privat. Das Feld token jedoch nicht, da die Datei parser.rs in den parser_ Funktionen 
auf das ```token``` zugreifen muss:
```rust
fn parse_e2(&mut self, left: Box<Exp>) -> Option<Box<Exp>>{
    return match self.t.token  //token muss public sein
    //...
```


###Modules
Module sind eine Ansammlung von Funktionen, Structs, Traits und Implementierungen.
Das Modul-System hilft dabei den Code in Gruppen zu strukturieren,
sowie Gültigkeitsbereiche und Datenschutz (public/ private) zu organisieren.
Andere .rs Dateien können diese Module durch ```use``` in ihre Datei einbinden, um deren Funktionalität zu nutzen.

```Rust
mod ast;
mod tokenizer;
mod parser;
```
Zur Weiteren Strukturierung des Projektes gilt zu erwähnen, dass mehrere Module eines Projektes ein Crate ergeben.

###Crates
Rust-Crates sind mit Bibliotheken oder Packages zu vergleichen wie man sie aus C++ kennt.
Sie können je nach Projekt eine geteilte Bibliothek oder ein ausführbares Programm sein, die durch den Rust Compiler
**rustc.** kompiliert werden. Jedes Crate hat implizit ein root-Modul, in der der
Compiler mit der Code-Ausführung startet.
Das Root-Modul kann wiederum in Sub-Module unterteilt werden kann, wodurch das Projekt strukturiert und organisiert werden kann.
Der **crate root** ist in unserem rust_parser die main.rs Datei mit den Modulen ast, parser und tokenizer.
Zur Veranschaulichung bietet Rust ein hilfreiches Feature das eine Dokumentation das Projekt erstellt.
```
cargo doc --open
```
![Doc1](pictures/doc1.JPG)
![Doc2](pictures/doc2.png)
Dabei sind alle Bestandteile des Projektes, inklusive alle Structs, Enums,
Funktionen und deren Implementierung zu sehen.

Außerdem bietet cargo noch die Möglichkeit seine Implementierungen zu veröffentlichen mit dem Befehl:
```
cargo publish
```
Hierbei wird jedoch ein Account bei [crates.io](https://crates.io/) gebraucht (sowie ein GitHub Account),
wodurch dann mithilfe eines einzigartigen API-Tokens, ein eigenes Crate veröffentlicht werden kann.

Will man nun ein externes Crate nutzen oder nur bestimmte Module davon,
kann man diese mit dem Schlüsselwort ```use``` einbinden:
```rust
use std::any::type_name; //Returns the name of a type as a string slice
```
Hierbei ist std das Crate, any das Modul und type_name die Funktion.
![extern_crate_example](pictures/extern_crate_example.JPG)

**Crates**, zu Deutsch "Kisten" sind wie erwähnt Bibliotheken oder Packages und können mithilfe von Rust's
Package-Manager **Cargo**, als "Fracht" verteilt werden. 

###Package manager
[Cargo](https://doc.rust-lang.org/cargo/) ist das offizielle Package-Management System
für Rust und auch wenn es für C++ viele gute Package-Manager gibt wie
beispielsweise [Conan](https://conan.io/) und
[Vcpkg](https://vcpkg.io/en/index.html) regelt Cargo das meiste für den Nutzer,
wohingegen in C++ die meisten Konfigurationen selbst in der CmakeFile
durchgeführt werden müssen.
```cargo install``` lädt die Bibliotheken,
```cargo new rust_parser``` erstellt ein neues Projekt und
```cargo build``` baut das Projekt auf.

--------

####Namespacing in Rust
####crates and modules

###C++ Klassen und Rust Structs
####Implementierung der Structs

###Rust Default Konstruktor
####Überladung new
####virtuelle Methoden

###Rust Ownership
####C++ shared Pointer
####Parameter self (self (mut) vs & self (mut))
####helper function

###"Vererbung" in Rust
####Traits
####Enum

###Option<T> vs Optional<T>
####None
####Some
####Methoden (Gegenüberstellung)

###Unbekannter Return Type
####Statische Sprache Rust
####Dyn Box
####Enum Branch Methode

###Anderes
####String Indexing
####C++ switch-case vs Rust pattern matching
####format! Macro
####type_of 

