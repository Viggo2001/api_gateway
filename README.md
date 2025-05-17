# 📚 Gateway Project Documentation

This project is an **API gateway** written in Rust using the [`axum`](https://docs.rs/axum/latest/axum/) framework. It handles request routing, authorization, rate limiting, and IP access control before forwarding requests to feature-specific service routes.

---

## 🗂️ Folder & File Structure

```
src/
│
├── app.rs                     # Main application runner and router setup
├── main.rs                    # Entry point that calls `app::run()`
│
├── routes/                    # All feature-based route modules
│   ├── bookings.rs
│   ├── feedback.rs
│   ├── profile_management.rs
│   ├── session_management.rs
│   └── social.rs
│
├── middleware/                # Middleware components for request handling
│   ├── auth.rs                # Authorization logic using JWT
│   ├── rate_limiter.rs        # Rate limiting based on IP
│   └── allow_deny.rs          # Blocks requests from denied IPs
│
└── utils/                     # Utility files (optional folder for helpers)
```

---

## 🚀 How It Works

### ✅ `main.rs`
The main entry point. It simply calls the `run()` function in `app.rs`:

```rust
#[tokio::main]
async fn main() {
    gateway::app::run().await;
}
```

### ✅ `app.rs`
Defines the core gateway router and layers the middleware:

```rust
Router::new()
    .nest("/bookings", routes::bookings::router())
    .nest("/feedback", routes::feedback::router())
    ...
    .layer(from_fn(rate_limit))
    .layer(from_fn(auth))
    .layer(from_fn(allow_deny));
```

- `.nest(...)`: Groups endpoints under specific base paths.
- `.layer(...)`: Applies middleware globally to all requests.

---

## 🔐 Middleware Explanation

1. **`auth.rs`**  
   - Middleware that checks for a valid JWT in the `Authorization` header.
   - Uses the `jsonwebtoken` crate.
   - Responds with `401 Unauthorized` if the token is invalid or missing.

2. **`rate_limiter.rs`**  
   - Middleware that rate-limits requests based on IP address.
   - Currently allows 1 request per second per IP.
   - Responds with `429 Too Many Requests` if exceeded.

3. **`allow_deny.rs`**  
   - Blocks requests from specific denied IP addresses.
   - Uses a hardcoded `HashSet` of blocked IPs.
   - Responds with `403 Forbidden` if the IP is blocked.

---

## 🔀 Routes

Each module inside `routes/` handles a group of related endpoints. Example from `bookings.rs`:

```rust
pub fn router() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/test", get(test))
}
```

Access via:  
- `GET http://localhost:3000/bookings/hello`  
- `GET http://localhost:3000/bookings/test`

All routes are protected by middleware.

---

## 🔑 Authentication

To access protected routes, you need a valid JWT token.

### Sample login endpoint (not shown in earlier structure):
```rust
POST /login
{
  "username": "admin",
  "password": "admin123"
}
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5..."
}
```

Use the token like this in headers:

```
Authorization: Bearer <token>
```

---

## 🛠 Dependencies

- `axum` – web framework
- `hyper` – HTTP server backend
- `tokio` – async runtime
- `serde`, `serde_json` – for serialization/deserialization
- `jsonwebtoken` – for JWT auth
- `dashmap` & `once_cell` – for thread-safe global state (rate limiter)

---

## ✅ Setup Instructions

1. Clone the repo:
   ```sh
   git clone https://github.com/your-username/gateway-project.git
   cd gateway-project
   ```

2. Run it:
   ```sh
   cargo run
   ```

3. API will be live at:  
   `http://127.0.0.1:3000`

---

## 📌 Notes

- Middleware is applied **globally**, so every request passes through rate-limiting, IP blocking, and auth.
- You can add unprotected endpoints (like `/login`) before applying the `auth` middleware in `app.rs` if needed.

---

## 🧠 Tips

- To avoid applying middleware to public routes like `/login`, create a separate `Router` without `auth` applied and merge it before layering middleware.
- Adjust rate limits or denied IPs by modifying `rate_limiter.rs` and `allow_deny.rs`.