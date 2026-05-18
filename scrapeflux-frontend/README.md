# ScrapeFlux SaaS Platform

## 🚀 Full-Stack Web Scraping SaaS

**Frontend:** Vue.js 3 + TypeScript + TailwindCSS
**Backend:** Fastify (Node.js) + TypeScript
**Database:** PostgreSQL + Redis
**Infrastructure:** Docker Ready

---

## 📁 Project Structure

```
scrape-flux-saas/
├── frontend/                    # Vue.js + TypeScript
│   ├── src/
│   │   ├── components/
│   │   │   ├── auth/          # Login, Register, ForgotPassword
│   │   │   ├── dashboard/     # User Dashboard
│   │   │   ├── admin/         # Admin Dashboard
│   │   │   ├── scraping/      # Data Collection Tools
│   │   │   ├── email/         # Email Automation
│   │   │   └── common/        # Shared Components
│   │   ├── views/
│   │   │   ├── auth/
│   │   │   │   ├── LoginView.vue
│   │   │   │   ├── RegisterView.vue
│   │   │   │   └── ForgotPasswordView.vue
│   │   │   ├── user/
│   │   │   │   ├── DashboardView.vue
│   │   │   │   ├── ScrapingView.vue
│   │   │   │   ├── ContactsView.vue
│   │   │   │   ├── EmailView.vue
│   │   │   │   ├── SubscriptionView.vue
│   │   │   │   └── SettingsView.vue
│   │   │   └── admin/
│   │   │       ├── AdminDashboard.vue
│   │   │       ├── UsersView.vue
│   │   │       ├── SubscriptionsView.vue
│   │   │       ├── ContactsView.vue
│   │   │       ├── EmailsView.vue
│   │   │       └── SettingsView.vue
│   │   ├── stores/            # Pinia stores
│   │   ├── api/               # API client
│   │   ├── types/             # TypeScript types
│   │   └── router/            # Vue Router
│   ├── package.json
│   └── vite.config.ts
│
├── backend/                    # Fastify + TypeScript
│   ├── src/
│   │   ├── routes/           # API endpoints
│   │   ├── controllers/      # Business logic
│   │   ├── services/         # Core services
│   │   ├── models/           # Database models
│   │   ├── middleware/       # Auth, validation
│   │   └── utils/            # Helpers
│   ├── package.json
│   └── tsconfig.json
│
├── docker-compose.yml
├── README.md
└── LICENSE
```

---

## 🎯 Features

### User Features
| Feature | Description |
|---------|-------------|
| **Registration/Login** | Email + Password authentication |
| **Subscription Plans** | Free, Starter, Professional, Enterprise |
| **Data Collection** | Google, Social Media, Websites |
| **Contact Management** | Add, edit, enrich, tag contacts |
| **Email Automation** | AI-powered personalized emails |
| **Export** | CSV, JSON, Excel export |

### Admin Features
| Feature | Description |
|---------|-------------|
| **Dashboard** | Stats, charts, analytics |
| **User Management** | View, suspend, delete users |
| **Subscription Control** | Upgrade/downgrade plans |
| **Data Monitoring** | Monitor all collections |
| **Email Monitoring** | Track all sent emails |
| **System Settings** | Configure scraping limits |

---

## 🖥️ Screenshots

### User Dashboard
```
┌─────────────────────────────────────────────────────────────┐
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│  │ Contacts │  │ Emails   │  │ Scrapes  │  │ Quota    │    │
│  │   1,234  │  │   567    │  │  8,901   │  │  75%     │    │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘    │
│                                                             │
│  [📊 Recent Activity]        [📈 Usage Charts]              │
│  ─────────────────         ──────────────────               │
│  • Scraped 50 Google       ████████░░░░ 80%                │
│  • Collected 20 Twitter    ██████░░░░░░░ 60%               │
│  • Sent 15 emails          ██████████░░░ 95%                │
└─────────────────────────────────────────────────────────────┘
```

### Admin Dashboard
```
┌─────────────────────────────────────────────────────────────┐
│  📊 System Overview                                          │
│  ─────────────────────────────────────────────────────────  │
│  Users: 1,234  |  Active: 1,100  |  Revenue: $45,000       │
│  Contacts: 50,000  |  Emails Sent: 120,000                 │
│                                                             │
│  [👥 Users]  [💰 Subscriptions]  [📧 Emails]  [⚙️ Settings] │
│                                                             │
│  Recent Signups:                                            │
│  ─────────────────────────────────────────────────────────  │
│  user1@email.com  →  Starter  →  2024-01-15                │
│  user2@email.com  →  Pro     →  2024-01-14                │
│  user3@email.com  →  Free    →  2024-01-13                │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 Quick Start

### Prerequisites
- Node.js 18+
- PostgreSQL 15+
- Redis 7+

### Backend Setup
```bash
cd backend
npm install
npm run dev
```

### Frontend Setup
```bash
cd frontend
npm install
npm run dev
```

### Docker Setup
```bash
docker-compose up -d
```

---

## 📡 API Endpoints

### Authentication
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/register` | User registration |
| POST | `/api/auth/login` | User login |
| POST | `/api/auth/logout` | User logout |
| POST | `/api/auth/forgot-password` | Password reset |

### Scraping
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/scraping/google` | Google search scrape |
| POST | `/api/scraping/maps` | Google Maps scrape |
| POST | `/api/scraping/twitter` | Twitter scrape |
| POST | `/api/scraping/linkedin` | LinkedIn scrape |
| POST | `/api/scraping/website` | Custom website scrape |

### Contacts
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/contacts` | List contacts |
| POST | `/api/contacts` | Add contact |
| PUT | `/api/contacts/:id` | Update contact |
| DELETE | `/api/contacts/:id` | Delete contact |
| POST | `/api/contacts/enrich` | Enrich contact |

### Emails
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/emails` | List email jobs |
| POST | `/api/emails/generate` | AI generate emails |
| POST | `/api/emails/send` | Send emails |
| GET | `/api/emails/stats` | Email statistics |

### Admin
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/admin/users` | List all users |
| PUT | `/api/admin/users/:id` | Update user |
| DELETE | `/api/admin/users/:id` | Delete user |
| GET | `/api/admin/stats` | System statistics |

---

## 👤 Author

**Antor Roy**
- GitHub: https://github.com/antorroybabu
- Email: antorroybabu@gmail.com

---

**Built with ❤️ by [Antor Roy](https://github.com/antorroybabu)**