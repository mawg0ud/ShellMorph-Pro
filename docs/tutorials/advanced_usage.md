
# Advanced Usage of ShellMorph Pro

This guide covers advanced features and configurations to leverage the full potential of ShellMorph Pro.

---

## Advanced Shellcode Options
ShellMorph Pro allows for granular control over shellcode generation and injection.

### Obfuscation
Enable obfuscation to evade detection:
```json
{
  "options": {
    "obfuscation": true
  }
}
```

### Anti-Debugging
Prevent execution in debugging environments:
```json
{
  "options": {
    "anti_debugging": true
  }
}
```

---

## Using the CLI
The CLI provides a quick way to interact with ShellMorph Pro.

### Generate Shellcode
```bash
./shellmorph-cli generate --file test.exe --arch x64 --obfuscation
```

### Inject Shellcode
```bash
./shellmorph-cli inject --pid 1234 --method APC
```

---

## Developing Custom Injection Techniques
ShellMorph Pro supports pluggable modules for custom methods:
1. Implement the `InjectionModule` interface in Rust.
2. Add your module to the `src/core/injection/` directory.
3. Register it in `injection_registry.rs`.

---

For further assistance, refer to the [API Documentation](../API.md).
