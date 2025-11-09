---
layout: default
title: MCP Server
---

# MCP Server Guide

The dart-re-analyzer includes a built-in Model Context Protocol (MCP) server for programmatic access to analysis results.

## Table of Contents
- [Starting the Server](#starting-the-server)
- [API Methods](#api-methods)
  - [Get All Errors](#1-get-all-errors)
  - [Get Filtered Errors](#2-get-filtered-errors)
  - [Get Statistics](#3-get-statistics)
- [Example Usage](#example-usage)
- [Error Handling](#error-handling)
- [Integration Tips](#integration-tips)

---

## Starting the Server

```bash
dart-re-analyzer serve --port 9000 /path/to/project
```

### Options
- `--port, -p`: Port to listen on (default: 9000)
- `--config`: Path to configuration file
- Path argument: Project directory to analyze

---

## API Methods

The MCP server uses a simple JSON-RPC protocol over TCP. Send JSON requests with a method and params field, one per line.

### 1. Get All Errors

Retrieve all diagnostics from the analysis.

**Request:**
```json
{"method": "get_all_errors", "params": {}}
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "rule_id": "avoid_dynamic",
      "message": "Avoid using 'dynamic' type as it bypasses type safety",
      "severity": "Warning",
      "category": "Runtime",
      "location": {
        "file": "/path/to/file.dart",
        "line": 10,
        "column": 5,
        "end_line": 10,
        "end_column": 12
      },
      "suggestion": "Use a specific type or Object? instead"
    }
  ]
}
```

---

### 2. Get Filtered Errors

Retrieve diagnostics filtered by category, severity, or file.

**Request:**
```json
{
  "method": "get_errors",
  "params": {
    "category": "runtime",
    "severity": "error",
    "file": "main.dart"
  }
}
```

**Filter Parameters:**

All filter fields are optional:
- `category`: "style" or "runtime"
- `severity`: "error", "warning", or "info"
- `file`: Partial file path match

**Response:** Same as get_all_errors but filtered

---

### 3. Get Statistics

Get aggregated statistics about the analysis results.

**Request:**
```json
{"method": "get_stats", "params": {}}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "total": 25,
    "errors": 3,
    "warnings": 18,
    "info": 4,
    "style_issues": 10,
    "runtime_issues": 15,
    "files_with_issues": 8
  }
}
```

---

## Example Usage

### Python Client

```python
import socket
import json

def query_analyzer(method, params=None):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect(('127.0.0.1', 9000))
    
    request = {"method": method, "params": params or {}}
    sock.sendall(json.dumps(request).encode() + b'\n')
    
    response = sock.recv(4096)
    sock.close()
    
    return json.loads(response)

# Get all errors
result = query_analyzer("get_all_errors")
print(f"Found {len(result['data'])} issues")

# Get only runtime errors
result = query_analyzer("get_errors", {"category": "runtime", "severity": "error"})
for issue in result['data']:
    print(f"{issue['location']['file']}:{issue['location']['line']} - {issue['message']}")

# Get statistics
stats = query_analyzer("get_stats")
print(f"Total issues: {stats['data']['total']}")
print(f"Errors: {stats['data']['errors']}")
```

### Node.js Client

```javascript
const net = require('net');

function queryAnalyzer(method, params = {}) {
  return new Promise((resolve, reject) => {
    const client = net.createConnection({ port: 9000, host: '127.0.0.1' }, () => {
      const request = JSON.stringify({ method, params }) + '\n';
      client.write(request);
    });

    client.on('data', (data) => {
      resolve(JSON.parse(data.toString()));
      client.end();
    });

    client.on('error', reject);
  });
}

// Usage
(async () => {
  const result = await queryAnalyzer('get_all_errors');
  console.log(`Found ${result.data.length} issues`);

  const stats = await queryAnalyzer('get_stats');
  console.log('Statistics:', stats.data);
})();
```

### curl Example

Using netcat (nc):
```bash
echo '{"method": "get_stats", "params": {}}' | nc 127.0.0.1 9000
```

---

## Error Handling

If a request fails, the response will have `success: false` and an `error` field:

```json
{
  "success": false,
  "data": null,
  "error": "Unknown method: invalid_method"
}
```

---

## Integration Tips

### 1. Continuous Monitoring
Keep the server running and query it periodically to monitor code quality over time.

### 2. CI/CD Integration
Start server, run analysis, query results, and shut down as part of your build pipeline.

**Example GitHub Actions:**
```yaml
- name: Start MCP Server
  run: dart-re-analyzer serve --port 9000 . &
  
- name: Query Results
  run: |
    sleep 2  # Wait for server to start
    echo '{"method": "get_stats", "params": {}}' | nc localhost 9000
```

### 3. Editor Integration
Use MCP server to provide real-time linting feedback in custom editor plugins.

### 4. Dashboard
Build a web dashboard that queries the MCP server for real-time metrics and visualizations.

---

## Performance

The MCP server is designed for efficiency:

- ✅ Analyzes the project once on startup
- ✅ Queries are served from in-memory cache
- ✅ Minimal overhead for query operations
- ⚡ For fresh analysis, restart the server
- 🔜 Future versions may support watch mode for automatic re-analysis

**Performance Characteristics:**

For large projects (1000+ files):
- Initial analysis: 1-3 seconds
- Query response: < 1ms (from cache)
- Memory usage: ~50-100MB

---

## Use Cases

### Quality Metrics Dashboard
Create a real-time dashboard showing:
- Total issues by category
- Trend over time
- Most problematic files
- Rule violation distribution

### Pre-commit Hooks
Query the MCP server before allowing commits:
```bash
#!/bin/bash
stats=$(echo '{"method": "get_stats", "params": {}}' | nc localhost 9000)
errors=$(echo $stats | jq '.data.errors')
if [ "$errors" -gt 0 ]; then
  echo "Fix errors before committing"
  exit 1
fi
```

### Team Reports
Generate weekly reports on code quality improvements or regressions.

---

[← Back to Home](index) | [View All Rules →](rules)
