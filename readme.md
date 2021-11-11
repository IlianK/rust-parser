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

###Header und Source Files
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

