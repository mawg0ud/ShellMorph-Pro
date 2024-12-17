# ShellMorph Pro API Documentation

The ShellMorph Pro API provides RESTful endpoints to interact with the tool programmatically. This document outlines available endpoints, usage examples, and expected responses.

## Base URL
```
http://localhost:8080/api/v1
```

---

## Authentication
All requests require an API key in the header:
```http
Authorization: Bearer <API_KEY>
```

---

## Endpoints

### 1. **Generate Shellcode**
- **POST** `/generate`
- **Description:** Converts a PE file into shellcode.
- **Request Body:**
  ```json
  {
    "file_path": "path/to/pe_file.exe",
    "architecture": "x64",
    "options": {
      "obfuscation": true,
      "anti_debugging": true
    }
  }
  ```
- **Response:**
  ```json
  {
    "status": "success",
    "shellcode": "<base64_encoded_shellcode>"
  }
  ```

### 2. **Inject Shellcode**
- **POST** `/inject`
- **Description:** Injects shellcode into a target process.
- **Request Body:**
  ```json
  {
    "pid": 1234,
    "shellcode": "<base64_encoded_shellcode>",
    "injection_method": "APC"
  }
  ```
- **Response:**
  ```json
  {
    "status": "success",
    "message": "Shellcode injected successfully"
  }
  ```

### 3. **Get Injection Techniques**
- **GET** `/methods`
- **Description:** Lists all supported injection techniques.
- **Response:**
  ```json
  {
    "methods": [
      "ThreadHijacking",
      "APC",
      "ProcessHollowing"
    ]
  }
  ```

---

## Error Handling
All errors are returned in the following format:
```json
{
  "status": "error",
  "message": "Description of the error"
}
```

---

## Example Usage (Python)
```python
import requests

url = "http://localhost:8080/api/v1/generate"
headers = {
    "Authorization": "Bearer <API_KEY>"
}
data = {
    "file_path": "test.exe",
    "architecture": "x64",
    "options": {
        "obfuscation": True
    }
}
response = requests.post(url, json=data, headers=headers)
print(response.json())
```
