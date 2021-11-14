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
durch #include implementieren.

In C++ können Klassen ihre Methoden und Felder für andere Klassen, welche von ihr erben,
sichtbar machen oder diese verstecken mit den Schlüsselwörtern public, protected und private.

* ```public``` kann intern und von jedem außerhalb der Klasse verwendet werden
* ```protected``` kann klassenintern und von Unterklassen verwendet werden
* ```private``` kann nur innerhalb der Klasse verwendet werden

Auch gibt es die Möglichkeit eine andere Klasse als "Freund" zu erlauben auf
alle privaten und geschützten Mitglieder der Klasse zuzugreifen.


Im Gegensatz dazu, können in Rust sowohl Structs (Vergleich: C++ Klassen), Funktionen 
und deren Implementierung in einer einzigen .rs Datei enthalten sein.

Auch sind die Zugriffsmodifikatoren in Rust etwas simpler.
Soll ein Struct außerhalb des Moduls sichtbar sein benutzt man das Schlüsselwort ```pub```.
Wenn nicht sind sie standardmäßig nur innerhalb des Moduls und für Submodule sichtbar.

```Rust
pub struct Tokenizer{
    pos: usize,
    s: String,
    pub token: Token
}
```
Hier am Beispiel das Äquivalent zur Tokenizer Klasse in C++. 
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
<p float="left">
    <img src="pictures/doc1.JPG" width=50% alt = "doc1" height=50%>
    <img src="pictures/doc2.png" width=50% alt = "doc2" height=50%>
</p>

Dabei sind alle Bestandteile des Projektes, inklusive alle Structs, Enums,
Funktionen und deren Implementierung zu sehen.

Außerdem bietet cargo noch die Möglichkeit seine Implementierungen zu veröffentlichen mit dem Befehl:
```
cargo publish
```

Hierbei wird jedoch ein Account bei [crates.io](https://crates.io/) gebraucht (sowie ein GitHub Account),
wodurch dann mithilfe eines einzigartigen API-Tokens, ein eigenes Crate veröffentlicht werden kann.

Will man ein externes Crate nutzen, nur bestimmte Module oder Funktionen davon,
kann man diese ebenfalls mit dem Schlüsselwort ```use``` einbinden:
```rust
use std::any::type_name; //Returns the name of a type as a string slice
```

Hierbei ist std das Crate, any das Modul und [type_name](https://doc.rust-lang.org/std/any/fn.type_name.html) die Funktion.

<img src="pictures/extern_crate_example.JPG" alt="extern_crates_example" width=50% height=50%>

###Namensräume
C++-Namensräume werden dazu benutzt um Funktionen, Variablen und Klassen zu gruppieren, damit der Compiler sie von anderen gleichnamigen Signaturen unterscheiden kann.
In Rust wird dies durch die Module geregelt, 
wobei jede .rs Datei implizit ein Modul ist, mit gleichem Namen wie die Datei.
Die Datei parser.rs braucht viele Funktionalitäten von tokenizer.rs: 
Zur Definition des Parser-Structs, 
Zugriff auf die helper() Funktion um einen neuen Tokenizer zuzuweisen 
und auf das Token enum um pattern-matching ausführen zu können.
Der Tokenizer (mit seiner Implementierung) und das Token enum kann
folgendermaßen inkludiert werden:
```
use crate::tokenizer::{Tokenizer, Token};
```
Allerdings besteht das ganze Modul tokenizer.rs allein aus dem Tokenizer und dem Token enum.
Deshalb ist es sinnvoll und kürzer einfach das ganze Modul zuzufügen, was mit dem Stern-Operator möglich ist:
```
use crate::tokenizer::*;
```

Von ast.rs braucht parser.rs allerdings nur das enum Exp um die 
jeweiligen arithmetischen Ausdrücke zu erzeugen.
```
use crate::ast::Exp;
```

Explizite Module sind auch möglich, werden aber in dem Projekt nicht genutzt. 
Meistens braucht man sie wenn es nicht sinnvol ist, dass die gesamte Funktionalität eines Moduls 
eine eigene .rs Datei ist.


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
```cargo new rust_parser``` erstellt das Projekt und
```cargo build``` baut das Projekt auf.

--------
##Implementierung
In diesem Abschnitt werde ich erst allgemein auf die Rust-Structs und deren Implementierung eingehen, 
die in jeder .rs Datei ihre Verwendung finden und danach einzeln auf 
die Quelldateien.

###Rust Structs
In C++ gibt es sowohl Klassen als auch Structs mit Feldern und Methoden.
Dabei ist der Unterschied nicht sehr groß. Das Vererbungsprinzip gilt für beide.
Standardmäßig sind Felder in Structs public und Klassen private, können jedoch Felder und Methoden mit 
verschiedenen Zugriffsbeschränkungen haben.

Rust hat nur Strukturen (Structs). Diese bestehen aus einer Definition, 
die die Felder, mit deren Zugriffsrecht festlegt und für die anschließend mit dem Schlüsselwort
```impl ``` Funktionen implementiert werden können.

In C++ wurde eine Basis-Klasse für die Expressions Exp definiert, von der dann 
IntExp, PlusExp und MultExp erben und durch ihre Felder val bzw. e1 und e2 erweitern.

Diese benutzerdefinierten Datentypen können in Rust jeweils als Struct 
zusammengefasst werden:

```rust
pub struct Int {
    pub val: i32
}

pub struct Plus<T:Exp> {
    pub e1: T,
    pub e2: T
}

pub struct Mult<T:Exp> {
    pub e1: T,
    pub e2: T
}
```
####"Vererbung" durch Traits
In Rust gibt es jedoch keine Vererbung wie man es aus C++ oder Java kennt. 
Es ist nicht möglich Felder eines Structs zu erweitern.
Das nächste, was eine Vererbung wie in den objektorientierten Sprachen simulieren kann, sind 
die sogenannten Traits, die den in Java vorhandenen Interfaces ähneln.

Mit Traits können Funktionen für bestimmtes Verhalten definiert werden. 
Im rust-parser könnte man die Funktionen zum evaluieren (eval()) und 
pretty_printing (pretty()) in einem Trait Exp zusammenfassen.

Die Structs Int, Plus und Mult können dieses Verhalten aufnehmen oder "erben" 
und durch ```impl ... for``` anpassen.

```rust
pub trait Exp {
    fn eval(&self) -> i32;
}

pub struct Int {
    pub val: i32
}

pub struct Plus<T:Exp> {
    pub e1: T,
    pub e2: T

}

impl Exp for Int {
    fn eval(&self) -> i32 {
        return self.val
    }
}

impl<T:Exp> Exp for Plus<T> {
    fn eval(&self) -> i32 {
      return self.e1.eval() + self.e2.eval()
    }

    //...
    
}
```
Bei Rust handelt es sich um eine statisch typisierte Sprache, 
bei der der Typ jeder Variable bei der Kompilierung bekannt sein muss.

Normalerweise werden alle Werte auf dem Stack allokiert, da sie statisch 
und damit bekannt sind. Um dennoch Rückgaben zu realisieren, die zur Laufzeit unbekannt gibt es das "Boxed"
Modell. ```Box<T>``` ist eine Art Smart Pointer der Heap-Speicher für den
generischen Typ ```T``` allokiert. 
Für die parser_ Funktionen wäre somit der Rückgabetyp das Exp-Trait: ```Option<Box<dyn Exp>>```
Das Schlüsselwort ```dyn``` steht für **dynamic** und ist nicht unbedingt notwendig.
Jedoch wird im
[RFC-2113 Standard](https://github.com/rust-lang/rfcs/blob/master/text/2113-dyn-trait-syntax.md)
folgender Grund für die Notwendigkeit angegeben:

> "impl Trait is going to require a significant shift in idioms 
and teaching materials all on its own, and “dyn Trait vs impl Trait” 
is much nicer for teaching and ergonomics than “bare trait vs impl Trait”"

Der Speicher wird automatisch durch den Destruktor
wieder freigegeben, wenn die Scope in der die Box definiert wurde verlassen wird.

####Branch-Based Inheritance
Für den rust-parser reicht jedoch eine Branch-basierte Umsetzung der Exp-Vererbung.
In Rust gibt es wie in C++ Enumerationen (enums). Wird ein Enum nun als Rückgabetyp 
festgelegt, ist zwar noch immer unbekannt welches der Enum-Varianten am Ende zurückgegeben wird, es ist jedoch
klar, eines davon wird zurückgegeben. Deswegen wird soviel Speicher allokiert wie die 
größte Enum-Variante benötigt. Daher muss auch der Rückgabetyp nicht mehr in ```Box``` eingehüllt werden. 

```rust
pub enum Exp {
    Int {
        val: i32,
    },
    Plus {
        e1: Box<Exp>,
        e2: Box<Exp>,
    },
    Mult{
        e1: Box<Exp>,
        e2: Box<Exp>,
    },
}
```
Die Instanziierung der Expressions findet an folgenden Stellen statt:
```rust
fn parse_e2(&mut self, left: Box<Exp>) -> Option<Exp> {
    //...
    Parser::parse_e2(self, Exp::Plus { e1: Box::from(left), e2: Box::from(right.unwrap()) })
    //...
}
fn parse_t2(&mut self, left: Box<Exp>) -> Option<Exp> {
    //...
    Parser::parse_t2(self, Exp::Mult { e1: Box::from(left), e2: Box::from(right.unwrap()) })
    //...
}
fn parse_f(& mut self) -> Option<Exp> {
    return match &self.t.token {
        Token::ZERO => {
            //...
            Some(Exp::Int { val: 0 })
        }
        //... same for Token::ONE and Token::TWO
    }
}
```
Bei der Instanziierung von Plus und Mult ist ```Box``` allerdings notwendig, denn 
würde man die Structs folgendermaßen konstruieren:
```rust
pub enum Exp {
    Int {
        val: i32,
    },
    Plus {
        e1: Exp,
        e2: Exp,
    },
    Mult{
        e1: Exp,
        e2: Exp,
    },
}
```
würde das theoretisch funktionieren wenn man unbegrenzten Speicher hätte.
Doch der Compiler warnt uns zu recht:

<img src="pictures/error_infinite_size.JPG" width=50% alt = "error_infinite_size" height=50%>

Um zu bestimmen wie viel Speicher für einen Exp-Ausdruck gebraucht wird, geht Rust durch
jede Möglichkeit durch und richtet sich nach der Variante, die am meisten Speicher braucht.
Für Int ist das kein Problem, denn wir definieren, dass wir lediglich soviel Platz für
die größt mögliche 32bit-Integer brauchen. 
Bei Plus und Mult sieht das anders aus.
Denn durch diese Definition würde man für jede mögliche Kombination Speicher brauchen.
Da man endlos Ausdrücke innerhalb Ausdrücke haben kann, würde 
Exp aufgrund der Rekursion eine unendliche Größe brauchen. 

Zum Glück wird uns ein hilfreicher Hinweis gegeben:

<img src="pictures/error_help.JPG" width=50% alt = "error_help" height=50%>

Es wird vorgeschlagen eine "Indirection" zu nutzen. Das bedeutet, wir speichern
den Wert nicht direkt, sondern speichern einen Pointer. Da die Größe eines Pointers
von der Größe der Daten auf die sie zeigt, unabhängig ist, 
löst Box das Problem des unendlichen, rekursiven Speichers.


Die Implementierung der eval() und pretty() Funktionen ändert sich leicht.
Da nun als Rückgabe einer der drei Enum-Varianten Int, Plus oder Mult entstehen kann,
wird ein pattern-matching durchgeführt um herauszufinden welche der eval() und pretty() Funktionen
gebraucht wird:

```rust
impl Exp {
    pub fn eval(self: &Exp) -> i32 {
        return match self {
            Exp::Int { val } => *val,
            
            Exp::Plus { e1, e2 } => {
                e1.eval() + e2.eval()
            },
            
            Exp::Mult { e1, e2 } => {
                e1.eval() * e2.eval()
            }
        }
    }
    //...pretty
}
```
####Pattern-Matching
```Match``` ähnelt dem Switch-Case Statement von C++. Genau wie bei switch-case
wird eine Variable mit einer Menge von bekannten Wert-Möglichkeiten verglichen 
und je nach Ergebnis ein spezifischer Code ausgeführt. Beide haben einen Default-Fall.
In C++ gekennzeichnet mit ```default:``` und in Rust mit ```__ => {}```
Rust's pattern-match geht jedoch noch weiter. **switch** Statements können nur
numerische oder boolsche Werte vergleichen, doch **match** funktioniert sowohl mit
Integer und Boolschen Werten, als auch mit weiteren Enums, Tupeln, Arrays und 
eigenen Structs.

In diesem Fall gibt es eine eval()-Methode mit einer Exp-Referenz als Eingabe-Paramter,
welcher mit allen Fällen verglichen wird und die passende Rückgabe liefert.

Im parser.rs wird in der main.rs das pattern-match initiiert:
```rust
fn display(e: Option<Exp>){
    if e.is_none() {
        println!("nothing \n");
    }
    else{
        let str: String = e.unwrap().pretty() + "\n"; // e bestimmt pretty()-Rückgabe
        println!("{}", str);
    }
}
```
Die display()-Funktion bekommt den ast vom Parser. pretty() nimmt als Parameter self entgegen.
Jede Funktion, die ein (mut) self oder &(mut) self als Parameter besitzt, kann von der an die 
Struktur gebundene Variable mit dem Punktoperator aufgerufen werden.
Variable ```e``` entspricht daher dem
```self``` im pattern-matching und gibt den passenden geklammerten oder ungeklammerten Ausdruck zurück.

#### Einschub Konstruktoren
In C++ gibt es zwei Klassen: Zum Einen die Tokenize Klasse mit den Feldern pos für die Position und s für 
den String, welcher in Tokens zerlegt werden soll. Zum Anderen die Tokenizer Klasse, die von Tokenize erbt 
und sie um das Feld token erweitert. 
In Rust können diese zwei Klassen in einem Struct zusammengefasst werden:

```rust
pub struct Tokenizer{
    pos: usize,
    s: String,
    pub token: Token
}
```
Anschließend folgt durch ``impl``` ein Code-Block in der man die zugehörigen Funktionen binden kann.

```rust
impl Tokenizer {
    pub fn new(text: &str) -> Tokenizer { //
        Tokenizer {
            pos: 0,
            s: text.parse().unwrap(),
            token: Token::DEFAULT
        }
    }
    
    //...
}
```
Die new()-Funktion ist meist optional und eher zur Vereinfachung der Instanziierung gedacht.
In Rust muss nämlich in Gegensatz zu C++ kein Konstruktor definiert werden.
Möchte man einen neuen Tokenizer erstellen könnte man das auch direkt ohne Methodenaufruf machen,
indem man die Felder des Tokenizers einen Wert zuweist.

```rust
let token = Tokenizer{pos:0, s: "1 + 0", Token::DEFAULT }
```

Damit gibt es in Rust eine Art impliziten Konstruktor für jedes Structs mit Feldern.
Ein Unterschied zu C++ ist allerdings, dass der Konstruktor nicht überladen werden kann.
Braucht man mehrere Konstruktoren (z.B. einmal ohne und einmal mit (mehreren) Parameter) ist es notwendig auch mehrere new() 
Funktionen zu haben, die sich im Namen unterscheiden. 

Die direkte Instanziierung macht in unserem rust-parser an der Stelle keinen Sinn, denn der Tokenizer
wird nur einmal als Struct-Feld im Parser benötigt. 
Schauen wir uns den C++ Code an:
```c++
class Tokenizer : Tokenize {
public:
    Token_t token;
    Tokenizer(string s) : Tokenize(s) { token = next(); }
    void nextToken() {
        token = next();
    }
};
```
Variable token bekommt seine erste Zuweisung bereits im Konstruktor durch die next() Funktion.
In Rust ist das nicht so einfach umzusetzen, da das Prinzip des Ownerships spezielle Betrachtung erfordert.

###Rust Ownership

####C++ shared Pointer
####Parameter self (self (mut) vs & self (mut))
####helper function

###Option<T> vs Optional<T>
####None
####Some
####Methoden (Gegenüberstellung)

###Anderes
####String Indexing
####format! Macro
####type_of 

