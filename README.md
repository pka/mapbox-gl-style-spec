# Mapbox GL TOML Style Spec

This is an automatic translation of the [Mapbox GL style spec](https://github.com/mapbox/mapbox-gl-style-spec) to [TOML](https://github.com/toml-lang/toml).

### Setup

Install Node modules:
```bash
npm install
```

Build JSON to TOML converter:
```bash
cd glstyleconv
cargo build
```

### Build documentation

* The style reference page exists here: `docs/_generate/index.html` and can be edited directly.

To build and view the documentation, run

```bash
npm start
```

and open the served page

```bash
open http://127.0.0.1:4000/mapbox-gl-style-spec
```

### Merge updates from JSON spec

```bash
git checkout mb-pages
git pull
git checkout toml
git merge mb-pages
```

### Publish documentation

```bash
git checkout gh-pages
cp _site/* .
git commit -a
git push
git checkout toml
```
