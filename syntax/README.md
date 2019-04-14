# Clasp Syntax

The following is a work-in-progress guide to Clasp's syntax, showcasing the
language's features.

## Messages

Clasp's purpose is to apply logical operations to structured data in order to
prove it matches a set of properties/facts. Structured data in Clasp takes the
form of `message` directives, which are designed to be automatically serialized
and deserialized:

```
message Token {
    body(0): !byte[]
    tag(1): !byte[32]
}
```

Breaking it down:

- `Token`: name of the message type
- `body(0)`: the `body` field is serialized with tag `0` (when using TLV formats)
