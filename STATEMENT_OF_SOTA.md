# STATEMENT OF SOTA (2026)

This document is the **Statement of SOTA** for this template.

It exists to:
- **Prove** (in plain language) why these choices are robust, secure, and maintainable.
- Explain the **principles** and **tradeoffs** that shape the template.
- Provide a shared reference for audits and reviews (especially where **PHI/HIPAA** is involved).

---

## 1) What “SOTA 2026” means for this template

SOTA here is not “trendy.” It is:

- **Reliable defaults** over cleverness.
- **Security-by-construction** over “remember to do the right thing.”
- **Clear boundaries** over architecture astronautics.
- **Portable development** across Windows/macOS, while staying **mobile-first**.

### Non-negotiable goals

- **Primary platforms**: Android + iOS.
- **Secondary platforms**: Windows/macOS via **Expo Web** (browser runtime).
- **Sensitive data**: eyeglass prescriptions are treated as **PHI**.
- **Template quality**: predictable, repeatable, boringly reliable.

---

## 2) High-level architecture

This is a monorepo designed for **one product** with two independently runnable parts:

- `mobile/` : Expo (React Native + TypeScript)
- `backend/`: Rust (Axum + Diesel + Postgres)

Root-level `package.json` is intentionally an **orchestrator** so developers can run common tasks without installing conflicting dependencies.

### System diagram

```
                ┌───────────────────────────────────────┐
                │              Developers               │
                │     (Windows / macOS / Linux)         │
                └───────────────────┬───────────────────┘
                                    │
                                    ▼
                      ┌─────────────────────────┐
                      │      Repo Root           │
                      │  package.json (runner)   │
                      └───────────┬─────────────┘
                                  │
                 ┌────────────────┴─────────────────┐
                 │                                  │
                 ▼                                  ▼
     ┌─────────────────────────┐        ┌─────────────────────────┐
     │        mobile/           │        │        backend/          │
     │ Expo + React Native + TS │        │ Rust + Axum + Diesel     │
     │ Android/iOS first        │        │ Postgres optional/req    │
     └───────────┬─────────────┘        └───────────┬─────────────┘
                 │ HTTP/JSON                          │ SQL
                 ▼                                    ▼
     ┌─────────────────────────┐        ┌─────────────────────────┐
     │  API Client + ReactQuery │        │       PostgreSQL         │
     │  Token storage adapter   │        │   (Encrypted at rest*)   │
     └─────────────────────────┘        └─────────────────────────┘

     * Typically provided by the hosting platform (managed Postgres / disk encryption).
```

---

## 3) Why Expo (React Native) for 2026 mobile-first templates

### Why Expo

- **Fast onboarding**: fewer native build edge-cases.
- **Cross-platform dev ergonomics**: consistent tooling across Windows/macOS.
- **One codebase**: Android + iOS first, and a web runtime for desktop.
- **Mature ecosystem**: common device capabilities have stable packages.

### Desktop requirement without needless complexity

We intentionally support Windows/macOS via:

- **Expo Web** (`npm run web`) → runs as a web app in the browser.

We do **not** default to packaging native desktop apps (Electron/Tauri/RN-Windows/RN-macOS) because:

- It increases CI matrix size and native edge-cases.
- It increases long-term maintenance burden.
- Most “desktop support” requirements are satisfied by a web runtime.

This is a deliberate SOTA tradeoff:

- **Default path**: web runtime (robust, low complexity)
- **Upgrade path**: add packaging later only when it pays rent

---

## 4) Why TypeScript end-to-end

TypeScript is chosen because:

- **Refactor safety** at scale.
- **API contract clarity** between app layers.
- **Tooling & ergonomics** in 2026 are excellent (autocomplete, static analysis).

For a template, the ability for new developers to safely change code is more valuable than micro-optimizations.

---

## 5) Data fetching, caching, and correctness

### Why React Query (@tanstack/react-query)

The template uses React Query because it enforces a correct data lifecycle:

- deduping requests
- caching
- background refresh
- consistent loading/error states

### The “Hook-Only for Reads” rule

Reads (GET) should be exposed via hooks only.

- Reduces “cache bypass” mistakes.
- Makes UI state consistent across screens.

Mutations are exposed as imperative functions (or mutation hooks), because they do not represent cached state.

### Data flow diagram

```
 UI Component
    │
    ▼
 React Query Hook (useX)
    │
    ▼
 API Client (axios)
    │   adds Authorization header (token adapter)
    ▼
 Backend HTTP Handler (Axum)
    │
    ├── validate input + auth
    │
    └── call infrastructure
          │
          ▼
        Diesel (blocking)  ──>  spawn_blocking  ──>  Postgres
```

---

## 6) Backend: why Rust + Axum + Diesel (and how we keep it safe)

### Why Rust

- **Memory safety** without GC.
- **Predictable performance**.
- **Strong type system** that prevents entire classes of bugs.
- Excellent fit for high-integrity code where PHI is involved.

### Why Axum

- Async-first, simple mental model.
- Strong ecosystem and middleware patterns.
- Minimal surface area compared to heavier frameworks.

### Why Diesel (with explicit async boundary)

Diesel is synchronous. That is not a problem if we treat it as a boundary:

- **Rule**: All Diesel work occurs in `spawn_blocking`.
- **Benefit**: Tokio runtime remains responsive under load.

This is a reliability choice. It avoids the most common failure mode:

- “DB is slow → entire async server hangs.”

### Honest health checks

We implement:

- `/health/live` : process is alive
- `/health/ready`: safe to receive traffic
  - returns **503** when DB is required and missing/down

Health check design is part of SOTA because:

- it prevents load balancers from routing traffic to a broken instance
- it is observable and automatable

#### Health check decision diagram

```
              /health/ready
                    │
                    ▼
           Is DB required?
            ├───────────────┐
            │               │
           yes             no
            │               │
            ▼               ▼
     Is DB configured?     200 OK
        ├───────┐
        │       │
       no      yes
        │       │
        ▼       ▼
     503      DB probe
             ├───────┐
             │       │
            ok      fail
             │       │
             ▼       ▼
           200      503
```

---

## 7) HIPAA / PHI posture (security by construction)

This template assumes that eyeglass prescriptions can be PHI.

### Core security principle

**PHI must never be accidentally leaked by default.**

That means:

- **No PHI in logs** (client or server).
- **No PHI persisted client-side by default**.
- **Transport security** is mandatory in production.
- **Explicit boundaries** for storage, network, and database.

### Data classification model

- **Public**: app version, non-sensitive UI strings
- **Sensitive**: account metadata
- **PHI**: prescription values, medical/health-related notes, user identifiers associated with PHI

### PHI flow (ideal)

```
 User enters prescription
          │
          ▼
  Mobile UI state (in-memory)
          │
          ▼
  POST /prescriptions  (TLS)
          │
          ▼
  Backend validates + stores
          │
          ▼
  DB (encrypted at rest via platform)
          │
          ▼
  Client reads via hooks (no persistence by default)
```

### Token storage policy (simple and explicit)

We choose a policy that is robust without building an auth platform:

- **Native (iOS/Android)**: tokens stored in **SecureStore** (hardware-backed where available)
- **Web (desktop runtime)**: tokens stored in **localStorage**
  - explicitly less secure than native
  - mitigated by **short-lived tokens** + re-login on expiry

This is a deliberate tradeoff:

- avoids refresh-token complexity
- reduces long-lived credential exposure on web

### Why this is acceptable under our stated constraints

- Chat history and prescriptions are **server-source-of-truth**.
- If a user must re-login on web after token expiry, they still recover all history.

---

## 8) Operational safety and reliability

### Fail-fast configuration

Startup validates config and exits with a clear error when unsafe to run.

This prevents:

- “server runs but is misconfigured”
- “health checks lie”

### Structured errors

We define a canonical `ApiError` contract:

- stable status codes
- JSON bodies
- client-safe messages (no secrets, no PHI)

This improves:

- client reliability (predictable handling)
- auditability
- maintainability

### Minimal observability stance

SOTA does not require a massive monitoring stack in the template.

It requires:

- deterministic startup behavior
- honest health checks
- consistent error surfaces
- redaction policy for logs

You can add Sentry/OTel later, but you should not need them to have a safe baseline.

---

## 9) Maintainability principles (how we keep this clean)

### Principle: enforce good patterns by making bad ones difficult

- Root orchestrator prevents wrong installs.
- Hook-only reads prevent cache bypass.
- Config module prevents env sprawl.
- Health checks make “readiness” explicit.

### Principle: comments explain “why”, not “what”

This repository prefers **contract comments**:

- purpose
- invariants
- failure modes
- security notes

Over time, this reduces stale comment rot while keeping the “why” durable.

---

## 10) Known tradeoffs (explicitly acknowledged)

- **Expo Web tokens**: localStorage is less secure than SecureStore.
  - We mitigate via short TTL and re-login.
- **Diesel synchronous**: requires `spawn_blocking` discipline.
  - We enforce via code patterns and examples.

SOTA is not pretending these don’t exist; it documents them and contains them.

---

## 11) “Prove it” checklist (what should always remain true)

- `cargo check` succeeds.
- Backend serves:
  - `/health/live` returns 200.
  - `/health/ready` returns:
    - 200 when safe
    - 503 when DB is required but missing/down
- Mobile app runs:
  - Android, iOS
  - Web runtime for Windows/macOS
- No PHI appears in:
  - logs
  - client persistent storage by default

---

## 12) Upgrade paths (only when needed)

- **Desktop packaging**: Electron/Tauri wrapper around Expo Web build.
- **Long-lived sessions on web**: introduce refresh tokens via httpOnly cookies.
- **Stronger DB crypto needs**: add envelope encryption for specific PHI columns.

These are intentionally not defaults because they add complexity.

---

## Bottom line

This template is designed to be the “best default” in 2026:

- mobile-first
- cross-platform dev ergonomics
- secure handling assumptions for PHI/HIPAA
- explicit, honest runtime health and configuration
- maintainable patterns that prevent common failure modes
