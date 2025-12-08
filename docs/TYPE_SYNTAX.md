# SSL v3.1 Type Syntax Reference

## Generic Lists

SSL verwendet **eckige Klammern** für generische Listen, nicht spitze Klammern:

### ✅ Korrekt:
```ssl
let numbers: List[Int] = [1, 2, 3]
let names: List[String] = ["Alice", "Bob"]

fn process[T](items: List[T]) -> List[T] {
    return items
}
```

### ❌ NICHT unterstützt:
```ssl
let numbers: List<Int> = [1, 2, 3]  // Syntax-Fehler!
```

## Why Square Brackets?

SSL nutzt `<` und `>` für:
- Vergleiche: `if x < 10`
- Funktionskomposition: `f >> g`
- Generics in Funktionsdefinitionen: `fn foo<T>(x: T)`

Aber für **Type Annotations** wird `[`/`]` verwendet, um Mehrdeutigkeit zu vermeiden.

## Andere Generics

```ssl
enum Result[T, E] {
    Ok(T),
    Err(E)
}

type UserId = Int
type UserMap = Map[String, User]
```
