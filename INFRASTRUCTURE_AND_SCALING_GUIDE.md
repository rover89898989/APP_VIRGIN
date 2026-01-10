# 🏗️ INFRASTRUCTURE & SCALING GUIDE
## Production-Ready Services for APP_VIRGIN Template

> **Version:** 1.0.0  
> **Last Updated:** January 10, 2026  
> **Audience:** App builders using this template  
> **Goal:** Deploy with confidence, scale without rewrites

---

## 📋 TABLE OF CONTENTS

1. [Philosophy & Principles](#1-philosophy--principles)
2. [Architecture Overview](#2-architecture-overview)
3. [Service Selection Logic Tree](#3-service-selection-logic-tree)
4. [Recommended Services (The Winners)](#4-recommended-services-the-winners)
5. [Scaling Decision Tree](#5-scaling-decision-tree)
6. [Migration Paths (Future-Proofing)](#6-migration-paths-future-proofing)
7. [Complete Setup Guide](#7-complete-setup-guide)
8. [Cost Analysis by Scale](#8-cost-analysis-by-scale)
9. [Troubleshooting](#9-troubleshooting)

---

## 1. PHILOSOPHY & PRINCIPLES

### Core Beliefs

**Start Simple, Scale Smart**
- Use managed services (not servers you maintain)
- Free tiers until you have revenue
- Upgrade based on metrics, not fear
- Avoid premature optimization

**No Vendor Lock-In**
- Use open standards (PostgreSQL, not DynamoDB)
- Containerize everything (Docker)
- Abstract infrastructure in code
- Easy migration paths at every level

**Future-Proof Architecture**
- Write code against interfaces, not services
- Environment-based configuration
- Stateless backend design
- Database-agnostic queries (Diesel helps here)

---

## 2. ARCHITECTURE OVERVIEW

### High-Level System Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        USER DEVICES                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   iOS App    │  │ Android App  │  │   Web App    │          │
│  │ (React Native│  │(React Native)│  │ (Expo Web)   │          │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘          │
└─────────┼──────────────────┼──────────────────┼─────────────────┘
          │                  │                  │
          └──────────────────┴──────────────────┘
                             │
                    ┌────────▼────────┐
                    │   CLOUDFLARE    │
                    │   (CDN + DNS)   │
                    │   Free Tier     │
                    └────────┬────────┘
                             │
          ┌──────────────────┴──────────────────┐
          │                                     │
    ┌─────▼──────┐                    ┌────────▼────────┐
    │  FLY.IO    │                    │  CLOUDFLARE R2  │
    │  Backend   │                    │  File Storage   │
    │  (Rust)    │                    │  (Images/Files) │
    │            │                    │                 │
    │  - API     │                    │  - Avatars      │
    │  - Auth    │                    │  - Uploads      │
    │  - Logic   │                    │  - Documents    │
    └─────┬──────┘                    └─────────────────┘
          │
          │
    ┌─────▼──────┐
    │ SUPABASE   │
    │ PostgreSQL │
    │            │
    │  - Users   │
    │  - Data    │
    │  - Sessions│
    └─────┬──────┘
          │
          │
    ┌─────▼──────────────────────────┐
    │      MONITORING LAYER          │
    │                                │
    │  ┌─────────┐  ┌──────────┐    │
    │  │ SENTRY  │  │ FIREBASE │    │
    │  │ Errors  │  │Analytics │    │
    │  └─────────┘  └──────────┘    │
    └────────────────────────────────┘
```

### Data Flow Diagram

```
USER ACTION (Button Press)
    │
    ▼
REACT QUERY CACHE CHECK
    │
    ├─── CACHE HIT → Return Data (Instant)
    │
    └─── CACHE MISS
         │
         ▼
API CLIENT (axios)
    │
    ├─── Add JWT Token (from SecureStore)
    └─── Add Request ID (for tracing)
         │
         ▼
CLOUDFLARE CDN (Edge Cache)
    │
    ├─── Static Assets → Cached (Fast)
    │
    └─── Dynamic API → Forward to Backend
         │
         ▼
FLY.IO (Rust Backend)
    │
    ├─── Rate Limiting (tower-governor)
    ├─── JWT Validation
    ├─── Request Logging (tracing)
    └─── Business Logic
         │
         ▼
SUPABASE (PostgreSQL)
    │
    ├─── Connection Pool (max 20)
    ├─── Diesel ORM Query
    ├─── Indexed Lookup (ms)
    └─── Return Data
         │
         ▼
RESPONSE
    │
    ├─── JSON Serialization
    ├─── CORS Headers
    └─── Cache Headers
         │
         ▼
REACT QUERY (Cache Update)
    │
    └─── Background Revalidation (if stale)
```

---

## 3. SERVICE SELECTION LOGIC TREE

### Decision Framework

```
START: Need Infrastructure?
    │
    ├─── Database Required?
    │    │
    │    ├─── YES → Use PostgreSQL (industry standard)
    │    │    │
    │    │    ├─── <1K users? → Supabase Free (500MB)
    │    │    ├─── <10K users? → Supabase Pro ($25/mo, 8GB)
    │    │    ├─── <100K users? → Neon/Railway ($50-100/mo)
    │    │    └─── 100K+ users? → Managed PostgreSQL (AWS RDS, $200+/mo)
    │    │
    │    └─── NO → Skip database, use local storage
    │
    ├─── Backend API Required?
    │    │
    │    ├─── YES → Use containerized deployment
    │    │    │
    │    │    ├─── <5K users? → Fly.io Free (3 VMs)
    │    │    ├─── <50K users? → Fly.io Paid ($20-50/mo)
    │    │    ├─── <500K users? → Railway/Render ($100-200/mo)
    │    │    └─── 500K+ users? → AWS/GCP Kubernetes ($500+/mo)
    │    │
    │    └─── NO → Use Supabase backend (if database chosen)
    │
    ├─── File Storage Required?
    │    │
    │    ├─── YES → Use S3-compatible storage
    │    │    │
    │    │    ├─── <10GB? → Cloudflare R2 Free
    │    │    ├─── <100GB? → Cloudflare R2 ($1-5/mo)
    │    │    ├─── <1TB? → Backblaze B2 ($5-10/mo)
    │    │    └─── 1TB+? → AWS S3 ($20+/mo)
    │    │
    │    └─── NO → Skip file storage
    │
    ├─── Error Tracking Required?
    │    │
    │    ├─── YES → Use error monitoring
    │    │    │
    │    │    ├─── <5K errors/mo? → Sentry Free
    │    │    ├─── <50K errors/mo? → Sentry Team ($26/mo)
    │    │    └─── 50K+ errors/mo? → Sentry Business ($80+/mo)
    │    │
    │    └─── NO → Use console.log (not recommended)
    │
    ├─── Analytics Required?
    │    │
    │    ├─── YES → Track user behavior
    │    │    │
    │    │    ├─── Basic? → Firebase Analytics Free
    │    │    ├─── Advanced? → PostHog ($0-50/mo)
    │    │    └─── Enterprise? → Mixpanel ($100+/mo)
    │    │
    │    └─── NO → Skip analytics (not recommended)
    │
    └─── Email Required?
         │
         ├─── YES → Transactional emails
         │    │
         │    ├─── <100/day? → Resend Free
         │    ├─── <1K/day? → Resend ($20/mo)
         │    └─── 1K+/day? → SendGrid/Postmark ($50+/mo)
         │
         └─── NO → Skip email
```

### Service Compatibility Matrix

| Service | Works With | Conflicts With | Migration Effort |
|---------|-----------|----------------|------------------|
| **Supabase** | Any backend, any framework | None | Low (standard PostgreSQL) |
| **Fly.io** | Any containerized app | None | Low (Docker standard) |
| **Cloudflare R2** | Any S3-compatible client | None | Low (S3 API standard) |
| **Sentry** | Any language/framework | None | Zero (SDK swap) |
| **Firebase Analytics** | Any mobile framework | None | Low (SDK swap) |
| **Resend** | Any email library | None | Zero (SMTP standard) |

---

## 4. RECOMMENDED SERVICES (THE WINNERS)

### 4.1 DATABASE: Supabase (PostgreSQL)

**Why Supabase?**
- Real PostgreSQL (not proprietary database)
- Auto-generated REST API (bonus feature)
- Built-in authentication (optional, you have Rust auth)
- Real-time subscriptions (WebSocket based)
- Free tier: 500MB database, unlimited API requests
- Dashboard for SQL queries, table management

**Setup (2 minutes):**
```bash
# 1. Go to https://supabase.com
# 2. Click "New Project"
# 3. Set password, region (choose closest to users)
# 4. Copy connection string

# Connection string format:
postgresql://postgres:[PASSWORD]@db.[PROJECT].supabase.co:5432/postgres
```

**Rust Integration:**
```rust
// backend/.env
DATABASE_URL=postgresql://postgres:your-password@db.your-project.supabase.co:5432/postgres

// backend/src/db.rs
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    r2d2::Pool::builder()
        .max_size(20) // Supabase free tier allows 20 connections
        .build(manager)
        .expect("Failed to create pool")
}
```

**Free Tier Limits:**
- Storage: 500MB
- Connections: 20 concurrent
- Egress: 2GB/month
- Pauses after 7 days inactivity (resume instant)

**Upgrade Triggers:**
- Storage >400MB → Upgrade to Pro ($25/mo, 8GB)
- Connections >15 active → Upgrade or optimize queries
- Egress >1.5GB/mo → Optimize large queries

**Migration Path (No Code Changes):**
```
Supabase Free
    ↓ (400MB storage reached)
Supabase Pro ($25/mo, 8GB)
    ↓ (6GB storage reached)
Neon/Railway ($50/mo, 25GB+)
    ↓ (100K+ users)
AWS RDS PostgreSQL ($200+/mo, dedicated)
    ↓ (1M+ users)
AWS Aurora PostgreSQL ($500+/mo, auto-scaling)
```

**Why No Code Changes:**
- All use standard PostgreSQL
- Same connection string format
- Diesel ORM works identically
- Export: `pg_dump`, Import: `psql`

---

### 4.2 BACKEND HOSTING: Fly.io

**Why Fly.io?**
- Deploy Docker containers globally
- Automatic HTTPS certificates
- Health checks built-in
- Graceful zero-downtime deploys
- 3 VMs free (256MB RAM each)
- Global edge network (low latency)

**Setup (10 minutes):**
```bash
# 1. Install Fly CLI
curl -L https://fly.io/install.sh | sh

# 2. Login
fly auth login

# 3. Create app (from backend directory)
cd backend
fly launch

# Fly.io detects Dockerfile and asks:
# - App name? your-app-backend
# - Region? Choose closest to users
# - PostgreSQL? No (using Supabase)
# - Deploy now? Yes

# 4. Set environment variables
fly secrets set JWT_SECRET=your-secret-key
fly secrets set DATABASE_URL=postgresql://...

# 5. Deploy
fly deploy
```

**Dockerfile (already in template):**
```dockerfile
# backend/Dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
CMD ["backend"]
```

**Auto-Scaling Configuration:**
```toml
# fly.toml
app = "your-app-backend"

[build]
  dockerfile = "Dockerfile"

[env]
  PORT = "8000"

[[services]]
  http_checks = []
  internal_port = 8000
  protocol = "tcp"

  [[services.ports]]
    handlers = ["http"]
    port = 80

  [[services.ports]]
    handlers = ["tls", "http"]
    port = 443

  [services.concurrency]
    hard_limit = 100
    soft_limit = 80
    type = "connections"

  [[services.tcp_checks]]
    grace_period = "10s"
    interval = "15s"
    restart_limit = 0
    timeout = "2s"

# Auto-scaling (scales VMs based on load)
[[vm]]
  cpu_kind = "shared"
  cpus = 1
  memory_mb = 256

[auto_scaling]
  min_count = 1  # Minimum VMs
  max_count = 3  # Maximum VMs (free tier)
```

**Free Tier Limits:**
- VMs: 3 shared CPU VMs (256MB each)
- Storage: 3GB persistent volumes
- Bandwidth: 160GB/month
- Auto HTTPS: Unlimited

**Upgrade Triggers:**
- CPU >80% sustained → Add more VMs ($5-10/mo each)
- Memory >200MB → Upgrade to 512MB VMs
- Bandwidth >120GB/mo → Consider CDN

**Migration Path:**
```
Fly.io Free (3 VMs, 256MB)
    ↓ (CPU/Memory pressure)
Fly.io Paid (more VMs, 512MB-1GB)
    ↓ (Complex scaling needs)
Railway/Render ($100-200/mo, simpler scaling)
    ↓ (100K+ users, need control)
AWS ECS/Fargate ($200+/mo, auto-scaling containers)
    ↓ (1M+ users, complex architecture)
Kubernetes (AWS EKS, GCP GKE, $500+/mo)
```

**Why Minimal Code Changes:**
- Docker container works anywhere
- Environment variables same pattern
- Health check endpoint unchanged
- Database connection string portable

---

### 4.3 FILE STORAGE: Cloudflare R2

**Why Cloudflare R2?**
- S3-compatible API (use AWS SDK)
- Zero egress fees (save $$$)
- 10GB free storage
- Unlimited bandwidth (!)
- Global CDN built-in

**Setup (5 minutes):**
```bash
# 1. Go to Cloudflare Dashboard
# 2. R2 → Create Bucket → "your-app-uploads"
# 3. API Tokens → Create API Token
# 4. Copy Access Key ID and Secret Access Key
```

**Rust Integration (S3-compatible):**
```rust
// Cargo.toml
[dependencies]
aws-sdk-s3 = "1.0"
aws-config = "1.0"

// backend/src/storage.rs
use aws_sdk_s3::{Client, Config, Credentials, Region};

pub async fn create_s3_client() -> Client {
    let creds = Credentials::new(
        std::env::var("R2_ACCESS_KEY_ID").unwrap(),
        std::env::var("R2_SECRET_ACCESS_KEY").unwrap(),
        None,
        None,
        "r2",
    );
    
    let config = Config::builder()
        .credentials_provider(creds)
        .region(Region::new("auto"))
        .endpoint_url(format!(
            "https://{}.r2.cloudflarestorage.com",
            std::env::var("R2_ACCOUNT_ID").unwrap()
        ))
        .build();
    
    Client::from_conf(config)
}

// Upload file
pub async fn upload_file(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(data.into())
        .send()
        .await?;
    
    Ok(())
}

// Generate presigned URL (for direct upload from mobile)
pub async fn generate_upload_url(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let presigned = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .presigned(PresigningConfig::expires_in(Duration::from_secs(3600))?)
        .await?;
    
    Ok(presigned.uri().to_string())
}
```

**Mobile Integration:**
```typescript
// mobile/src/api/storage.ts

/**
 * Upload image to R2
 * 
 * FLOW:
 * 1. Request upload URL from backend
 * 2. Upload directly to R2 (bypasses backend)
 * 3. Save file reference in database
 */
export async function uploadImage(imageUri: string): Promise<string> {
  // 1. Get presigned upload URL from backend
  const { uploadUrl, fileKey } = await api.post('/storage/upload-url', {
    filename: 'avatar.jpg',
    contentType: 'image/jpeg',
  });
  
  // 2. Upload directly to R2
  const formData = new FormData();
  formData.append('file', {
    uri: imageUri,
    type: 'image/jpeg',
    name: 'avatar.jpg',
  });
  
  await fetch(uploadUrl, {
    method: 'PUT',
    body: formData,
  });
  
  // 3. Return public URL
  return `https://cdn.yourapp.com/${fileKey}`;
}
```

**Free Tier Limits:**
- Storage: 10GB
- Class A Operations: 1M/month (uploads)
- Class B Operations: 10M/month (downloads)
- Egress: Unlimited (!)

**Upgrade Triggers:**
- Storage >8GB → Paid tier ($0.015/GB/month)
- Operations >500K uploads/mo → Monitor costs

**Migration Path (Zero Code Changes - S3 API Standard):**
```
Cloudflare R2 Free (10GB)
    ↓ (8GB storage reached)
Cloudflare R2 Paid ($0.015/GB/mo)
    ↓ (Need AWS integration)
AWS S3 ($0.023/GB/mo + egress fees)
    ↓ (Multi-region redundancy)
AWS S3 + CloudFront CDN
```

**Why Zero Code Changes:**
- All use S3 API
- Change endpoint URL only
- Same SDK (aws-sdk-s3)
- Same presigned URL logic

---

### 4.4 MONITORING: Sentry

**Why Sentry?**
- Best-in-class error tracking
- Source maps (see original code, not minified)
- Performance monitoring
- Release tracking
- Breadcrumbs (events leading to error)
- 5K errors/month free

**Setup (3 minutes):**
```bash
# Backend (Rust)
cargo add sentry

# Mobile (React Native)
npx expo install @sentry/react-native
```

**Rust Integration:**
```rust
// backend/src/main.rs
use sentry;

#[tokio::main]
async fn main() {
    let _guard = sentry::init((
        std::env::var("SENTRY_DSN").unwrap(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(std::env::var("ENVIRONMENT").unwrap().into()),
            ..Default::default()
        },
    ));
    
    // Your app code
}
```

**Mobile Integration (already in my additions):**
```typescript
// mobile/src/shared/utils/monitoring.ts
import * as Sentry from '@sentry/react-native';

Sentry.init({
  dsn: process.env.EXPO_PUBLIC_SENTRY_DSN,
  environment: process.env.EXPO_PUBLIC_ENV || 'production',
  tracesSampleRate: 1.0,
});
```

**Free Tier Limits:**
- Errors: 5K/month
- Performance: 10K transactions/month
- Retention: 30 days
- Team: 1 user

**Upgrade Triggers:**
- Errors >4K/mo → Team plan ($26/mo, 50K errors)
- Need more users → Add seats ($26/user/mo)

**Migration Path:**
```
Sentry Free (5K errors/mo)
    ↓ (4K errors/mo reached)
Sentry Team ($26/mo, 50K errors/mo)
    ↓ (Need custom integrations)
Sentry Business ($80/mo, unlimited errors)
    ↓ (Enterprise needs)
Self-hosted Sentry (free, but you manage)
```

---

### 4.5 ANALYTICS: Firebase Analytics

**Why Firebase Analytics?**
- Unlimited events (free)
- Mobile-optimized
- Automatic screen tracking
- Integration with other Firebase services
- BigQuery export (for advanced analysis)

**Setup (10 minutes):**
```bash
# Install
npx expo install @react-native-firebase/app @react-native-firebase/analytics

# Configure (see Firebase Console for credentials)
```

**Mobile Integration:**
```typescript
// mobile/src/shared/utils/analytics.ts
import analytics from '@react-native-firebase/analytics';

export const trackEvent = async (
  eventName: string,
  params?: Record<string, any>
) => {
  await analytics().logEvent(eventName, params);
};

// Usage
await trackEvent('login', { method: 'email' });
await trackEvent('purchase', { value: 29.99, currency: 'USD' });
```

**Free Tier Limits:**
- Events: Unlimited
- Properties: 25 per event
- User properties: 25 per user
- Retention: Indefinite

**Upgrade Path (Free Forever for Basic Analytics):**
```
Firebase Analytics Free
    ↓ (Need advanced analysis)
BigQuery Export (pay per query)
    ↓ (Need real-time)
Mixpanel/Amplitude ($100+/mo)
```

---

### 4.6 EMAIL: Resend

**Why Resend?**
- Simple API
- Great deliverability
- Beautiful default templates
- React Email integration
- 100 emails/day free

**Setup (5 minutes):**
```bash
# 1. Go to resend.com
# 2. Create API key
# 3. Verify domain (add DNS records)
```

**Rust Integration:**
```rust
// Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde_json = "1.0"

// backend/src/email.rs
use reqwest;
use serde_json::json;

pub async fn send_email(
    to: &str,
    subject: &str,
    html: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let response = client
        .post("https://api.resend.com/emails")
        .header("Authorization", format!("Bearer {}", std::env::var("RESEND_API_KEY")?))
        .json(&json!({
            "from": "noreply@yourapp.com",
            "to": [to],
            "subject": subject,
            "html": html,
        }))
        .send()
        .await?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed to send email: {}", response.status()).into())
    }
}

// Usage: Password reset
send_email(
    user_email,
    "Reset your password",
    format!(
        "Click here to reset: https://yourapp.com/reset-password/{}",
        reset_token
    ),
).await?;
```

**Free Tier Limits:**
- Emails: 100/day (3K/month)
- Domains: 1
- API calls: Unlimited

**Upgrade Triggers:**
- Emails >80/day → Paid plan ($20/mo, 50K/month)
- Need multiple domains → Business plan

**Migration Path:**
```
Resend Free (100/day)
    ↓ (80/day reached)
Resend Paid ($20/mo, 50K/month)
    ↓ (Need advanced features)
SendGrid/Postmark ($50+/mo)
```

---

## 5. SCALING DECISION TREE

### When to Scale (Metrics-Based Triggers)

```
┌─────────────────────────────────────────────────────────┐
│          SCALING DECISION FRAMEWORK                     │
└─────────────────────────────────────────────────────────┘

MONITOR THESE METRICS WEEKLY:
├── Database
│   ├── Storage: >80% of quota → Upgrade tier
│   ├── Connections: >80% of max → Optimize or upgrade
│   ├── Query time: >100ms p95 → Add indexes or upgrade
│   └── CPU: >70% sustained → Upgrade tier
│
├── Backend
│   ├── Response time: >200ms p95 → Scale VMs or optimize
│   ├── CPU: >80% sustained → Add VMs
│   ├── Memory: >80% sustained → Upgrade VM size
│   └── Error rate: >1% → Investigate and fix
│
├── Storage
│   ├── Size: >80% of quota → Upgrade tier
│   ├── Operations: >80% of quota → Optimize or upgrade
│   └── Bandwidth: >80% of quota → Add CDN
│
└── Costs
    ├── >$50/mo with <1K users → Something wrong, investigate
    ├── >$200/mo with <10K users → Optimize before scaling
    └── >$500/mo with <50K users → Normal, but audit costs

DECISION MATRIX:

User Count → Monthly Cost Target → Service Tier
────────────────────────────────────────────────
0-1K      → $0-10          → All free tiers
1K-5K     → $10-50         → Database Pro, Backend free
5K-10K    → $50-100        → Database Pro, Backend paid
10K-50K   → $100-300       → All paid tiers, optimize
50K-100K  → $300-800       → Managed services, monitoring
100K-500K → $800-2000      → Enterprise tiers, dedicated
500K-1M   → $2000-5000     → Multi-region, redundancy
1M+       → $5000+         → Custom infrastructure
```

### Performance Thresholds

```
RESPONSE TIME TARGETS (95th percentile):

Mobile App:
├── Screen load: <2 seconds
├── Button press → feedback: <100ms
├── API call → data shown: <500ms
└── Image load: <1 second

Backend API:
├── Health check: <50ms
├── Auth endpoints: <200ms
├── Simple queries: <100ms
├── Complex queries: <500ms
└── File uploads: <2 seconds (10MB)

Database:
├── Indexed queries: <10ms
├── Table scans: <100ms (avoid!)
├── Joins (2-3 tables): <50ms
└── Aggregations: <200ms

IF TARGETS MISSED 3 DAYS IN A ROW:
1. Investigate (slow query log, APM)
2. Optimize (indexes, caching, code)
3. Scale (last resort)
```

---

## 6. MIGRATION PATHS (FUTURE-PROOFING)

### Principle: Abstract Infrastructure in Code

**Database Abstraction Pattern:**
```rust
// backend/src/db.rs

/// Database pool trait
/// Allows swapping database implementations
pub trait DatabasePool: Send + Sync {
    type Connection;
    fn get_connection(&self) -> Result<Self::Connection, Error>;
}

/// PostgreSQL implementation (current)
pub struct PostgresPool {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl DatabasePool for PostgresPool {
    type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
    
    fn get_connection(&self) -> Result<Self::Connection, Error> {
        self.pool.get().map_err(|e| Error::DatabaseError(e.to_string()))
    }
}

// Future: Can add MySQL, SQLite implementations without changing business logic
```

**Storage Abstraction Pattern:**
```rust
// backend/src/storage.rs

/// Storage backend trait
/// S3-compatible API means easy swapping
#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn upload(&self, key: &str, data: Vec<u8>) -> Result<String, Error>;
    async fn download(&self, key: &str) -> Result<Vec<u8>, Error>;
    async fn delete(&self, key: &str) -> Result<(), Error>;
    async fn generate_presigned_url(&self, key: &str) -> Result<String, Error>;
}

/// Cloudflare R2 implementation (current)
pub struct R2Storage {
    client: aws_sdk_s3::Client,
    bucket: String,
}

#[async_trait]
impl StorageBackend for R2Storage {
    async fn upload(&self, key: &str, data: Vec<u8>) -> Result<String, Error> {
        // R2-specific implementation
    }
    // ... other methods
}

// Future: Can add S3Storage, LocalStorage without changing business logic
```

**Environment-Based Configuration:**
```rust
// backend/src/config.rs

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub storage_backend: StorageBackend,
    pub monitoring_enabled: bool,
    // ... other config
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        let env = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        
        match env.as_str() {
            "development" => Self::development(),
            "staging" => Self::staging(),
            "production" => Self::production(),
            _ => Err(Error::InvalidEnvironment(env)),
        }
    }
    
    fn development() -> Result<Self, Error> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            storage_backend: StorageBackend::Local, // Use local filesystem
            monitoring_enabled: false,
        })
    }
    
    fn production() -> Result<Self, Error> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            storage_backend: StorageBackend::R2, // Use Cloudflare R2
            monitoring_enabled: true,
        })
    }
}
```

### Migration Playbooks

#### Database Migration: Supabase → AWS RDS

```bash
# 1. Export from Supabase
pg_dump -h db.your-project.supabase.co \
        -U postgres \
        -d postgres \
        -F c \
        -f backup.dump

# 2. Create AWS RDS instance
# (Use AWS Console or Terraform)

# 3. Import to AWS RDS
pg_restore -h your-rds.amazonaws.com \
           -U postgres \
           -d your_database \
           backup.dump

# 4. Update connection string in environment
# OLD: DATABASE_URL=postgresql://postgres:...@db.supabase.co:5432/postgres
# NEW: DATABASE_URL=postgresql://postgres:...@your-rds.amazonaws.com:5432/your_database

# 5. Test connection
cargo test

# 6. Deploy with new connection string
fly deploy

# ZERO CODE CHANGES REQUIRED
```

**Estimated Downtime:** <5 minutes (during DNS switch)

#### Backend Migration: Fly.io → AWS ECS

```bash
# 1. Your app is already containerized (Dockerfile)
# 2. Push to AWS ECR (container registry)
docker build -t your-app .
docker tag your-app:latest 123456789.dkr.ecr.us-east-1.amazonaws.com/your-app:latest
docker push 123456789.dkr.ecr.us-east-1.amazonaws.com/your-app:latest

# 3. Create ECS task definition (use AWS Console or Terraform)
# - Use same Dockerfile
# - Set environment variables (same as Fly.io)
# - Configure health check (same /health endpoint)

# 4. Create ECS service
# - Auto-scaling based on CPU/memory
# - Load balancer (Application Load Balancer)
# - SSL certificate (AWS Certificate Manager)

# 5. Update DNS to point to ALB
# OLD: your-app.fly.dev
# NEW: your-app.com (pointing to ALB)

# ZERO CODE CHANGES REQUIRED
```

**Estimated Downtime:** <1 minute (DNS propagation)

#### Storage Migration: Cloudflare R2 → AWS S3

```bash
# 1. Create S3 bucket
aws s3 mb s3://your-app-uploads

# 2. Copy files from R2 to S3
rclone sync r2:your-app-uploads s3:your-app-uploads

# 3. Update environment variable
# OLD: R2_ENDPOINT=https://[account].r2.cloudflarestorage.com
# NEW: S3_ENDPOINT=https://s3.amazonaws.com

# 4. Code change (only endpoint URL)
# backend/src/storage.rs
let config = Config::builder()
    .region(Region::new("us-east-1"))
    .endpoint_url("https://s3.amazonaws.com") // Changed this line
    .build();

# 5. Deploy
cargo build --release
fly deploy

# MINIMAL CODE CHANGE (1 line)
```

**Estimated Downtime:** 0 (gradual migration, serve from both during transition)

---

## 7. COMPLETE SETUP GUIDE

### Step-by-Step: From Zero to Production (1 Hour)

#### Prerequisites
```bash
# 1. Install tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
npm install -g expo-cli
curl -L https://fly.io/install.sh | sh

# 2. Clone template
git clone https://github.com/YOUR_USERNAME/APP_VIRGIN.git my-app
cd my-app
```

#### Week 1: Database Setup (Day 1)

**Supabase Setup (5 minutes):**
```bash
# 1. Go to https://supabase.com
# 2. Click "New Project"
# 3. Fill in:
#    - Name: my-app
#    - Database Password: (generate strong password)
#    - Region: (choose closest to target users)
# 4. Click "Create Project"
# 5. Wait 2 minutes for provisioning

# 6. Copy connection string
# Dashboard → Settings → Database → Connection String → URI

# 7. Add to backend .env
cd backend
echo "DATABASE_URL=postgresql://postgres:[PASSWORD]@db.[PROJECT].supabase.co:5432/postgres" > .env

# 8. Run migrations
diesel migration run

# 9. Test connection
cargo test
```

**Success Criteria:**
- ✅ Database created
- ✅ Migrations run successfully
- ✅ Tests pass

#### Week 1: Backend Deployment (Day 2-3)

**Fly.io Setup (15 minutes):**
```bash
# 1. Login to Fly.io
fly auth login

# 2. Navigate to backend
cd backend

# 3. Create Fly.io app
fly launch
# Answer prompts:
# - App name: my-app-backend
# - Region: (same as database)
# - PostgreSQL: No (using Supabase)
# - Deploy now: No (set secrets first)

# 4. Set secrets
fly secrets set JWT_SECRET=$(openssl rand -hex 32)
fly secrets set DATABASE_URL="postgresql://postgres:[PASSWORD]@db.[PROJECT].supabase.co:5432/postgres"
fly secrets set RUST_LOG=info

# 5. Deploy
fly deploy

# 6. Test deployment
curl https://my-app-backend.fly.dev/health
# Should return: {"status":"ok"}

# 7. Test API
curl https://my-app-backend.fly.dev/api/v1/health
# Should return health status with database connection
```

**Success Criteria:**
- ✅ Backend deployed
- ✅ Health check returns 200
- ✅ Database connection works
- ✅ HTTPS certificate active

#### Week 1: Mobile Setup (Day 4-5)

**Expo Setup (10 minutes):**
```bash
# 1. Navigate to mobile
cd mobile

# 2. Install dependencies
npm install

# 3. Configure environment
cp .env.example .env

# 4. Update .env
# EXPO_PUBLIC_API_URL=https://my-app-backend.fly.dev/api

# 5. Start development
npm start

# 6. Test on device
# - Scan QR code with Expo Go app (iOS/Android)
# - OR press 'w' for web browser

# 7. Verify API connection
# - App should load without errors
# - Check network tab for successful API calls
```

**Success Criteria:**
- ✅ App runs on device
- ✅ API calls succeed
- ✅ No console errors

#### Week 2: Monitoring Setup (Day 1)

**Sentry Setup (10 minutes):**
```bash
# 1. Go to https://sentry.io
# 2. Create account (GitHub login)
# 3. Create project
#    - Platform: React Native
#    - Name: my-app-mobile
# 4. Copy DSN

# 5. Configure mobile
cd mobile
echo "EXPO_PUBLIC_SENTRY_DSN=https://[KEY]@[ORG].ingest.sentry.io/[PROJECT]" >> .env

# 6. Test error tracking
# Add to mobile/App.tsx:
throw new Error("Test Sentry integration");

# 7. Verify in Sentry dashboard
# - Error should appear in Issues
# - Remove test error after verification

# 8. Create backend project
# - Platform: Rust
# - Name: my-app-backend

# 9. Configure backend
cd ../backend
fly secrets set SENTRY_DSN="https://[KEY]@[ORG].ingest.sentry.io/[PROJECT]"

# 10. Deploy
fly deploy
```

**Success Criteria:**
- ✅ Test error appears in Sentry
- ✅ Source maps working (see original code)
- ✅ Backend errors tracked

#### Week 2: File Storage (Day 2-3)

**Cloudflare R2 Setup (10 minutes):**
```bash
# 1. Go to Cloudflare Dashboard
# 2. Navigate to R2
# 3. Create bucket: "my-app-uploads"
# 4. Generate API token
#    - R2 → Manage R2 API Tokens → Create API Token
#    - Permissions: Object Read & Write
#    - Copy Access Key ID and Secret Access Key

# 5. Configure backend
cd backend
fly secrets set R2_ACCESS_KEY_ID="[ACCESS_KEY]"
fly secrets set R2_SECRET_ACCESS_KEY="[SECRET_KEY]"
fly secrets set R2_ACCOUNT_ID="[ACCOUNT_ID]"
fly secrets set R2_BUCKET="my-app-uploads"

# 6. Deploy
fly deploy

# 7. Test upload
# (Use Postman or curl to test /storage/upload endpoint)
```

**Success Criteria:**
- ✅ File upload works
- ✅ Files visible in R2 dashboard
- ✅ Presigned URLs work

#### Week 2: Analytics (Day 4)

**Firebase Setup (15 minutes):**
```bash
# 1. Go to https://console.firebase.google.com
# 2. Create project: "my-app"
# 3. Add iOS app
#    - Bundle ID: com.yourcompany.myapp
#    - Download GoogleService-Info.plist
# 4. Add Android app
#    - Package name: com.yourcompany.myapp
#    - Download google-services.json

# 5. Configure mobile
cd mobile
npm install @react-native-firebase/app @react-native-firebase/analytics

# 6. Add config files
# - iOS: mobile/ios/GoogleService-Info.plist
# - Android: mobile/android/app/google-services.json

# 7. Test analytics
# Add to mobile/App.tsx:
import analytics from '@react-native-firebase/analytics';
analytics().logEvent('app_open');

# 8. Verify in Firebase Console
# - Analytics → Events → app_open should appear (24 hour delay)
```

**Success Criteria:**
- ✅ Firebase configured
- ✅ Events tracked (check after 24h)
- ✅ No console errors

#### Week 3: Email (Day 1)

**Resend Setup (10 minutes):**
```bash
# 1. Go to https://resend.com
# 2. Create account
# 3. Add domain
#    - Domain: yourapp.com
#    - Add DNS records (provided by Resend)
#    - Verify domain

# 4. Create API key
# 5. Configure backend
cd backend
fly secrets set RESEND_API_KEY="[API_KEY]"

# 6. Deploy
fly deploy

# 7. Test email
# (Implement password reset endpoint and test)
```

**Success Criteria:**
- ✅ Domain verified
- ✅ Test email delivered
- ✅ Emails not in spam

#### Week 3: Custom Domain (Day 2-3)

**Domain Setup (20 minutes):**
```bash
# 1. Buy domain (Cloudflare Registrar, Namecheap, etc.)
# 2. Add to Cloudflare (if not already)
#    - Add Site → yourapp.com
#    - Change nameservers at registrar

# 3. Configure DNS (Cloudflare Dashboard)
# A record:
#    - Name: api
#    - Content: (Fly.io IP - get from 'fly ips list')
#    - Proxy: OFF (orange cloud off)

# 4. Add certificate to Fly.io
fly certs add api.yourapp.com

# 5. Wait for DNS propagation (5-60 minutes)
# 6. Test
curl https://api.yourapp.com/health

# 7. Update mobile .env
EXPO_PUBLIC_API_URL=https://api.yourapp.com/api
```

**Success Criteria:**
- ✅ Domain resolves
- ✅ HTTPS certificate active
- ✅ Mobile app connects to custom domain

#### Week 4: Production Hardening (Day 1-5)

**Checklist:**
```bash
# 1. Enable auto-backups (Supabase)
# - Dashboard → Settings → Database → Backups → Enable

# 2. Configure CORS (backend)
# - Update allowed origins to production domain only
# - Remove localhost from production

# 3. Set up monitoring alerts (Sentry)
# - Alerts → Create Alert
# - Trigger: >10 errors in 5 minutes
# - Notification: Email/Slack

# 4. Load testing
k6 run load-test.js
# - Target: 1000 concurrent users
# - Check response times <500ms
# - Check error rate <0.1%

# 5. Security audit
npm audit fix           # Frontend
cargo audit             # Backend
docker scan yourimage   # Container

# 6. Update documentation
# - Update README with production URLs
# - Document deployment process
# - Create runbook for common issues

# 7. Set up staging environment
# - Create separate Fly.io app (my-app-backend-staging)
# - Create separate Supabase project (my-app-staging)
# - Deploy to staging before production
```

**Success Criteria:**
- ✅ Backups enabled and tested
- ✅ Load test passed
- ✅ No critical vulnerabilities
- ✅ Staging environment works
- ✅ Documentation complete

---

## 8. COST ANALYSIS BY SCALE

### Monthly Cost Breakdown

```
┌─────────────────────────────────────────────────────────────────┐
│                    COST BY USER COUNT                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  0-1,000 USERS                                                  │
│  ├── Database (Supabase Free)         $0                        │
│  ├── Backend (Fly.io Free)            $0                        │
│  ├── Storage (Cloudflare R2 Free)     $0                        │
│  ├── Monitoring (Sentry Free)         $0                        │
│  ├── Analytics (Firebase Free)        $0                        │
│  ├── Email (Resend Free)              $0                        │
│  ├── Domain (Cloudflare)              $0.75/mo ($9/year)        │
│  └── TOTAL                             $0.75/mo                 │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1,000-5,000 USERS                                              │
│  ├── Database (Supabase Pro)          $25                       │
│  ├── Backend (Fly.io Free)            $0                        │
│  ├── Storage (Cloudflare R2)          $1-2                      │
│  ├── Monitoring (Sentry Free)         $0                        │
│  ├── Analytics (Firebase Free)        $0                        │
│  ├── Email (Resend Free)              $0                        │
│  ├── Domain                            $0.75                     │
│  └── TOTAL                             $27-28/mo                │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  5,000-10,000 USERS                                             │
│  ├── Database (Supabase Pro)          $25                       │
│  ├── Backend (Fly.io 3-5 VMs)         $15-25                    │
│  ├── Storage (Cloudflare R2)          $3-5                      │
│  ├── Monitoring (Sentry Team)         $26                       │
│  ├── Analytics (Firebase Free)        $0                        │
│  ├── Email (Resend Paid)              $20                       │
│  ├── Domain                            $0.75                     │
│  └── TOTAL                             $90-102/mo               │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  10,000-50,000 USERS                                            │
│  ├── Database (Neon/Railway)          $50-100                   │
│  ├── Backend (Fly.io 10+ VMs)         $50-100                   │
│  ├── Storage (Cloudflare R2)          $10-20                    │
│  ├── Monitoring (Sentry Team)         $26                       │
│  ├── Analytics (Firebase Free)        $0                        │
│  ├── Email (Resend/SendGrid)          $50-80                    │
│  ├── Domain + CDN                      $5-10                     │
│  └── TOTAL                             $191-336/mo              │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  50,000-100,000 USERS                                           │
│  ├── Database (AWS RDS)               $200-400                  │
│  ├── Backend (AWS ECS/Fargate)        $200-400                  │
│  ├── Storage (AWS S3)                 $50-100                   │
│  ├── Monitoring (Sentry Business)     $80-150                   │
│  ├── Analytics (Mixpanel)             $100-200                  │
│  ├── Email (SendGrid)                 $100-200                  │
│  ├── CDN (Cloudflare)                 $20-50                    │
│  └── TOTAL                             $750-1,500/mo            │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  100,000-500,000 USERS                                          │
│  ├── Database (AWS Aurora)            $500-1,000                │
│  ├── Backend (Kubernetes)             $500-1,000                │
│  ├── Storage (AWS S3)                 $200-500                  │
│  ├── Monitoring (Datadog)             $200-500                  │
│  ├── Analytics (Amplitude)            $200-500                  │
│  ├── Email (SendGrid)                 $200-500                  │
│  ├── CDN (Cloudflare)                 $50-200                   │
│  └── TOTAL                             $1,850-4,200/mo          │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  500,000+ USERS                                                 │
│  ├── Database (Multi-region)          $2,000-5,000              │
│  ├── Backend (Multi-region K8s)       $2,000-5,000              │
│  ├── Storage (Multi-region S3)        $1,000-2,000              │
│  ├── Monitoring (Datadog Enterprise)  $500-1,000                │
│  ├── Analytics (Amplitude Enterprise) $500-1,000                │
│  ├── Email (SendGrid Enterprise)      $500-1,000                │
│  ├── CDN (Enterprise)                 $200-500                  │
│  └── TOTAL                             $6,700-15,500/mo         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

REVENUE TARGETS (Suggested):
├── 0-1K users     → Pre-revenue (free tier)
├── 1K-5K users    → $100-500/mo revenue (3-20x infrastructure cost)
├── 5K-10K users   → $500-2K/mo revenue (5-20x infrastructure cost)
├── 10K-50K users  → $2K-20K/mo revenue (10-60x infrastructure cost)
└── 50K+ users     → $20K+/mo revenue (10-30x infrastructure cost)

PROFITABILITY RULE OF THUMB:
Infrastructure should be <10% of revenue
```

---

## 9. TROUBLESHOOTING

### Common Issues & Solutions

#### Database Connection Failed

**Symptom:**
```
Error: Failed to connect to database
could not connect to server: Connection refused
```

**Diagnosis:**
```bash
# 1. Check database is running
# Supabase: Dashboard should show "Active"

# 2. Test connection manually
psql "postgresql://postgres:[PASSWORD]@db.[PROJECT].supabase.co:5432/postgres"

# 3. Check connection string format
echo $DATABASE_URL
# Should be: postgresql://postgres:password@host:5432/database
```

**Solutions:**
```bash
# Solution 1: Verify credentials
# - Check password in .env matches Supabase dashboard
# - Check project name in URL is correct

# Solution 2: Check firewall
# - Supabase: Whitelist your IP (Dashboard → Settings → Database → Connection Pooling)
# - Fly.io: Should work by default (uses Supabase's connection pooler)

# Solution 3: Check SSL mode
# Add ?sslmode=require to connection string
DATABASE_URL=postgresql://postgres:password@host:5432/database?sslmode=require
```

#### Backend Deploy Failed

**Symptom:**
```
Error: failed to deploy
health check failed
```

**Diagnosis:**
```bash
# 1. Check build logs
fly logs

# 2. Check health endpoint locally
docker build -t test .
docker run -p 8000:8000 test
curl http://localhost:8000/health

# 3. Check environment variables
fly secrets list
```

**Solutions:**
```bash
# Solution 1: Health check timeout
# Increase timeout in fly.toml
[[services.tcp_checks]]
  timeout = "10s"  # Increase from 2s

# Solution 2: Missing secrets
fly secrets set JWT_SECRET=$(openssl rand -hex 32)
fly secrets set DATABASE_URL="postgresql://..."

# Solution 3: Port mismatch
# Ensure PORT in fly.toml matches Rust code
# fly.toml: internal_port = 8000
# Rust: .bind("0.0.0.0:8000")
```

#### Mobile App Won't Connect

**Symptom:**
```
Network request failed
Unable to connect to API
```

**Diagnosis:**
```bash
# 1. Check API URL in .env
cat mobile/.env
# EXPO_PUBLIC_API_URL should be https://your-app.fly.dev/api

# 2. Test API manually
curl https://your-app.fly.dev/api/v1/health

# 3. Check CORS
curl -H "Origin: http://localhost:19006" \
     -H "Access-Control-Request-Method: GET" \
     -X OPTIONS \
     https://your-app.fly.dev/api/v1/health
```

**Solutions:**
```bash
# Solution 1: Update API URL
# mobile/.env
EXPO_PUBLIC_API_URL=https://your-app.fly.dev/api

# Solution 2: Configure CORS
# backend/src/main.rs - Allow Expo development origin
let cors = CorsLayer::new()
    .allow_origin(["http://localhost:19006".parse().unwrap()])
    .allow_methods([Method::GET, Method::POST])
    .allow_credentials(true);

# Solution 3: Check HTTPS
# Mobile apps require HTTPS in production
# Use Fly.io automatic HTTPS or add custom domain
```

#### File Upload Fails

**Symptom:**
```
Error: Access Denied
403 Forbidden
```

**Diagnosis:**
```bash
# 1. Check R2 credentials
fly secrets list | grep R2

# 2. Test credentials manually
aws s3 ls s3://your-bucket \
  --endpoint-url https://[account].r2.cloudflarestorage.com

# 3. Check bucket permissions
# Cloudflare Dashboard → R2 → Bucket → Settings
```

**Solutions:**
```bash
# Solution 1: Regenerate API token
# Cloudflare → R2 → API Tokens → Create New
# Update secrets:
fly secrets set R2_ACCESS_KEY_ID="new-key"
fly secrets set R2_SECRET_ACCESS_KEY="new-secret"

# Solution 2: Check bucket name
fly secrets set R2_BUCKET="correct-bucket-name"

# Solution 3: Verify endpoint URL
# Should be: https://[account-id].r2.cloudflarestorage.com
# NOT: https://[project].r2.cloudflarestorage.com
```

#### Slow Queries

**Symptom:**
```
API response time >1 second
Database CPU >80%
```

**Diagnosis:**
```bash
# 1. Enable slow query log (Supabase)
# Dashboard → Settings → Database → Query Performance

# 2. Check query plans
psql "postgresql://..." -c "EXPLAIN ANALYZE SELECT * FROM users WHERE email = 'test@example.com';"

# 3. Check missing indexes
psql "postgresql://..." -c "SELECT schemaname, tablename, attname, n_distinct, correlation FROM pg_stats WHERE tablename = 'users';"
```

**Solutions:**
```bash
# Solution 1: Add indexes
# Create migration:
diesel migration generate add_indexes

# migrations/YYYY-MM-DD-HHMMSS_add_indexes/up.sql
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at DESC);

# Run migration:
diesel migration run

# Solution 2: Optimize query
# Before: SELECT * FROM users;  -- Fetches all columns
# After:  SELECT id, email FROM users;  -- Fetches only needed columns

# Solution 3: Add connection pooling
# backend/src/db.rs
r2d2::Pool::builder()
    .max_size(20)  # Limit concurrent connections
    .connection_timeout(Duration::from_secs(5))
    .build(manager)
```

---

## 10. QUICK REFERENCE CARDS

### Deployment Commands

```bash
# BACKEND
cd backend
fly deploy                    # Deploy to production
fly logs                      # View logs
fly secrets set KEY=value     # Set environment variable
fly ssh console               # SSH into VM
fly scale count 3             # Scale to 3 VMs
fly scale memory 512          # Increase RAM to 512MB

# MOBILE
cd mobile
npm start                     # Start development
npm run ios                   # Run on iOS simulator
npm run android               # Run on Android emulator
eas build --platform ios      # Build for App Store
eas build --platform android  # Build for Play Store
eas submit --platform ios     # Submit to App Store
eas submit --platform android # Submit to Play Store

# DATABASE
diesel migration generate name  # Create migration
diesel migration run            # Run migrations
diesel migration revert         # Rollback last migration
pg_dump > backup.sql            # Backup database
psql < backup.sql               # Restore database
```

### Environment Variables Checklist

```bash
# BACKEND (.env)
DATABASE_URL=postgresql://...
JWT_SECRET=$(openssl rand -hex 32)
RUST_LOG=info
SENTRY_DSN=https://...
R2_ACCESS_KEY_ID=...
R2_SECRET_ACCESS_KEY=...
R2_ACCOUNT_ID=...
R2_BUCKET=uploads
RESEND_API_KEY=...
ENVIRONMENT=production

# MOBILE (.env)
EXPO_PUBLIC_API_URL=https://api.yourapp.com
EXPO_PUBLIC_SENTRY_DSN=https://...
EXPO_PUBLIC_ENVIRONMENT=production
```

### Health Check URLs

```bash
# Backend
https://your-app.fly.dev/health/live      # Process alive?
https://your-app.fly.dev/health/ready     # Ready for traffic?
https://your-app.fly.dev/api/v1/health    # Full health check

# Mobile (after build)
# Open app → Should connect to backend
# Check Sentry dashboard → No errors
# Check Firebase Console → Events tracked
```

---

## CONCLUSION

This infrastructure stack provides:

✅ **Free Start** — $0.75/mo (just domain) for first 1K users  
✅ **Smooth Scaling** — Clear upgrade paths at every tier  
✅ **No Lock-In** — Standard protocols (PostgreSQL, S3, Docker)  
✅ **No Rewrites** — Infrastructure abstraction prevents code changes  
✅ **Production-Ready** — HTTPS, monitoring, backups, auto-scaling  

**Your app can grow from 0 to 1M+ users on this foundation with zero architectural rewrites.**

**Next Steps:**
1. Follow Week 1 setup guide (database + backend)
2. Deploy to production
3. Monitor metrics weekly
4. Scale based on data, not fear
5. Build something amazing 🚀

---

**Questions? Issues? Contributions?**
- Open an issue on GitHub
- Check troubleshooting section
- Review service documentation links

**Last Updated:** January 10, 2026  
**Template Version:** 1.0.0  
**Maintainer:** APP_VIRGIN Team
