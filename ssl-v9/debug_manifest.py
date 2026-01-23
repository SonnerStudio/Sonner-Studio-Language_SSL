
import subprocess
import os

makeappx = r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\makeappx.exe"
manifest = r"msix\AppxManifest.xml"

print(f"Validating {manifest}...")

try:
    result = subprocess.run(
        [makeappx, "validate", "/m", manifest, "/v"],
        capture_output=True,
        text=True,
        cwd=r"c:\Dev\Repos\SonnerStudio\Sonner-Studio-Language_SSL\ssl-v9"
    )
    print("STDOUT:", result.stdout)
    print("STDERR:", result.stderr)
    print("Return Code:", result.returncode)
except Exception as e:
    print(f"Error: {e}")
