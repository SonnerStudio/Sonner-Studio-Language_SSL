#!/usr/bin/env python3
"""
SSL v7 + Ollama Integration Test
Demonstriert Code-Generierung und -Validierung mit Ollama
"""

import requests
import json

OLLAMA_API = "http://localhost:11434/api/generate"

def ollama_generate(prompt, model="codellama", temperature=0.7):
    """Generiere Code/Text mit Ollama"""
    try:
        response = requests.post(OLLAMA_API, json={
            "model": model,
            "prompt": prompt,
            "stream": False,
            "options": {
                "temperature": temperature
            }
        })
        return response.json()['response']
    except Exception as e:
        return f"Error: {e}"

def test_1_code_generation():
    """Test 1: SSL-Code generieren"""
    print("=" * 60)
    print("TEST 1: SSL v7 Code-Generierung")
    print("=" * 60)
    
    prompt = """
Schreibe eine SSL v7.0 Funktion für Fibonacci mit:
- Typ-Signatur: fn fibonacci(n: Int) -> Int
- Rekursive Implementierung
- Base cases für n <= 1
Nur Code, keine Erklärungen.
"""
    
    code = ollama_generate(prompt, model="codellama", temperature=0.2)
    print(code)
    return code

def test_2_code_validation(code):
    """Test 2: Code validieren"""
    print("\n" + "=" * 60)
    print("TEST 2: Code-Validierung")
    print("=" * 60)
    
    prompt = f"""
Analysiere folgenden SSL-Code auf Fehler und gib Feedback:

{code}

Prüfe: Syntax, Typen, Performance, Best Practices.
Gib strukturiertes Feedback auf Deutsch.
"""
    
    feedback = ollama_generate(prompt, model="mistral", temperature=0.3)
    print(feedback)
    return feedback

def test_3_nlp_code():
    """Test 3: NLP-Code in Deutsch"""
    print("\n" + "=" * 60)
    print("TEST 3: NLP-Code-Generierung (Deutsch)")
    print("=" * 60)
    
    prompt = """
Schreibe SSL v7.0 Code auf DEUTSCH (NLP-Modus) für:
Berechne Summe von 1 bis n

Nutze deutsche Keywords:
- funktion (statt fn)
- wenn (statt if)
- solange (statt while)
- rückgabe (statt return)
- Ganzzahl (statt Int)

Nur Code mit #!lang de Header.
"""
    
    code = ollama_generate(prompt, model="codellama", temperature=0.2)
    print(code)
    return code

def test_4_window_code():
    """Test 4: Herzförmiges Fenster"""
    print("\n" + "=" * 60)
    print("TEST 4: Herz-Fenster Code-Generierung")
    print("=" * 60)
    
    prompt = """
Generiere SSL v7.0 Code für herzförmiges Fenster:

import Window from "gui"

fn create_heart_app() -> Int {
    // Herz-Fenster erstellen mit:
    // - width: 300, height: 320
    // - color: 0xff1744 (rot)
    // - glow: true
    // Label: "Made with ❤️"
    // window.show()
    // return 0
}

Nur vollständigen Code.
"""
    
    code = ollama_generate(prompt, model="codellama", temperature=0.2)
    print(code)
    return code

def test_5_error_detection():
    """Test 5: Fehler erkennen"""
    print("\n" + "=" * 60)
    print("TEST 5: Fehler-Erkennung")
    print("=" * 60)
    
    buggy_code = """
fn fibonacci(n: String) -> Int {
    if n <= 1 {
        return "eins"
    }
    return fibonacci(n-1) + fibonacci(n-2)
}
"""
    
    prompt = f"""
Finde ALLE Fehler in diesem SSL-Code:

{buggy_code}

Liste jeden Fehler mit:
- ❌ Beschreibung
- 💡 Korrektur

Auf Deutsch.
"""
    
    errors = ollama_generate(prompt, model="mistral", temperature=0.3)
    print(errors)
    return errors

def main():
    print("\n🚀 SSL v7 + Ollama Integration Test\n")
    print("Ollama muss laufen auf http://localhost:11434")
    print("Benötigte Modelle: codellama, mistral")
    print("\n")
    
    # Test 1: Code generieren
    code = test_1_code_generation()
    
    # Test 2: Code validieren
    test_2_code_validation(code)
    
    # Test 3: NLP-Code (Deutsch)
    test_3_nlp_code()
    
    # Test 4: Window-Code
    test_4_window_code()
    
    # Test 5: Fehler erkennen
    test_5_error_detection()
    
    print("\n" + "=" * 60)
    print("✅ Alle Tests abgeschlossen!")
    print("=" * 60)

if __name__ == "__main__":
    main()
