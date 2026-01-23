# Sonner Studio Language (SSL) - Beispiele

## 1. Hello World

```ssl
fn main() {
    print("Hello, SSL!")
}
```

## 2. Fibonacci (Rekursion)

```ssl
fn fib(n: Int) -> Int {
    if n <= 1 { return n }
    return fib(n-1) + fib(n-2)
}

fn main() {
    print(fib(10))  // 55
}
```

## 3. Parallel Computing

```ssl
fn main() {
    let chan = channel()
    let tx = chan[0]
    let rx = chan[1]
    
    // Producer Thread
    spawn {
        for i in 0..5 {
            send(tx, i)
        }
    }
    
    // Consumer Thread
    for i in 0..5 {
        let value = recv(rx)
        print(value)
    }
}
```

## 4. Quantum Computing

```ssl
fn main() {
    // Bell State Erzeugung (simuliert)
    let q1 = Qubit()
    let q2 = Qubit()
    
    H(q1)  // Superposition
    // CNOT(q1, q2)  // Verschränkung (TODO)
    
    let result = Measure(q1)
    print("Measurement result:", result)
}
```

## 5. Self-Healing Code

```ssl
fn main() {
    try {
        let result = 10 / 0
        print(result)
    } recover (err) {
        print("Error caught:", err)
        print("Using fallback value")
    }
}
```

## 6. Kombiniertes Beispiel

```ssl
fn parallel_quantum() {
    let results = channel()
    
    // Spawn 10 parallel quantum experiments
    for i in 0..10 {
        spawn {
            let q = Qubit()
            H(q)
            let measurement = Measure(q)
            send(results[0], measurement)
        }
    }
    
    // Collect results
    let sum = 0
    for i in 0..10 {
        sum = sum + recv(results[1])
    }
    
    print("Average measurement:", sum / 10)
}

fn main() {
    try {
        parallel_quantum()
    } recover (err) {
        print("Experiment failed:", err)
    }
}
```
