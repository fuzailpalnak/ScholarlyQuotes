# ScholarlyQuotes 📚

A curated collection of thought-provoking quotes from scholars, philosophers, and intellectuals, now backed by a **PostgreSQL database** for enhanced data management and scalability.

## 🚀 Features

- **PostgreSQL Integration**: Store and retrieve quotes efficiently using a robust relational database.
- **REST API**: Fetch quotes programmatically with a structured API.
- **API Key Verification**: Supports API key-based authentication for secure access.
- **Rate Limiting**: Prevents excessive requests to ensure fair usage.
- **Redis Caching**: Provides a cached "Quote of the Day" for faster retrieval and reduced database load.

## 🏗️ Tech Stack

- **Backend**: Rust (with Actix-Web)
- **Database**: PostgreSQL
- **Cache**: Redis
- **ORM**: Sea-ORM
- **Deployment**: Docker



## 🔑 Authentication & Rate Limiting
- Requests require an API key for access.
- Each request is validated using API key verification.
- Rate limits are enforced to prevent excessive API calls.

## ⚡ Redis Caching
- The **Quote of the Day** is stored in Redis to enable fast retrieval.


## 🛠️ Contributing

1. **Fork** the repository.
2. **Create** a new branch (`feature/awesome-feature`).
3. **Commit** your changes (`git commit -m 'Add an awesome feature'`).
4. **Push** to your branch (`git push origin feature/awesome-feature`).
5. Open a **Pull Request**.


🔥 *ScholarlyQuotes – because great ideas deserve to be remembered!*

