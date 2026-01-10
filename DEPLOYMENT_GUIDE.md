# 🚀 Deployment Guide - APP_VIRGIN Template

**Last Updated:** January 10, 2026  
**Purpose:** Complete deployment strategy for production environments

---

## 📋 Table of Contents

1. [Deployment Overview](#deployment-overview)
2. [Docker Deployment](#docker-deployment)
3. [Cloud Platform Deployment](#cloud-platform-deployment)
4. [Environment Configuration](#environment-configuration)
5. [CI/CD Pipeline Setup](#cicd-pipeline-setup)
6. [Database Migration](#database-migration)
7. [Monitoring & Logging](#monitoring--logging)
8. [Security Hardening](#security-hardening)
9. [Rollback Procedures](#rollback-procedures)
10. [Troubleshooting](#troubleshooting)

---

## 🎯 Deployment Overview

### Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Load Balancer │────│   Web Server    │────│   Database      │
│   (Nginx/ALB)   │    │   (Rust/Axum)   │    │  (PostgreSQL)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │   Static Assets  │
                    │   (CDN/S3)       │
                    └─────────────────┘
```

### Deployment Targets

| Platform | Backend | Mobile Web | Mobile Native |
|----------|---------|------------|---------------|
| Docker | ✅ | ✅ | ❌ |
| AWS | ✅ | ✅ | ✅ |
| Vercel | ❌ | ✅ | ❌ |
| Railway | ✅ | ✅ | ❌ |
| DigitalOcean | ✅ | ✅ | ❌ |

---

## 🐳 Docker Deployment

### Backend Dockerfile

```dockerfile
# File: backend/Dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/backend .

# Change ownership
RUN chown appuser:appuser backend

USER appuser

# Expose port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health/live || exit 1

# Run the application
CMD ["./backend"]
```

### Mobile Web Dockerfile

```dockerfile
# File: mobile/Dockerfile
FROM node:18-alpine as builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy source code
COPY . .

# Build the web app
RUN npm run build

# Production stage
FROM nginx:alpine

# Copy built app
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Expose port
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
```

### Docker Compose

```yaml
# File: docker-compose.yml
version: '3.8'

services:
  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    ports:
      - "8000:8000"
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/app_virgin
      - JWT_SECRET=${JWT_SECRET}
      - ENVIRONMENT=production
      - ALLOWED_ORIGINS=https://yourdomain.com
    depends_on:
      - db
    restart: unless-stopped

  frontend:
    build:
      context: ./mobile
      dockerfile: Dockerfile
    ports:
      - "80:80"
    depends_on:
      - backend
    restart: unless-stopped

  db:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=app_virgin
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./backend/migrations:/docker-entrypoint-initdb.d
    ports:
      - "5432:5432"
    restart: unless-stopped

volumes:
  postgres_data:
```

### Deployment Commands

```bash
# Build and deploy
docker-compose up -d --build

# Check status
docker-compose ps

# View logs
docker-compose logs -f backend
docker-compose logs -f frontend

# Update deployment
docker-compose pull
docker-compose up -d

# Stop deployment
docker-compose down
```

---

## ☁️ Cloud Platform Deployment

### AWS Deployment

#### 1. ECS (Elastic Container Service)

```json
// File: aws/task-definition.json
{
  "family": "app-virgin-backend",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "256",
  "memory": "512",
  "executionRoleArn": "arn:aws:iam::account:role/ecsTaskExecutionRole",
  "containerDefinitions": [
    {
      "name": "backend",
      "image": "your-account.dkr.ecr.region.amazonaws.com/app-virgin-backend:latest",
      "portMappings": [
        {
          "containerPort": 8000,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "DATABASE_URL",
          "value": "postgresql://user:pass@rds-endpoint:5432/dbname"
        },
        {
          "name": "JWT_SECRET",
          "value": "your-jwt-secret"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/app-virgin",
          "awslogs-region": "us-east-1",
          "awslogs-stream-prefix": "ecs"
        }
      },
      "healthCheck": {
        "command": ["CMD-SHELL", "curl -f http://localhost:8000/health/live || exit 1"],
        "interval": 30,
        "timeout": 5,
        "retries": 3
      }
    }
  ]
}
```

#### 2. RDS Database Setup

```bash
# Create RDS instance
aws rds create-db-instance \
    --db-instance-identifier app-virgin-db \
    --db-instance-class db.t3.micro \
    --engine postgres \
    --engine-version 15.4 \
    --master-username postgres \
    --master-user-password your-password \
    --allocated-storage 20 \
    --vpc-security-group-ids sg-xxxxxxxxx \
    --db-subnet-group-name default

# Get connection endpoint
aws rds describe-db-instances --db-instance-identifier app-virgin-db
```

#### 3. Application Load Balancer

```bash
# Create target group
aws elbv2 create-target-group \
    --name app-virgin-backend \
    --protocol HTTP \
    --port 8000 \
    --target-type ip \
    --vpc-id vpc-xxxxxxxxx

# Create load balancer
aws elbv2 create-load-balancer \
    --name app-virgin-alb \
    --subnets subnet-xxxxxxxxx subnet-yyyyyyyyy \
    --security-groups sg-xxxxxxxxx \
    --scheme internet-facing \
    --type application
```

### Vercel Deployment (Frontend Only)

```json
// File: vercel.json
{
  "version": 2,
  "builds": [
    {
      "src": "mobile/package.json",
      "use": "@vercel/static-build",
      "config": {
        "distDir": "dist"
      }
    }
  ],
  "routes": [
    {
      "src": "/api/(.*)",
      "dest": "https://your-backend.com/api/$1"
    },
    {
      "src": "/(.*)",
      "dest": "/$1"
    }
  ],
  "env": {
    "EXPO_PUBLIC_API_URL": "https://your-backend.com"
  }
}
```

```bash
# Deploy to Vercel
npm install -g vercel
cd mobile
vercel --prod
```

### Railway Deployment

```yaml
# File: railway.toml
[build]
builder = "nixpacks"

[deploy]
startCommand = "cargo run"
healthcheckPath = "/health/live"
healthcheckTimeout = 100
restartPolicyType = "on_failure"

[[services]]
name = "backend"

[services.variables]
DATABASE_URL = "${{RAILWAY_DATABASE_CONNECTION_URL}}"
JWT_SECRET = "${{JWT_SECRET}}"
ENVIRONMENT = "production"
```

```bash
# Deploy to Railway
npm install -g @railway/cli
railway login
railway init
railway up
```

---

## ⚙️ Environment Configuration

### Production Environment Variables

```bash
# Backend Configuration
DATABASE_URL=postgresql://user:password@host:5432/database
JWT_SECRET=your-super-secure-32-byte-secret-key
ENVIRONMENT=production
ALLOWED_ORIGINS=https://yourdomain.com,https://www.yourdomain.com
RUST_LOG=info

# Security Headers
SECURE_COOKIES=true
CORS_MAX_AGE=86400
RATE_LIMIT_AUTH=1
RATE_LIMIT_GENERAL=50

# Mobile Configuration
EXPO_PUBLIC_API_URL=https://api.yourdomain.com
EXPO_PUBLIC_ENVIRONMENT=production
```

### Nginx Configuration

```nginx
# File: nginx.conf
events {
    worker_connections 1024;
}

http {
    include       /etc/nginx/mime.types;
    default_type  application/octet-stream;

    # Logging
    log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                    '$status $body_bytes_sent "$http_referer" '
                    '"$http_user_agent" "$http_x_forwarded_for"';

    access_log /var/log/nginx/access.log main;
    error_log /var/log/nginx/error.log warn;

    # Security headers
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    limit_req_zone $binary_remote_addr zone=auth:10m rate=1r/s;

    server {
        listen 80;
        server_name yourdomain.com www.yourdomain.com;
        return 301 https://$server_name$request_uri;
    }

    server {
        listen 443 ssl http2;
        server_name yourdomain.com www.yourdomain.com;

        # SSL configuration
        ssl_certificate /etc/ssl/certs/yourdomain.com.crt;
        ssl_certificate_key /etc/ssl/private/yourdomain.com.key;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384;
        ssl_prefer_server_ciphers off;

        # Frontend static files
        location / {
            root /usr/share/nginx/html;
            index index.html index.htm;
            try_files $uri $uri/ /index.html;
        }

        # Backend API
        location /api/ {
            limit_req zone=api burst=20 nodelay;
            
            proxy_pass http://backend:8000;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            
            # CORS headers
            add_header Access-Control-Allow-Origin $http_origin;
            add_header Access-Control-Allow-Credentials true;
        }

        # Auth endpoints (stricter rate limiting)
        location /api/v1/auth/ {
            limit_req zone=auth burst=5 nodelay;
            
            proxy_pass http://backend:8000;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Health check (no rate limiting)
        location /health/ {
            proxy_pass http://backend:8000;
            access_log off;
        }
    }
}
```

---

## 🔄 CI/CD Pipeline Setup

### GitHub Actions Workflow

```yaml
# File: .github/workflows/deploy.yml
name: Deploy to Production

on:
  push:
    branches: [ main ]
    tags: [ 'v*' ]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '18'
        cache: 'npm'
        cache-dependency-path: mobile/package-lock.json
        
    - name: Install dependencies
      run: |
        cd mobile
        npm ci
        
    - name: Run backend tests
      run: |
        cd backend
        cargo test --verbose
      env:
        DATABASE_URL: postgresql://postgres:postgres@localhost/postgres
        
    - name: Run mobile tests
      run: |
        cd mobile
        npm test -- --coverage

  build-and-deploy:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Log in to Container Registry
      uses: docker/login-action@v3
      with:
        registry: ${{ env.REGISTRY }}
        username: ${{ github.actor }}
        password: ${{ secrets.GITHUB_TOKEN }}
        
    - name: Extract metadata
      id: meta
      uses: docker/metadata-action@v5
      with:
        images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
        tags: |
          type=ref,event=branch
          type=ref,event=pr
          type=sha,prefix={{branch}}-
          type=raw,value=latest,enable={{is_default_branch}}
          
    - name: Build and push backend image
      uses: docker/build-push-action@v5
      with:
        context: ./backend
        push: true
        tags: ${{ steps.meta.outputs.tags }}-backend
        labels: ${{ steps.meta.outputs.labels }}
        
    - name: Build and push frontend image
      uses: docker/build-push-action@v5
      with:
        context: ./mobile
        push: true
        tags: ${{ steps.meta.outputs.tags }}-frontend
        labels: ${{ steps.meta.outputs.labels }}
        
    - name: Deploy to production
      run: |
        # Update production environment
        echo "Deploying to production..."
        # Add your deployment commands here
        # For example: kubectl apply -f k8s/
```

### Database Migration Pipeline

```yaml
# File: .github/workflows/migrate.yml
name: Database Migration

on:
  workflow_dispatch:
    inputs:
      environment:
        description: 'Environment to migrate'
        required: true
        default: 'staging'
        type: choice
        options:
        - staging
        - production

jobs:
  migrate:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Diesel CLI
      run: |
        cargo install diesel_cli --no-default-features --features postgres
        
    - name: Run migrations
      run: |
        cd backend
        diesel migration run --database-url ${{ secrets.DATABASE_URL }}
      env:
        DATABASE_URL: ${{ secrets.DATABASE_URL }}
```

---

## 🗄 Database Migration

### Migration Files

```sql
-- File: backend/migrations/0001_create_users_table.sql
-- Create users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);

-- File: backend/migrations/0002_create_sessions_table.sql
-- Create sessions table for JWT refresh tokens
CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_refresh_token ON sessions(refresh_token);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
```

### Migration Commands

```bash
# Create new migration
cd backend
diesel migration generate create_new_table

# Run migrations
diesel migration run

# Rollback migration
diesel migration revert

# Check migration status
diesel migration list

# Redo last migration
diesel migration redo
```

### Production Migration Strategy

```bash
# 1. Backup database before migration
pg_dump -h host -U user -d database > backup_$(date +%Y%m%d_%H%M%S).sql

# 2. Run migration with rollback plan
diesel migration run --database-url $DATABASE_URL

# 3. Verify migration success
diesel migration list --database-url $DATABASE_URL

# 4. Test application functionality
curl -f https://api.yourdomain.com/health/live

# 5. Monitor for errors
# Check logs and metrics
```

---

## 📊 Monitoring & Logging

### Application Monitoring

```rust
// File: backend/src/middleware/metrics.rs
use axum::middleware;
use axum::response::Response;
use std::time::Instant;
use prometheus::{Counter, Histogram, IntGauge};

lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: Counter = Counter::new(
        "http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();
    
    static ref HTTP_REQUEST_DURATION: Histogram = Histogram::with_opts(
        prometheus::HistogramOpts::new(
            "http_request_duration_seconds",
            "HTTP request duration in seconds"
        ).buckets(vec![0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0])
    ).unwrap();
    
    static ref ACTIVE_CONNECTIONS: IntGauge = IntGauge::new(
        "active_connections",
        "Number of active connections"
    ).unwrap();
}

pub async fn metrics_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    ACTIVE_CONNECTIONS.inc();
    
    let response = next.run(req).await;
    
    let duration = start.elapsed();
    HTTP_REQUESTS_TOTAL.inc();
    HTTP_REQUEST_DURATION.observe(duration.as_secs_f64());
    ACTIVE_CONNECTIONS.dec();
    
    tracing::info!(
        "Request: {} {} - Status: {} - Duration: {:?}",
        method,
        uri,
        response.status(),
        duration
    );
    
    Ok(response)
}
```

### Health Check Enhancements

```rust
// File: backend/src/api/health.rs
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
    pub database: DatabaseStatus,
    pub uptime_seconds: u64,
}

#[derive(Serialize)]
pub struct DatabaseStatus {
    pub status: String,
    pub connection_pool_size: u32,
    pub active_connections: u32,
}

pub async fn health_check() -> Result<Json<HealthResponse>, ApiError> {
    let db_status = check_database_health().await?;
    
    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        timestamp: chrono::Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
        uptime_seconds: get_uptime_seconds(),
    }))
}

async fn check_database_health() -> Result<DatabaseStatus, ApiError> {
    // Check database connectivity
    let pool = get_db_pool();
    let pool_size = pool.size() as u32;
    let active_connections = pool.num_idle() as u32;
    
    // Test query
    let _result: i64 = sql_query("SELECT 1")
        .get_one(&mut pool.get().await?)
        .await?;
    
    Ok(DatabaseStatus {
        status: "healthy".to_string(),
        connection_pool_size: pool_size,
        active_connections,
    })
}
```

### Log Aggregation

```yaml
# File: docker-compose.logging.yml
version: '3.8'

services:
  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:8.11.0
    environment:
      - discovery.type=single-node
      - xpack.security.enabled=false
    ports:
      - "9200:9200"
    volumes:
      - elasticsearch_data:/usr/share/elasticsearch/data

  kibana:
    image: docker.elastic.co/kibana/kibana:8.11.0
    ports:
      - "5601:5601"
    environment:
      - ELASTICSEARCH_HOSTS=http://elasticsearch:9200
    depends_on:
      - elasticsearch

  logstash:
    image: docker.elastic.co/logstash/logstash:8.11.0
    volumes:
      - ./logstash.conf:/usr/share/logstash/pipeline/logstash.conf
    depends_on:
      - elasticsearch

volumes:
  elasticsearch_data:
```

---

## 🔒 Security Hardening

### SSL/TLS Configuration

```bash
# Generate SSL certificate with Let's Encrypt
sudo apt install certbot python3-certbot-nginx

# Request certificate
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com

# Auto-renewal
sudo crontab -e
# Add: 0 12 * * * /usr/bin/certbot renew --quiet
```

### Firewall Configuration

```bash
# UFW firewall setup
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable
```

### Security Headers

```rust
// File: backend/src/middleware/security.rs
use axum::http::{HeaderMap, HeaderValue};
use axum::response::Response;

pub fn add_security_headers(mut response: Response) -> Response {
    let headers = response.headers_mut();
    
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff")
    );
    headers.insert(
        "X-Frame-Options",
        HeaderValue::from_static("DENY")
    );
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block")
    );
    headers.insert(
        "Strict-Transport-Security",
        HeaderValue::from_static("max-age=31536000; includeSubDomains")
    );
    headers.insert(
        "Content-Security-Policy",
        HeaderValue::from_static("default-src 'self'")
    );
    
    response
}
```

---

## 🔄 Rollback Procedures

### Blue-Green Deployment

```bash
# Deploy to green environment
docker-compose -f docker-compose.green.yml up -d

# Test green environment
curl -f http://green.yourdomain.com/health/live

# Switch traffic to green
# Update load balancer configuration

# Monitor for issues
# If issues detected, switch back to blue
```

### Database Rollback

```bash
# 1. Identify problematic migration
diesel migration list

# 2. Rollback specific migration
diesel migration revert --database-url $DATABASE_URL

# 3. Verify application works
curl -f https://api.yourdomain.com/health/live

# 4. Monitor for data consistency
```

### Quick Rollback Script

```bash
#!/bin/bash
# File: scripts/rollback.sh

set -e

ENVIRONMENT=${1:-production}
PREVIOUS_VERSION=${2:-$(git describe --tags --abbrev=0 HEAD~1)}

echo "Rolling back $ENVIRONMENT to $PREVIOUS_VERSION"

# 1. Rollback backend
docker pull your-registry/app-virgin-backend:$PREVIOUS_VERSION
docker-compose up -d --no-deps backend

# 2. Rollback frontend
docker pull your-registry/app-virgin-frontend:$PREVIOUS_VERSION
docker-compose up -d --no-deps frontend

# 3. Verify rollback
sleep 30
curl -f https://api.yourdomain.com/health/live

echo "Rollback completed successfully"
```

---

## 🐛 Troubleshooting

### Common Deployment Issues

#### Database Connection Issues

```bash
# Check database connectivity
psql -h host -U user -d database -c "SELECT 1;"

# Check connection pool status
curl -f https://api.yourdomain.com/health/live

# Review database logs
docker-compose logs db
```

#### SSL Certificate Issues

```bash
# Check certificate validity
openssl s_client -connect yourdomain.com:443 -servername yourdomain.com

# Test certificate renewal
sudo certbot renew --dry-run

# Check nginx configuration
sudo nginx -t
```

#### Performance Issues

```bash
# Check resource usage
docker stats

# Monitor response times
curl -w "@curl-format.txt" -o /dev/null -s https://api.yourdomain.com/health/live

# Analyze slow queries
# Enable slow query log in PostgreSQL
```

### Debugging Container Issues

```bash
# Enter container shell
docker exec -it container_name bash

# View container logs
docker logs container_name

# Inspect container
docker inspect container_name
```

### Health Check Failures

```bash
# Manual health check
curl -v https://api.yourdomain.com/health/live

# Check dependencies
curl -v https://api.yourdomain.com/health/ready

# Review application logs
docker-compose logs -f backend
```

---

## 📋 Deployment Checklist

### Pre-Deployment

- [ ] All tests passing
- [ ] Security scan completed
- [ ] Performance benchmarks met
- [ ] Database migrations tested
- [ ] Environment variables configured
- [ ] SSL certificates valid
- [ ] Backup procedures tested

### Post-Deployment

- [ ] Health checks passing
- [ ] Monitoring alerts configured
- [ ] Log aggregation working
- [ ] Performance metrics baseline
- [ ] Security headers verified
- [ ] Rollback plan documented

### Monitoring

- [ ] Response time < 200ms
- [ ] Error rate < 1%
- [ ] CPU usage < 80%
- [ ] Memory usage < 80%
- [ ] Database connections healthy
- [ ] SSL certificate expiry monitored

---

**Happy Deploying! 🚀**

Remember: A successful deployment is not just about getting code to production—it's about ensuring it runs reliably, securely, and performs well for your users.
