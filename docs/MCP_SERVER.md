# MCP Server Usage Guide

The dart-re-analyzer includes a built-in Model Context Protocol (MCP) server for programmatic access to analysis results.

## Starting the Server

```bash
dart-re-analyzer serve --port 9000 /path/to/project
```

Options:
- `--port, -p`: Port to listen on (default: 9000)
- `--config`: Path to configuration file
- Path argument: Project directory to analyze

## API Methods

The MCP server uses a simple JSON-RPC protocol over TCP. Send JSON requests with a method and params field, one per line.

### 1. Get All Errors

Request:
```json
{"method": "get_all_errors", "params": {}}
```

Response:
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

### 2. Get Filtered Errors

Request:
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

All filter fields are optional:
- `category`: "style" or "runtime"
- `severity`: "error", "warning", or "info"
- `file`: Partial file path match

Response: Same as get_all_errors but filtered

### 3. Get Statistics

Request:
```json
{"method": "get_stats", "params": {}}
```

Response:
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

## Error Handling

If a request fails, the response will have `success: false` and an `error` field:

```json
{
  "success": false,
  "data": null,
  "error": "Unknown method: invalid_method"
}
```

## Integration Tips

1. **Continuous Monitoring**: Keep the server running and query it periodically
2. **CI/CD Integration**: Start server, run analysis, query results, shut down
3. **Editor Integration**: Use MCP server to provide real-time linting feedback
4. **Dashboard**: Build a web dashboard that queries the MCP server for metrics

## Performance

- The server analyzes the project once on startup
- Queries are served from in-memory cache
- For fresh analysis, restart the server
- Future versions may support watch mode for automatic re-analysis
