# intertypes

Write your type once in a featureful dsl with first-class support for

- rich primitive types (int, float, string, bool, binary, etc.)
- algebraic data types
- finite sets and functions out of them
- generic types

then automatically derive:

- definitions for all languages
- ...which use as much of the host language type system as possible, and then get the rest of the way there with dynamic validation
- ...which have compatible serialization/deserialization
- ...which uses content-addressing to automatically de-duplicate dag-shaped data and minimize data transfer
- ...which is bundled with offline-first, but realtime-ready change management, for correct synchronization across multiple parties.

When it's time to change your mind about how to structure your data, then write 

Interested? Let's get started.

```
record Graph {
    E: fintype
    V: fintype
    src(e: E): V
    tgt(e: E): V
}
```
