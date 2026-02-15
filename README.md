# gggp

Minimal Rust Library + Python SDK foundation for GGGP.

This repository currently provides a small Python extension module (built from Rust via `pyo3` + `maturin`) to establish release and CI/CD workflows.

## Install (local)

```bash
python -m pip install -U pip maturin
maturin build --release -o dist
python -m pip install dist/*.whl
```

## Python API

```python
import gggp

print(gggp.version())
child_a, child_b = gggp.one_point_crossover([1, 2, 3], [10, 20, 30], split=1)
mutated = gggp.mutate_gene([1, 2, 3], index=2, value=99)
```

## Release

1. Configure PyPI Trusted Publisher for this repository and workflow file:
   - Repository: `olkhovoy/gggp`
   - Workflow: `.github/workflows/publish.yml`
2. Create and push a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The `publish` workflow will build wheel + sdist and upload to PyPI.

