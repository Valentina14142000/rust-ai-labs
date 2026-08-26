# 🦀 Rust AI Labs

### High-Performance AI Infrastructure & Agentic Systems

Rust AI Labs is a collection of production-oriented AI engineering projects built with **Rust and Python**, focused on Agentic AI, RAG, LLM inference, MCP, LLMOps, and AI infrastructure.

The goal is to explore how Rust can provide **high performance, safe concurrency, low memory overhead, and reliable infrastructure** for modern AI systems.

---

## 🚀 Projects

| Project | Description | Status |
|---|---|---|
| **01-AgentForge** | Rust-based autonomous AI agent runtime | 🚧 Active |
| **02-RustRAG** | High-performance Hybrid RAG & GraphRAG | 🔜 Planned |
| **03-InferGate** | LLM inference gateway & model router | 🔜 Planned |
| **04-MCP-Ecosystem** | Rust MCP clients & servers | 🔜 Planned |
| **05-RustInfer** | Lightweight model inference | 🔜 Planned |
| **06-DeepResearch-RS** | Autonomous multi-agent research | 🔜 Planned |
| **07-EdgeVision-RS** | Edge AI inference platform | 🔜 Planned |
| **08-TraceLLM** | LLM observability & tracing | 🔜 Planned |

---

## 🧠 Core Areas

- 🤖 Agentic AI & Multi-Agent Systems
- 🔍 RAG, Hybrid Retrieval & GraphRAG
- ⚙️ LLMOps & Model Serving
- 🦀 Rust AI Infrastructure
- 🔌 Model Context Protocol (MCP)
- 🚀 LLM Inference & Optimization
- 📊 AI Observability & Benchmarking
- 💻 Edge AI

---

## 🛠️ Technology Stack

**Rust:**  
Rust · Tokio · Axum · Reqwest · Serde · Tracing

**AI/ML:**  
Python · PyTorch · Hugging Face · Transformers · ONNX · Candle

**Agentic AI:**  
LangGraph · LangChain · LlamaIndex · MCP · Tool Calling

**RAG:**  
Vector Search · Hybrid Retrieval · GraphRAG · Knowledge Graphs · Reranking

**Inference:**  
vLLM · llama.cpp · GGUF · AWQ · GPTQ · Triton

**Infrastructure:**  
Docker · PostgreSQL · Redis · Qdrant · Neo4j · OpenTelemetry

---

## 🏗️ Architecture

```text
                  AI Applications
                         │
                         ▼
                  ┌─────────────┐
                  │ AgentForge  │
                  └──────┬──────┘
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
       RustRAG          MCP         InferGate
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                  LLM / AI Models
                         │
                         ▼
                   TraceLLM
```

---

## 🤖 AgentForge

AgentForge is the first project in the ecosystem: a Rust-based runtime for building autonomous, tool-using AI agents.

Current components:

Agent runtime
Agent state
Async execution
LLM provider abstraction
Tool abstraction
Tool registry
Calculator tool
Axum API
Structured JSON responses
Tracing

Planned:

Real LLM function calling
Dynamic tool selection
Agent memory
Multi-agent orchestration
MCP
Streaming
vLLM/Ollama integration

## ⚡ Quick Start

git clone github repository
cd rust-ai-labs/01-agentforge

cargo check
cargo run

API:

http://localhost:3000

Test:

curl http://localhost:3000/health
🗺️ Roadmap
 AgentForge foundation
 Agent executor
 Tool registry
 Calculator tool
 LLM function calling
 Agent memory
 RustRAG
 InferGate
 MCP ecosystem
 DeepResearch-RS
 Edge AI
 LLM observability
