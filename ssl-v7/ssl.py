#!/usr/bin/env python3
"""
SSL v7.0 - Sonner Studio Language
Native Compilation Edition
"""

import sys
import os
import subprocess
from pathlib import Path

VERSION = "7.0.0"
EDITION = "Native Compilation Edition"

def get_ssl_dir():
    """Get SSL installation directory"""
    if hasattr(sys, '_MEIPASS'):
        # PyInstaller bundle
        return Path(sys._MEIPASS)
    else:
        # Development/installed
        return Path(__file__).parent

def main():
    ssl_dir = get_ssl_dir()
    compiler = ssl_dir / "ssl-v7" / "src" / "main.ssl"
    
    if len(sys.argv) == 1:
        print(f"SSL v{VERSION} - {EDITION}")
        print("\nUsage:")
        print("  ssl compile <file.ssl>    - Compile SSL source to executable")
        print("  ssl run <file.ssl>        - Run SSL program")
        print("  ssl check <file.ssl>      - Check syntax")
        print("  ssl version               - Show version")
        print("  ssl help                  - Show this help")
        return 0
    
    command = sys.argv[1]
    
    if command == "version" or command == "--version":
        print(f"SSL v{VERSION} - {EDITION}")
        print("Copyright © 2025 SonnerStudio")
        print("License: Apache 2.0")
        return 0
    
    elif command == "help" or command == "--help":
        print(f"SSL v{VERSION} - {EDITION}\n")
        print("Commands:")
        print("  compile <file>   Compile SSL to native executable")
        print("  run <file>       Compile and run")
        print("  check <file>     Syntax check")
        print("  version          Show version")
        print("\nOptions:")
        print("  --arch <arch>    Target architecture (x86_64, arm64, apple_m)")
        print("  --opt <level>    Optimization (0-3)")
        print("  --output <file>  Output file name")
        return 0
    
    elif command == "compile":
        if len(sys.argv) < 3:
            print("Error: No input file specified")
            print("Usage: ssl compile <file.ssl>")
            return 1
        
        input_file = sys.argv[2]
        if not os.path.exists(input_file):
            print(f"Error: File not found: {input_file}")
            return 1
        
        # Run SSL compiler
        print(f"Compiling {input_file}...")
        result = subprocess.run([
            "python", str(compiler), input_file
        ] + sys.argv[3:])
        
        return result.returncode
    
    elif command == "run":
        if len(sys.argv) < 3:
            print("Error: No input file specified")
            return 1
        
        input_file = sys.argv[2]
        # Compile and run
        output = Path(input_file).stem
        subprocess.run(["python", str(compiler), input_file, "--output", output])
        
        if os.path.exists(output):
            return subprocess.run([f"./{output}"]).returncode
        return 1
    
    elif command == "check":
        if len(sys.argv) < 3:
            print("Error: No input file specified")
            return 1
        
        input_file = sys.argv[2]
        print(f"Checking {input_file}...")
        result = subprocess.run([
            "python", str(compiler), input_file, "--check-only"
        ])
        return result.returncode
    
    else:
        print(f"Unknown command: {command}")
        print("Run 'ssl help' for usage information")
        return 1

if __name__ == "__main__":
    sys.exit(main())
