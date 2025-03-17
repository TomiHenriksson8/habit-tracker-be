# Habit Tracker Backend (Rust)

A powerful backend service built with Rust and Axum, designed for managing user habits effectively. This API supports user registration, authentication, and CRUD operations on habits, leveraging MongoDB for data storage.

## 🚀 Technologies Used

- **Rust**
- **Axum Framework** (Routing and middleware)
- **MongoDB** (Database)
- **Serde** (Serialization and deserialization)
- **JWT** (User authentication)
- **CorsLayer & TraceLayer** (Middleware for security and debugging)
- **Dotenvy** (Environment variables)
- **Tokio** (Asynchronous runtime)

## 📁 Project Structure

```
src
├── controllers
│   └── auth_controller.rs
│   └── habit_controller.rs
├── db
│   └── mod.rs
├── models
│   ├── user.rs
│   └── habit.rs
├── repositories
│   └── user_repository.rs
├── routes
│   ├── auth.rs
│   └── habits.rs
├── utils
│   └── mod.rs
└── main.rs
```

## 🛠️ Setup and Installation

1. Clone the repository:

```bash
git clone https://github.com/TomiHenriksson8/habit-tracker-be
cd habit-tracker-be
```

2. Install dependencies:

```bash
cargo build
```

3. Create `.env` file and set environment variables:

```env
MONGO_URI=your_mongodb_connection_string
JWT_SECRET=your_jwt_secret_key
```

4. Run the development server:

```bash
cargo run
```

The server will run on `http://localhost:8000`.

## 📌 API Endpoints

### 🔑 Auth Routes

| Method | Route                | Description           |
| ------ | -------------------- | --------------------- |
| POST   | `/api/auth/register` | Register a new user   |
| POST   | `/api/auth/login`    | Authenticate user     |
| GET    | `/api/auth/me`       | Get current user data |

### 📋 Habit Routes

| Method | Route                      | Description              |
| ------ | -------------------------- | ------------------------ |
| POST   | `/api/habits/`             | Create a new habit       |
| GET    | `/api/habits/`             | List habits for the user |
| GET    | `/api/habits/:id`          | Get habit details by ID  |
| PUT    | `/api/habits/:id`          | Update a habit by ID     |
| PUT    | `/api/habits/:id/complete` | Mark habit as completed  |
| DELETE | `/api/habits/:id`          | Delete a habit by ID     |

## 🌐 Server

[https://habit-tracker-production-74a7.up.railway.app/](https://habit-tracker-production-74a7.up.railway.app/)

## 🛠 Running Locally

Start the server:

```bash
cargo run
```

The server will run at:

```
http://localhost:8000
```
