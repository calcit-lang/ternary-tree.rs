# Benchmark Comparison Report (Template)

## 1) Scope

- Branch/Commit A (baseline):
- Branch/Commit B (candidate):
- Date:
- Machine / CPU:
- Rust toolchain (`rustc -V`):
- Command shell / OS:

---

## 2) Baseline Setup

Run once on the **candidate** version to save baseline:

```bash
cargo bench --bench iterator  -- --noplot --save-baseline optimized_v1
cargo bench --bench accessing -- --noplot --save-baseline optimized_v1
cargo bench --bench searching -- --noplot --save-baseline optimized_v1
```

Run comparison on another commit/branch:

```bash
cargo bench --bench iterator  -- --noplot --baseline optimized_v1
cargo bench --bench accessing -- --noplot --baseline optimized_v1
cargo bench --bench searching -- --noplot --baseline optimized_v1
```

---

## 3) Pass/Fail Criteria

- Critical paths (`iter`, `index`, `get`, `loop_get`, `ref_get`):
  - PASS if regression <= 5%
  - WARN if 5% < regression <= 10%
  - FAIL if regression > 10%
- Search paths (`index_of_*`, `last_index_of_*`, `find_index_*`):
  - PASS if regression <= 8%
  - WARN if 8% < regression <= 15%
  - FAIL if regression > 15%
- Any improvement is always PASS.

---

## 4) Result Table

| Benchmark | Baseline time | Candidate time | Change | Status | Notes |
|---|---:|---:|---:|---|---|
| iter |  |  |  |  |  |
| traverse |  |  |  |  |  |
| index |  |  |  |  |  |
| get |  |  |  |  |  |
| loop_get |  |  |  |  |  |
| ref_get |  |  |  |  |  |
| first |  |  |  |  |  |
| index_of_hit/tree |  |  |  |  |  |
| index_of_hit/vec |  |  |  |  |  |
| last_index_of_hit/tree |  |  |  |  |  |
| last_index_of_hit/vec |  |  |  |  |  |
| index_of_miss/tree |  |  |  |  |  |
| last_index_of_miss/tree |  |  |  |  |  |
| find_index_hit/tree |  |  |  |  |  |
| find_index_miss/tree |  |  |  |  |  |

---

## 5) Key Findings (Summary)

- Main win:
- Main regression:
- Expected trade-off confirmed:
- Unexpected result:

---

## 6) Decision

- [ ] Approve merge
- [ ] Needs follow-up optimization
- [ ] Block merge

Reason:

---

## 7) Follow-up Actions

- Action 1:
- Action 2:
- Action 3:

---

## 8) Raw Logs (Optional)

Paste selected Criterion outputs here for auditability.
