# Metadata

Values in Nu pipelines have metadata (tags) attached that don't affect the data but improve the experience.

## Viewing Metadata

```nu
metadata (open Cargo.toml)
# => {span: {start: 212970, end: 212987}}

metadata (open Cargo.toml) | get span
# => {start: 212970, end: 212987}
```

The span shows where the value originated - useful for error messages.

## Custom Metadata

Attach arbitrary metadata:
```nu
"data" | metadata set --merge {custom_key: "custom_value"}
```

## HTTP Response Metadata

HTTP commands attach response metadata:
```nu
http get https://api.example.com | metadata | get http_response.status
# => 200
```
