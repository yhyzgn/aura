# Aura Logo Candidates

These files are alternate main-library Aura brand marks. They intentionally do
not replace the current `packaging/icons/aura.*` assets yet.

Candidate set:

- `aura-candidate-orbit.*` — abstract orbital core, no letterform.
- `aura-candidate-prism.*` — 3D component/system prism.
- `aura-candidate-ribbon.*` — flowing aurora ribbon mark.
- `aura-candidate-monolith.*` — document/component slab with aura arc.

Each candidate includes:

- `.svg` editable source.
- `.png` 1024x1024 preview/package source.
- `.ico` multi-size Windows icon.
- `.icns` multi-size macOS icon.

Once a candidate is selected, copy its SVG/PNG/ICO/ICNS files over
`packaging/icons/aura.*` and rerun `cargo xtask package validate`.
