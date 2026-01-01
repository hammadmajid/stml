# STML

STML (Strict Text Markup Language) is a strict and minimal markup language. This is not intended for production use and only exists as a fun project.

## Core goals

* No raw text. All text is inside string literals.
* References are explicit, typed, and validated.
* Missing or failing resources are handled by syntax, not logic.
* Minimal symbols. No free-form flexibility.

---

## Document structure

A document is a list of blocks. Nothing exists outside a block.

```
doc "title" {
  ...
}
```

Anything outside `doc {}` is invalid.

---

### Strings

All human text must be a string literal.

```
"text"
```

Multiline strings are explicit.

```
"""
line one
line two
"""
```

No implicit text. No bare words.

---

### Sections

Sections must be named and have IDs. IDs are global within the document.

```
section #intro {
  p "This is the intro."
}
```

If an ID is reused, the document is invalid.

---

### Paragraphs

Paragraphs are blocks, not raw text.

```
p "Single line paragraph."

p """
Multi
line
paragraph.
"""
```

No paragraph may contain raw text. Only strings or inline elements.

---

### Inline references (strict)

References must declare what they point to.

```
ref section:#intro "see intro"
```

If the referenced section does not exist, the document is invalid.

---

### Cross-file references

Files must be declared before use.

```
use "./other.stml" as other
```

Reference syntax:

```
ref file:other.section:#usage "usage section"
```

If the file or section is missing, invalid.

---

### Lists

Lists require explicit items.

```
ol {
  i "first"
  i "second"
}

ul {
  item "first"
  item "second"
}
```

For `olist` the renderer should automaically increment.

---

### Images

Images with enforced failure handling. Images must declare a fallback. No fallback means invalid syntax.

```
image {
  src "https://site/img.png"
  alt "diagram"
  fallback {
    p "Image failed to load."
  }
}
```

The renderer decides when fallback is used. The author must provide it.

---

### Links

```
link {
  to "https://example.com"
  text "example"
}
```

---

### Null handling (non-optional blocks)

Some blocks may explicitly allow absence. This is done with `maybe`.

```
maybe image {
  src "https://site/img.png"
  alt "optional image"
  fallback {
    p "No image."
  }
}
```

If `maybe` is not used, the renderer must treat failure as fatal.

---

### Metadata

```
meta {
  author "name"
  date "2026-01-02"
}
```

The renderer may do whatever with the metadata.

---

### Not allowed

* Raw text
* Implicit paragraphs
* Implicit references
* Missing fallbacks
* Silent failure
* Auto-generated IDs

---