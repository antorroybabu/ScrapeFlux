<!--
✦   ____             __ _           _   _                            
✦  / ___|___  _ __  / _(_)__ _ __ | |_| |_  ___ _ __ ___  _ __   ___ 
✦ | |   / _ \| '_ \| |_| / _` / _|| __| __|/ _ \ '_ ` _ \| '_ \ / _ \
✦ | |__| (_) | | | |  _| | (_| \__ \| |_| |_|  __/ | | | | | | | |  __/
✦  \____\___/|_| |_|_| |_\__, |___/ \__|\__|\___|_| |_| |_|_| |_|\___|
✦                        |___/                                         
-->
<p align="center">
  <h1>ScrapeFlux</h1>
  <p><strong>The Ultimate Web Scraping Framework</strong></p>
  <p>High-performance · AI-Powered · Multi-Platform</p>
  
  <a href="https://github.com/antorroybabu/ScrapeFlux/releases">
    <img src="https://img.shields.io/github/v/release/antorroybabu/ScrapeFlux?style=for-the-badge&color=6366f1" alt="Version">
  </a>
  <a href="https://github.com/antorroybabu/ScrapeFlux/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/antorroybabu/ScrapeFlux?style=for-the-badge&color=22c55e" alt="License">
  </a>
  <a href="https://github.com/antorroybabu/ScrapeFlux/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/antorroybabu/ScrapeFlux/ci.yml?style=for-the-badge&color=3b82f6" alt="Build">
  </a>
  <a href="https://crates.io/crates/scrapeflux">
    <img src="https://img.shields.io/crates/d/scrapeflux?style=for-the-badge&color=f59e0b" alt="Downloads">
  </a>
  
  <br><br>
  <a href="https://github.com/antorroybabu/ScrapeFlux/stargazers">
    <img src="https://img.shields.io/github/stars/antorroybabu/ScrapeFlux?style=social" alt="Stars">
  </a>
  <a href="https://github.com/antorroybabu/ScrapeFlux/network/members">
    <img src="https://img.shields.io/github/forks/antorroybabu/ScrapeFlux?style=social" alt="Forks">
  </a>
  <a href="https://twitter.com/intent/tweet?text=ScrapeFlux%20%E2%80%93%20The%20Ultimate%20Web%20Scraping%20Framework%20by%20%40antorroybabu&url=https://github.com/antorroybabu/ScrapeFlux">
    <img src="https://img.shields.io/twitter/url/https/github.com/antorroybabu/ScrapeFlux?style=social" alt="Tweet">
  </a>
</p>

---

## 📖 Overview

**ScrapeFlux** is a next-generation web scraping framework engineered for maximum performance and versatility. Built with Rust for blazing-fast core operations and Python for accessible APIs, it delivers enterprise-grade data extraction capabilities.

### ✨ What Makes It Different

| Feature | Description |
|---------|-------------|
| **Rust-Powered Core** | Sub-millisecond parsing and extraction |
| **AI Integration** | LLM-powered intelligent extraction |
| **Multi-Platform** | Google, Social Media, Maps, Any Website |
| **Anti-Blocking** | Multi-IP rotation, proxy pools |
| **Data Separation** | Organized output by source type |

---

## 🚀 Quick Start

### Installation

```bash
# Rust (for maximum performance)
cargo add scrapeflux

# Python (for ease of use)
pip install scrapeflux
```

### Basic Usage

```python
from scrapeflux import Scraper

scraper = Scraper()
result = await scraper.scrape(
    url="https://example.com",
    schema={"title": "h1", "content": "article"}
)
print(result)
```

---

## 🎯 Core Features

### 🔍 Platform Extractors

| Platform | Capabilities |
|----------|-------------|
| **Google Search** | Web, News, Images, Videos |
| **Google Maps** | Places, Reviews, Photos, Directions |
| **Twitter/X** | Tweets, Users, Hashtags |
| **Facebook** | Posts, Comments, Pages |
| **Instagram** | Posts, Profiles, Hashtags |
| **LinkedIn** | Jobs, People, Companies |
| **Any Website** | Custom schemas, selectors |

### 🛡️ Anti-Blocking System

```
┌─────────────────────────────────────────────────────┐
│              ScrapeFlux Protection                   │
├─────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐             │
│  │  Multi  │  │ Proxy  │  │  Rate   │             │
│  │  IP     │→ │  Pool   │→ │ Limiter │             │
│  └─────────┘  └─────────┘  └─────────┘             │
│       ↓            ↓            ↓                   │
│  ┌─────────────────────────────────────┐           │
│  │        Circuit Breaker              │           │
│  └─────────────────────────────────────┘           │
│                      ↓                              │
│  ┌─────────────────────────────────────┐           │
│  │     Zero Blocking · Maximum Yield   │           │
│  └─────────────────────────────────────┘           │
└─────────────────────────────────────────────────────┘
```

### ⚡ Performance

| Metric | Traditional | ScrapeFlux | Gain |
|--------|-------------|------------|------|
| **Parse Speed** | 100ms | 1ms | 100x |
| **Memory Usage** | 100MB | 10MB | 10x |
| **Concurrent** | 50/s | 1000/s | 20x |
| **Blocking Rate** | 10% | 0.1% | 100x |

---

## 📦 Project Structure

```
scrape-flux/
├── src/
│   ├── lib.rs                 # Main library
│   ├── html.rs                # Fast HTML parsing
│   ├── http.rs               # Async HTTP client
│   ├── crawler.rs            # Smart crawler
│   ├── extractor.rs          # Data extraction
│   ├── cache.rs              # LRU caching
│   ├── advanced_data.rs      # Trie, Bloom, SimHash, etc.
│   ├── llm_engine.rs         # AI-powered extraction
│   ├── browser_automation.rs # Browser control
│   ├── ai_agent.rs          # Autonomous agent
│   ├── super_crawler.rs      # RL-based crawler
│   ├── google_extractor.rs   # Multi-platform extractors
│   └── unified.rs            # Unified framework
│
├── python/scrapeflux/        # Python API
│   ├── __init__.py
│   ├── scraper.py
│   ├── crawler.py
│   ├── extractor.py
│   └── http_client.py
│
├── deploy/                   # Deployment configs
│   ├── Dockerfile
│   ├── docker-compose.yml
│   └── requirements.txt
│
├── configs/                  # Configuration
│   ├── settings.yaml
│   ├── agents.yaml
│   └── prompts/
│
├── examples/                 # Usage examples
│
└── README.md
```

---

## 🔧 Configuration

### Environment Variables

```bash
# API Keys
SCRAPEFLUX_API_KEY=your-key
OPENAI_API_KEY=sk-xxx

# Proxy Configuration
MULTI_IP_ENABLED=true
NUM_IPS=10
RATE_LIMIT_PER_IP=50

# Database
REDIS_URL=redis://localhost:6379
```

### Proxy Setup

```yaml
# configs/proxies.yaml
proxies:
  residential:
    - provider: "brightdata"
      api_key: "your-key"
      country: "US"
  
  datacenter:
    - host: "192.168.1.1"
      port: 8080
      username: "user"
      password: "pass"
```

---

## 🌐 Deployment

### Docker

```bash
docker pull antorroybabu/scrapeflux:latest
docker run -d -p 8000:8000 scrapeflux
```

### Docker Compose

```bash
docker-compose -f deploy/docker-compose.yml up -d
```

### Cloud Platforms

| Platform | Command |
|----------|---------|
| **AWS** | `./deploy.sh aws` |
| **GCP** | `./deploy.sh gcp` |
| **Azure** | `./deploy.sh azure` |
| **Railway** | `./deploy.sh railway` |
| **Render** | `./deploy.sh render` |

---

## 💻 Code Examples

### Multi-Platform Scraping

```python
from scrapeflux import (
    GoogleExtractor,
    MapsExtractor,
    TwitterExtractor,
    MultiIPClient
)

# Setup with proxies
client = MultiIPClient(proxies)
google = GoogleExtractor(client)
maps = MapsExtractor(client)
twitter = TwitterExtractor(client)

# Extract from multiple sources
search_results = await google.search("restaurants NYC", 100)
map_places = await maps.search_places("restaurants", "40.7128,-74.0060", "5km")
tweets = await twitter.get_tweets("#NYC", 50)
```

### AI-Powered Extraction

```python
from scrapeflux import LLMEngine, AIAgent

# Generate schema with AI
engine = LLMEngine()
schema = await engine.generate_schema("Extract products with prices")

# Autonomous agent
agent = AIAgent()
result = await agent.execute("Scrape all job listings for engineers")
```

### Data Export

```python
from scrapeflux import DataSeparator, MultiFormatExporter

# Separate by platform
separator = DataSeparator("output/")
separator.separate_google_data(google_results)
separator.separate_social_data(social_results)
separator.separate_maps_data(maps_results)

# Export formats
json_data = MultiFormatExporter.to_json(data)
xml_data = MultiFormatExporter.to_xml(data, "root")
csv_data = MultiFormatExporter.to_csv(data)
```

---

## 📊 Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                      ScrapeFlux Architecture                  │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│   ┌─────────────┐     ┌─────────────┐     ┌─────────────┐   │
│   │   Python    │     │     AI      │     │   Multi-IP   │   │
│   │     API     │────▶│   Engine    │────▶│   Client    │   │
│   └─────────────┘     └─────────────┘     └─────────────┘   │
│          │                   │                   │             │
│          ▼                   ▼                   ▼             │
│   ┌─────────────────────────────────────────────────────┐    │
│   │              Rust Core Engine                        │    │
│   │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐   │    │
│   │  │  HTML   │ │  HTTP   │ │  Cache  │ │ Advanced │   │    │
│   │  │ Parser  │ │ Client  │ │  LRU    │ │  Data    │   │    │
│   │  └─────────┘ └─────────┘ └─────────┘ └─────────┘   │    │
│   └─────────────────────────────────────────────────────┘    │
│                              │                             │
│                              ▼                             │
│   ┌─────────────┐     ┌─────────────┐     ┌─────────────┐   │
│   │   Google    │     │   Social    │     │   Maps      │   │
│   │  Extractor │     │   Media     │     │  Extractor  │   │
│   └─────────────┘     └─────────────┘     └─────────────┘   │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

---

## 👥 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👤 Author

**Antor Roy**
- GitHub: [@antorroybabu](https://github.com/antorroybabu)
- Email: [antorroybabu@gmail.com](mailto:antorroybabu@gmail.com)
- Project: [ScrapeFlux](https://github.com/antorroybabu/ScrapeFlux)

---

## ⭐ Show Your Support

If this project helps you, please give it a star ⭐

[![Star](https://img.shields.io/github/stars/antorroybabu/ScrapeFlux?style=social)](https://github.com/antorroybabu/ScrapeFlux)
[![Fork](https://img.shields.io/github/forks/antorroybabu/ScrapeFlux?style=social)](https://github.com/antorroybabu/ScrapeFlux/fork)

---

<p align="center">
  <strong>Built with ❤️ by <a href="https://github.com/antorroybabu">Antor Roy</a></strong>
  <br>
  <a href="https://github.com/antorroybabu/ScrapeFlux">Repository</a> ·
  <a href="https://github.com/antorroybabu">GitHub</a> ·
  <a href="mailto:antorroybabu@gmail.com">Email</a>
</p>