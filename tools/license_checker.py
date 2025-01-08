import subprocess
import json

def check_licenses():
    """Checks for any incompatible licenses in the project dependencies."""
    try:
        # Run license checker tool
        result = subprocess.run(
            ["license-checker", "--json"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )

        if result.returncode != 0:
            print("License checker failed:", result.stderr)
            return

        dependencies = json.loads(result.stdout)
        for package, details in dependencies.items():
            license_type = details.get("licenses", "Unknown")
            if license_type not in ["MIT", "Apache-2.0", "BSD-3-Clause"]:
                print(f"Warning: {package} has incompatible license: {license_type}")
            else:
                print(f"{package}: {license_type}")

    except Exception as e:
        print(f"Error running license checker: {e}")

if __name__ == "__main__":
    check_licenses()
