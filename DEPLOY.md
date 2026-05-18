# 🚀 ScrapeFlux Deployment Guide

## Quick Deploy Options

### 1. Docker (Easiest)
```bash
# Pull and run
docker pull antorroybabu/scrapeflux:latest
docker run -d -p 8000:8000 \
  -e MULTI_IP_ENABLED=true \
  -e NUM_IPS=10 \
  -v ./data:/data \
  antorroybabu/scrapeflux:latest

# Access at http://localhost:8000
```

### 2. Docker Compose (Full Stack)
```bash
git clone https://github.com/antorroybabu/ScrapeFlux.git
cd ScrapeFlux/deploy
docker-compose up -d

# Services:
# - API: http://localhost:8000
# - Metrics: http://localhost:8080
# - RabbitMQ: http://localhost:15672
# - Grafana: http://localhost:3000
```

### 3. Build from Source
```bash
# Clone
git clone https://github.com/antorroybabu/ScrapeFlux.git
cd ScrapeFlux

# Build Rust
cargo build --release

# Build Python
pip install -e python/

# Run
python -m scrapeflux serve --host 0.0.0.0 --port 8000
```

---

## Cloud Platforms

### AWS EC2
```bash
# Launch instance
aws ec2 run-instances \
  --image-id ami-0c55b159cbfafe1f0 \
  --instance-type t3.large \
  --key-name my-key

# SSH and install
ssh ubuntu@ec2-ip
git clone https://github.com/antorroybabu/ScrapeFlux.git
cd ScrapeFlux
docker-compose up -d
```

### Google Cloud Run
```bash
# Build
gcloud builds submit --tag gcr.io/PROJECT/scrapeflux

# Deploy
gcloud run deploy scrapeflux \
  --image gcr.io/PROJECT/scrapeflux \
  --platform managed \
  --region us-central1
```

### Railway/Render
```bash
# 1. Go to railway.app or render.com
# 2. Connect GitHub repo: https://github.com/antorroybabu/ScrapeFlux
# 3. Railway auto-detects Docker
# 4. Deploy!
```

### Heroku
```bash
# Install Heroku CLI
heroku login
heroku create my-scrapeflux
git push heroku main
```

---

## VPS/Server Deployment

### Ubuntu/Debian
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER

# Install ScrapeFlux
git clone https://github.com/antorroybabu/ScrapeFlux.git
cd ScrapeFlux
docker-compose up -d

# Check status
docker-compose ps
docker-compose logs -f
```

### Configure Nginx
```nginx
# /etc/nginx/sites-available/scrapeflux
server {
    listen 80;
    server_name yourdomain.com;

    location / {
        proxy_pass http://localhost:8000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

```bash
sudo ln -s /etc/nginx/sites-available/scrapeflux /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## Environment Variables

Create `.env` file:
```bash
# API Keys
SCRAPEFLUX_API_KEY=your-secret-key
OPENAI_API_KEY=sk-xxx

# Proxy (for multi-IP)
MULTI_IP_ENABLED=true
NUM_IPS=10
RESIDENTIAL_PROXY_KEY=your-proxy-key

# Database
REDIS_URL=redis://localhost:6379
DATABASE_URL=postgresql://user:pass@host:5432/scrapeflux

# Security
SECRET_KEY=your-secret-key
ALLOWED_HOSTS=yourdomain.com
```

---

## HTTPS/SSL Setup

### Let's Encrypt (Free)
```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d yourdomain.com
```

### Reverse Proxy with Cloudflare
```bash
# Point Cloudflare to your server IP
# Enable "Flexible" or "Full" SSL in Cloudflare
# No certbot needed
```

---

## Monitoring

### Check Logs
```bash
docker-compose logs -f scrapeflux
```

### Check Metrics
```bash
# Prometheus metrics
curl http://localhost:8080/metrics

# Health check
curl http://localhost:8000/health
```

### Grafana Dashboard
```
URL: http://localhost:3000
Username: admin
Password: scrape123
```

---

## Scaling

### Horizontal Scaling
```bash
# Increase worker replicas
docker-compose up -d --scale scrapeflux-worker=5
```

### Kubernetes
```bash
kubectl apply -f deploy/kubernetes/
kubectl scale deployment scrapeflux --replicas=10
```

---

## Troubleshooting

### Container won't start?
```bash
docker-compose logs scrapeflux
docker-compose restart
```

### Port already in use?
```bash
# Kill process on port 8000
sudo lsof -i :8000 | kill -9
```

### Out of memory?
```bash
# Increase Docker memory
# Or reduce worker count
docker-compose up -d --scale scrapeflux-worker=1
```

---

## Support

- **GitHub Issues:** https://github.com/antorroybabu/ScrapeFlux/issues
- **Email:** antorroybabu@gmail.com
- **Author:** Antor Roy

---

**Built with ❤️ by [Antor Roy](https://github.com/antorroybabu)**