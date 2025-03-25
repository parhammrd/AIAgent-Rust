# 🧠 AI Agent System 🚀

A modular AI-powered system with:
	•	Rust Backend → Handles AI requests, caching, and database logging.
	•	Rust Frontend (WASM) → Interactive UI for AI interactions.
	•	Python LLM Service (FastAPI + LLM models) → Serves language model responses.

## 🚀 Setup & Run

### 1️⃣ Clone the Repository

### 2️⃣ Setup Environment Variables

### 3️⃣ Start Services

* Run DeepSeek LLM (Python)

* Run MongoDB

* Run Rust Backend

        cd backend
        cargo run

## 📡 LLM API Endpoints

| Method | Endpoint | Description |
| -------|----------|-------------|
| GET | /health | Check if the backend is running. |
| POST | /generate | Generate AI text (prompt, max_length). |

Example Request:

    curl -X GET http://localhost:8080/health

    curl -X POST "http://localhost:8000/generate" \
        -H "Content-Type: application/json" \
        -d '{"prompt": "Hello AI!", "max_length": 100}'
