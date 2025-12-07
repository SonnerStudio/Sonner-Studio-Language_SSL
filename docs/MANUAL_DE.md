# ⚡ SSL v5.0 Handbuch

> **Vollständige Feature-Referenz & API-Dokumentation**

---

### Sprache auswählen / Select Language

| 🇩🇪 🇦🇹 🇨🇭 <br> **DEUTSCH** | 🇬🇧 🇺🇸 <br> [**ENGLISH**](MANUAL_EN.md) |
| :---: | :---: |
| *(Ausgewählt / Selected)* | *(Wechseln / Switch)* |

---

## Inhaltsverzeichnis

1. [Kernsprache](#1-kernsprache)
2. [Standardbibliothek](#2-standardbibliothek-stdlib)
3. [Erweiterte Features](#3-erweiterte-features)
4. [GUI Framework](#4-gui-framework)
5. [Entwicklungstools](#5-entwicklungstools)
6. [Quantum Computing](#6-quantum-computing)
7. [Distributed Computing](#7-distributed-computing)
8. [AI/ML](#8-aiml)

---

## Über dieses Handbuch

Dieses Handbuch ist die offizielle Referenz für die **Sonner Studio Language v5.0 (Self-Hosting Edition)**. Es deckt alle Sprachfeatures, die Standardbibliothek und die spezialisierten Module (Quantum, AI, etc.) ab.

Alle gezeigten Code-Beispiele sind valider SSL v5.0 Code.

---

## 1. Kernsprache

### Variablen

```ssl
let immutable = 42       // Unveränderlich
var mutable = 0          // Veränderlich
const PI = 3.14159       // Konstante
```

### Datentypen

| Typ | Beispiel | Beschreibung |
|-----|----------|--------------|
| `Int` | `42` | 64-bit Integer |
| `Float` | `3.14` | 64-bit Floating Point |
| `Bool` | `true` | Boolean |
| `String` | `"Hello"` | UTF-8 String |
| `Char` | `'a'` | Unicode Character |
| `List<T>` | `[1, 2, 3]` | Dynamisches Array |
| `Map<K, V>` | `{"a": 1}` | Hash Map |
| `Option<T>` | `Some(42)` | Optional Value |
| `Result<T, E>` | `Ok(value)` | Error Handling |

### Funktionen

```ssl
// Einfache Funktion
fn add(a: Int, b: Int) -> Int {
    a + b
}

// Generische Funktion
fn first<T>(list: List<T>) -> Option<T> {
    if list.is_empty() { None } else { Some(list[0]) }
}

// Lambda / Closure
let double = |x| x * 2
let result = [1, 2, 3].map(|x| x * x)

// Higher-Order Function
fn apply_twice<T>(f: fn(T) -> T, x: T) -> T {
    f(f(x))
}
```

### Kontrollfluss

```ssl
// If-Expression
let max = if a > b { a } else { b }

// Match (Pattern Matching)
match value {
    0 => "zero"
    1..=9 => "einstellig"
    n if n < 0 => "negativ"
    _ => "groß"
}

// Loops
for item in list { ... }
for i in 0..10 { ... }
while condition { ... }
loop { if done { break } }
```

### Structs und Enums

```ssl
// Struct
struct Point {
    x: Float
    y: Float
}

impl Point {
    fn new(x: Float, y: Float) -> Point {
        Point { x, y }
    }
    
    fn distance(&self, other: &Point) -> Float {
        let dx = self.x - other.x
        let dy = self.y - other.y
        math::sqrt(dx*dx + dy*dy)
    }
}

// Enum
type Color =
    | Red
    | Green
    | Blue
    | RGB(Int, Int, Int)
    | Named(String)

// Verwendung
let c = Color.RGB(255, 128, 0)
match c {
    Red => "rot"
    RGB(r, _, _) if r > 200 => "rötlich"
    _ => "andere"
}
```

### Traits

```ssl
trait Printable {
    fn to_string(&self) -> String
}

trait Comparable<T> {
    fn compare(&self, other: &T) -> Ordering
}

impl Printable for Point {
    fn to_string(&self) -> String {
        "(${self.x}, ${self.y})"
    }
}
```

---

## 2. Standardbibliothek (stdlib)

### io

```ssl
import stdlib.io

io::println("Hello!")
io::print("No newline")
io::eprintln("Error!")

let line = io::input()
let formatted = io::format("x = {}, y = {}", x, y)
```

### string

```ssl
import stdlib.string

let s = "Hello, World!"
s.length()              // 13
s.to_upper()            // "HELLO, WORLD!"
s.to_lower()            // "hello, world!"
s.contains("World")     // true
s.starts_with("Hello")  // true
s.split(", ")           // ["Hello", "World!"]
s.replace("World", "SSL")
s.trim()
s.substring(0, 5)       // "Hello"
```

### list

```ssl
import stdlib.list

let list = [1, 2, 3, 4, 5]

list.push(6)
list.pop()
list.get(0)             // Some(1)
list.first()            // Some(1)
list.last()             // Some(5)
list.length()           // 5
list.is_empty()         // false

// Funktionale Operationen
list.map(|x| x * 2)     // [2, 4, 6, 8, 10]
list.filter(|x| x > 2)  // [3, 4, 5]
list.reduce(0, |a, b| a + b)  // 15
list.fold(1, |a, b| a * b)    // 120
list.any(|x| x > 4)     // true
list.all(|x| x > 0)     // true
list.find(|x| x == 3)   // Some(3)
list.sort()
list.reverse()
list.unique()
list.zip(other_list)
list.flatten()
```

### map

```ssl
import stdlib.map

let map = { "a": 1, "b": 2 }

map.get("a")            // Some(1)
map.insert("c", 3)
map.remove("a")
map.contains_key("b")   // true
map.keys()              // ["a", "b"]
map.values()            // [1, 2]
map.entries()           // [("a", 1), ("b", 2)]
```

### math

```ssl
import stdlib.math

math::PI                // 3.14159...
math::E                 // 2.71828...

math::abs(-5)           // 5
math::min(3, 7)         // 3
math::max(3, 7)         // 7
math::clamp(x, 0, 100)

math::floor(3.7)        // 3.0
math::ceil(3.2)         // 4.0
math::round(3.5)        // 4.0

math::sqrt(16.0)        // 4.0
math::pow(2.0, 10.0)    // 1024.0
math::exp(1.0)          // 2.718...
math::ln(math::E)       // 1.0
math::log10(100.0)      // 2.0
math::log2(8.0)         // 3.0

math::sin(x)
math::cos(x)
math::tan(x)
math::asin(x)
math::acos(x)
math::atan(x)
math::atan2(y, x)
```

### file

```ssl
import stdlib.file

let content = file::read_string("data.txt")?
file::write_string("output.txt", content)?

let bytes = file::read_bytes("image.png")?
file::write_bytes("copy.png", bytes)?

file::exists("path")
file::is_file("path")
file::is_directory("path")
file::list_dir("dir")
file::create_dir("new_dir")
file::remove("file")
file::copy("src", "dst")
file::rename("old", "new")
```

---

## 3. Erweiterte Features

### Algebraic Effects

```ssl
import features.effects

effect Logger {
    fn log(level: Level, message: String) -> Unit
}

effect State<S> {
    fn get() -> S
    fn set(value: S) -> Unit
    fn modify(f: fn(S) -> S) -> Unit
}

// Handler
handle {
    log(Info, "Starting...")
    let x = get()
    set(x + 1)
} with Logger {
    log(level, msg) => {
        println("[${level}] ${msg}")
        resume(())
    }
} with State {
    get() => resume(current_state)
    set(v) => { current_state = v; resume(()) }
    modify(f) => { current_state = f(current_state); resume(()) }
}
```

### Linear Types

```ssl
import features.linear

linear struct Connection {
    host: String
    port: Int
    socket: Socket
}

impl Connection {
    fn open(host: String, port: Int) -> Connection { ... }
    fn send(&self, data: Bytes) -> Result<(), Error> { ... }
    fn receive(&self) -> Result<Bytes, Error> { ... }
    fn close(self) { ... }  // Konsumiert self
}

// Verwendung - Compiler stellt sicher, dass close() aufgerufen wird
let conn = Connection.open("localhost", 8080)
conn.send(data)?
let response = conn.receive()?
conn.close()  // Pflicht!
```

### Pattern Matching

```ssl
import features.patterns

// Alle Pattern-Arten
match value {
    // Literale
    0 => "zero"
    1 | 2 | 3 => "small"
    
    // Ranges
    4..=10 => "medium"
    
    // Structs
    Point { x: 0, y } => "on y-axis at ${y}"
    Point { x, y: 0 } => "on x-axis at ${x}"
    
    // Enums
    Option.Some(inner) => "has ${inner}"
    Option.None => "empty"
    
    // Guards
    n if n < 0 => "negative"
    n if n % 2 == 0 => "even"
    
    // Binding
    n @ 100..=999 => "three digits: ${n}"
    
    // Wildcard
    _ => "other"
}

// Array patterns
match list {
    [] => "empty"
    [x] => "single: ${x}"
    [first, second] => "pair"
    [head, ..tail] => "head: ${head}, rest: ${tail}"
}
```

### Reactive Streams

```ssl
import features.reactive

let source = Observable.interval(1000)  // Jede Sekunde

source
    .map(|i| i * 2)
    .filter(|x| x % 4 == 0)
    .take(10)
    .debounce(500)
    .distinct_until_changed()
    .subscribe(
        on_next: |x| println("Value: ${x}"),
        on_error: |e| println("Error: ${e}"),
        on_complete: || println("Done")
    )

// Subjects
let subject = Subject.new()
subject.next(1)
subject.next(2)
subject.complete()

// BehaviorSubject (mit initialem Wert)
let behavior = BehaviorSubject.new(0)
behavior.subscribe(|x| println(x))  // Sofort: 0
behavior.next(1)  // 1
```

### Property-Based Testing

```ssl
import features.property_test

@property(iterations: 1000)
fn test_reverse_twice_identity() {
    for_all(gen_list(gen_int()), |list| {
        list.reverse().reverse() == list
    })
}

@property(iterations: 500)
fn test_sort_length_preserved() {
    for_all(gen_list(gen_string()), |list| {
        list.sort().length() == list.length()
    })
}

@property(iterations: 100)
fn test_map_length() {
    for_all(gen_list(gen_int()), |list| {
        for_all(gen_fn(gen_int()), |f| {
            list.map(f).length() == list.length()
        })
    })
}
```

### Async/Await

```ssl
import features.async

async fn fetch_user(id: Int) -> Result<User, Error> {
    let response = http::get("/users/${id}").await?
    json::parse(response.body)
}

async fn main() {
    // Sequential
    let user1 = fetch_user(1).await?
    let user2 = fetch_user(2).await?
    
    // Parallel
    let (u1, u2) = join(
        fetch_user(1),
        fetch_user(2)
    ).await
    
    // Race
    let first = race(
        fetch_user(1),
        fetch_user(2)
    ).await
    
    // Channels
    let (tx, rx) = channel()
    spawn(|| tx.send(42))
    let value = rx.receive().await
}
```

### GPU/SIMD

```ssl
import features.gpu

// SIMD Vektoren
let v1 = Vec4.new(1.0, 2.0, 3.0, 4.0)
let v2 = Vec4.new(5.0, 6.0, 7.0, 8.0)

let sum = v1.add(v2)
let dot = v1.dot(v2)
let cross = v1.cross(v2)

// Matrizen
let m = Mat4.identity()
let rotated = m.rotate(45.0, Vec3.up())
let scaled = m.scale(Vec3.new(2.0, 2.0, 2.0))

// Parallele Operationen
let data = (0..1_000_000).collect()
let squares = parallel_map(data, |x| x * x)
let sum = parallel_reduce(data, 0, |a, b| a + b)
```

---

## 4. GUI Framework

### Fenster-Formen

```ssl
import gui

// Standard
let rect = Window.new("App", 800.0, 600.0)

// Kreisförmig
let circle = Window.circle("Circle", 200.0)

// Elliptisch
let ellipse = Window.ellipse("Ellipse", 200.0, 150.0)

// Polygone
let triangle = Window.triangle("Triangle", 200.0)
let pentagon = Window.pentagon("Pentagon", 150.0)
let hexagon = Window.hexagon("Hexagon", 150.0)
let octagon = Window.octagon("Octagon", 150.0)

// Spezialformen
let egg = Window.egg("Egg", 100.0, 150.0, 0.3)
let heart = heart_window("Heart", 100.0)
let star = star_window("Star", 100.0, 50.0, 5)

// Freiform
let custom = Window.custom("Custom", |builder| {
    builder
        .move_to(Point.new(0.0, 50.0))
        .line_to(Point.new(50.0, 0.0))
        .curve_to(
            Point.new(75.0, 25.0),
            Point.new(75.0, 75.0),
            Point.new(50.0, 100.0)
        )
        .close()
})
```

### Widgets

```ssl
import gui.widgets

let label = Label.new("Hello!")
    .font_size(16.0)
    .color(Color.white())

let button = Button.new("Click Me")
    .on_click(|| println("Clicked!"))
    .style(Primary)

let input = TextInput.new()
    .placeholder("Enter text...")
    .on_change(|text| println("Changed: ${text}"))

let slider = Slider.new(0.0, 100.0)
    .value(50.0)
    .on_change(|v| println("Value: ${v}"))
```

---

## 5. Entwicklungstools

### CLI

```bash
ssl run file.ssl         # Ausführen
ssl build file.ssl       # Kompilieren
ssl check file.ssl       # Typenprüfung
ssl fmt file.ssl         # Formatieren
ssl test tests/          # Tests
ssl repl                 # Interaktiv
ssl doc file.ssl         # Dokumentation
ssl bootstrap            # Self-Compilation
```

### LSP

```ssl
import lsp

let server = LspServer.new()
server.initialize(root_uri)

// Features:
// - textDocument/completion
// - textDocument/hover
// - textDocument/definition
// - textDocument/references
// - textDocument/formatting
// - textDocument/diagnostics
```

### Hot Reload

```ssl
import hotreload

dev("src/", "main.ssl")

// Alternativ: programmatisch
var engine = HotReloadEngine.new("src/")
engine.on_reload(|path| println("Reloaded: ${path}"))
engine.on_error(|path, err| println("Error: ${err}"))
engine.start()
```

### Debugger

```ssl
import debugger

var dbg = TimeTravelDebugger.new()

dbg.add_breakpoint_line("main.ssl", 42)
dbg.watch("counter")

dbg.run(chunk)

dbg.step_back()
dbg.goto_step(100)
dbg.replay(50, 75, 100)
```

---

## 6. Quantum Computing

```ssl
import quantum

// Einzelnes Qubit
let q = Qubit.zero()
let superposition = hadamard().apply(q)
let result = q.measure()  // true oder false

// Quantenregister
var reg = QuantumRegister.new(3)
reg.hadamard(0)
reg.cnot(0, 1)
reg.cnot(0, 2)  // GHZ-State

// Algorithmen
let found = grover_search(items, target)
let (p, q) = shor_factor(15)  // (3, 5)
let is_constant = deutsch_jozsa(n, oracle)
```

---

## 7. Distributed Computing

```ssl
import distributed

// Actor System
var system = ActorSystem.create("my-system")

let actor = system.spawn("counter", CounterActor { count: 0 })
actor.send(Increment)
let count = actor.ask(GetCount).await

// Cluster
var cluster = Cluster.create("192.168.1.100", 2551, ClusterConfig.default())
cluster.join()
cluster.broadcast(message)
cluster.leave()
```

---

## 8. AI/ML

```ssl
import ai

// Tensoren
let t = Tensor.randn([3, 3])
let result = t.matmul(&other).relu().softmax()

// Neuronales Netz
var model = Sequential.new()
model.add(Linear.new(784, 256))
model.add(ReLU.new())
model.add(Linear.new(256, 10))

let optimizer = Adam.new(0.001)
let loss_fn = CrossEntropyLoss.new()

// Training
for epoch in 0..100 {
    let loss = train_epoch(&mut model, &loader, &loss_fn, &mut optimizer)
}
```

---

**SSL v5.0 - Self-Hosting Edition**

*Alle Features in 20.376 Zeilen SSL-Code*
