# BookStore API 👾
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Ruby Version](https://img.shields.io/badge/ruby-3.2.2-red)
![Rust Version](https://img.shields.io/badge/rust-1.75.0-orange)

A high-performance bookstore API built with Ruby on Rails and Rust. Rails handles the web interface and database operations, while Rust manages complex computations and search functionality.

## Architecture 🍯

```plaintext
/bookstore-api
├── rails-service/    # Rails API service
└── rust-service/     # Rust computational service
```

## Features 🌙

- 🔐 JWT Authentication
- 📚 Book Management
- 🔍 Advanced Search Algorithm
- 💫 Real-time Recommendations
- 📊 Rating System
- 🏷️ Category Management
- 🔄 Order Processing

## Prerequisites ⚙️

- Ruby 3.2.2
- Rust 1.75.0
- SQLite3
- Bundler
- Cargo

## Installation 🏛️

1. **Clone the repository**
```bash
git clone https://github.com/yourusername/bookstore-api.git
cd bookstore-api
```

2. **Set up Rails service**
```bash
cd rails-service
bundle install
rails db:create
rails db:migrate
rails db:seed
```

3. **Set up Rust service**
```bash
cd ../rust-service
cargo build
```

## Configuration 🔑

1. **Environment Variables**
```bash
# .env
RAILS_ENV=development
RUST_SERVICE_URL=http://localhost:7878
JWT_SECRET=your_jwt_secret
```

2. **Database Configuration**
```yaml
# config/database.yml
default: &default
  adapter: sqlite3
  pool: <%= ENV.fetch("RAILS_MAX_THREADS") { 5 } %>
  timeout: 5000
```

## Running the Services 🧲

1. **Start Rails server**
```bash
cd rails-service
rails server -p 3000
```

2. **Start Rust server**
```bash
cd rust-service
cargo run
```

## API Endpoints 📍

### Authentication
```plaintext
POST   /api/v1/authenticate    # Login
POST   /api/v1/register       # Register new user
```

### Books
```plaintext
GET    /api/v1/books          # List all books
POST   /api/v1/books          # Create a book
GET    /api/v1/books/:id      # Get a specific book
PUT    /api/v1/books/:id      # Update a book
DELETE /api/v1/books/:id      # Delete a book
```

### Search & Recommendations 🔍
```plaintext
POST   /api/v1/books/search           # Search books
GET    /api/v1/books/recommendations  # Get recommendations
```

## API Usage Examples

### Authentication 🔗
```javascript
const response = await fetch('api/v1/authenticate', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    email: 'user@example.com',
    password: 'password123'
  })
});
```

### Create Book
```javascript
const response = await fetch('api/v1/books', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    book: {
      title: 'The Rust Programming Language',
      author: 'Steve Klabnik',
      genre: 'Technical',
      price: 29.99
    }
  })
});
```

## Testing 🍴

### Rails Tests
```bash
cd rails-service
rails test
```

### Rust Tests
```bash
cd rust-service
cargo test
```

## Performance ⚓

- Rails API: Handles ~1000 requests/second
- Rust Service: Handles ~10,000 requests/second
- Search Response Time: < 100ms
- Recommendation Generation: < 200ms

## Security 🛡️

- JWT Authentication
- CORS Protection
- Request Rate Limiting
- Input Validation
- SQL Injection Prevention
- XSS Protection

## Contributing 🤝🏾

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License ⚖️

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Contact ☎️

Your Name - [@dvble.m](https://twitter.com/BookofT) 
Project Link: [https://github.com/Thewsthews/Restful](https://github.com/Thewsthews/Restful)

## Acknowledgments ✊🏾

- Ruby on Rails Team
- Rust Team
- All contributors
