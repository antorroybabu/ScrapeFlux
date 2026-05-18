/*
 * =============================================================================
 * Module: User Authentication & Subscription System
 * Project: ScrapeFlux - Ultimate Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * GitHub: https://github.com/antorroybabu/ScrapeFlux
 * License: MIT
 * 
 * Description:
 *     User authentication, registration, login, subscription management,
 *     data collection, and email automation with AI.
 * =============================================================================
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

// ============================================================================
// USER MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub company_name: String,
    pub company_website: Option<String>,
    pub role: UserRole,
    pub status: UserStatus,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum UserRole {
    Free,
    Starter,
    Professional,
    Enterprise,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum UserStatus {
    Pending,
    Active,
    Suspended,
    Deleted,
}

impl User {
    pub fn new(email: &str, password_hash: &str, first_name: &str, last_name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            company_name: String::new(),
            company_website: None,
            role: UserRole::Free,
            status: UserStatus::Pending,
            email_verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: None,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn can_access_feature(&self, feature: &str) -> bool {
        match self.role {
            UserRole::Free => matches!(feature, "basic_scrape" | "limited_data"),
            UserRole::Starter => matches!(feature, "basic_scrape" | "limited_data" | "email_basic"),
            UserRole::Professional => true,
            UserRole::Enterprise | UserRole::Admin => true,
        }
    }
}

// ============================================================================
// SUBSCRIPTION MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub user_id: String,
    pub plan: SubscriptionPlan,
    pub status: SubscriptionStatus,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub billing_cycle: BillingCycle,
    pub next_billing_date: Option<DateTime<Utc>>,
    pub monthly_price: f64,
    pub total_paid: f64,
    pub payment_method: Option<PaymentMethod>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum SubscriptionPlan {
    Free,
    Starter,      // $29/month
    Professional, // $99/month
    Enterprise,   // $299/month
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Cancelled,
    Expired,
    Trial,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    Card { last4: String, brand: String },
    PayPal { email: String },
    BankTransfer { account: String },
}

impl Subscription {
    pub fn new(user_id: &str, plan: SubscriptionPlan) -> Self {
        let monthly_price = match plan {
            SubscriptionPlan::Free => 0.0,
            SubscriptionPlan::Starter => 29.0,
            SubscriptionPlan::Professional => 99.0,
            SubscriptionPlan::Enterprise => 299.0,
        };

        let end_date = match plan {
            SubscriptionPlan::Free => None,
            _ => Some(Utc::now() + Duration::days(30)),
        };

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            plan,
            status: SubscriptionStatus::Trial,
            start_date: Utc::now(),
            end_date,
            billing_cycle: BillingCycle::Monthly,
            next_billing_date: end_date,
            monthly_price,
            total_paid: 0.0,
            payment_method: None,
            created_at: Utc::now(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == SubscriptionStatus::Active || self.status == SubscriptionStatus::Trial
    }

    pub fn days_remaining(&self) -> i64 {
        if let Some(end) = self.end_date {
            (end - Utc::now()).num_days()
        } else {
            i64::MAX
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanFeatures {
    pub plan: SubscriptionPlan,
    pub monthly_price: f64,
    pub yearly_price: f64,
    pub scrapes_per_month: usize,
    pub emails_per_month: usize,
    pub team_members: usize,
    pub api_access: bool,
    pub multi_ip: bool,
    pub priority_support: bool,
    pub custom_integrations: bool,
    pub features: Vec<String>,
}

impl PlanFeatures {
    pub fn get_features(plan: SubscriptionPlan) -> Self {
        match plan {
            SubscriptionPlan::Free => Self {
                plan,
                monthly_price: 0.0,
                yearly_price: 0.0,
                scrapes_per_month: 100,
                emails_per_month: 10,
                team_members: 1,
                api_access: false,
                multi_ip: false,
                priority_support: false,
                custom_integrations: false,
                features: vec![
                    "Basic web scraping".to_string(),
                    "100 scrapes/month".to_string(),
                    "10 emails/month".to_string(),
                    "Community support".to_string(),
                ],
            },
            SubscriptionPlan::Starter => Self {
                plan,
                monthly_price: 29.0,
                yearly_price: 290.0,
                scrapes_per_month: 1000,
                emails_per_month: 500,
                team_members: 3,
                api_access: true,
                multi_ip: false,
                priority_support: false,
                custom_integrations: false,
                features: vec![
                    "Everything in Free".to_string(),
                    "1000 scrapes/month".to_string(),
                    "500 emails/month".to_string(),
                    "3 team members".to_string(),
                    "API access".to_string(),
                    "Email templates".to_string(),
                ],
            },
            SubscriptionPlan::Professional => Self {
                plan,
                monthly_price: 99.0,
                yearly_price: 990.0,
                scrapes_per_month: 10000,
                emails_per_month: 5000,
                team_members: 10,
                api_access: true,
                multi_ip: true,
                priority_support: true,
                custom_integrations: false,
                features: vec![
                    "Everything in Starter".to_string(),
                    "10,000 scrapes/month".to_string(),
                    "5000 emails/month".to_string(),
                    "10 team members".to_string(),
                    "Multi-IP rotation".to_string(),
                    "Priority support".to_string(),
                    "AI email writing".to_string(),
                ],
            },
            SubscriptionPlan::Enterprise => Self {
                plan,
                monthly_price: 299.0,
                yearly_price: 2990.0,
                scrapes_per_month: usize::MAX,
                emails_per_month: usize::MAX,
                team_members: usize::MAX,
                api_access: true,
                multi_ip: true,
                priority_support: true,
                custom_integrations: true,
                features: vec![
                    "Everything in Professional".to_string(),
                    "Unlimited scrapes".to_string(),
                    "Unlimited emails".to_string(),
                    "Unlimited team members".to_string(),
                    "Custom integrations".to_string(),
                    "Dedicated support".to_string(),
                    "Custom SLA".to_string(),
                ],
            },
        }
    }
}

// ============================================================================
// DATA COLLECTION MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedContact {
    pub id: String,
    pub user_id: String,
    pub person_name: String,
    pub email: String,
    pub mobile: Option<String>,
    pub company_name: String,
    pub company_website: Option<String>,
    pub position: Option<String>,
    pub linkedin_url: Option<String>,
    pub source: DataSource,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub enriched_at: Option<DateTime<Utc>>,
    pub email_sent: bool,
    pub last_email_sent: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum DataSource {
    Google,
    LinkedIn,
    CompanyWebsite,
    Manual,
    Import,
}

impl CollectedContact {
    pub fn new(user_id: &str, person_name: &str, email: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            person_name: person_name.to_string(),
            email: email.to_string(),
            mobile: None,
            company_name: String::new(),
            company_website: None,
            position: None,
            linkedin_url: None,
            source: DataSource::Manual,
            notes: None,
            tags: Vec::new(),
            created_at: Utc::now(),
            enriched_at: None,
            email_sent: false,
            last_email_sent: None,
        }
    }

    pub fn with_company(mut self, company_name: &str, website: Option<&str>) -> Self {
        self.company_name = company_name.to_string();
        self.company_website = website.map(|s| s.to_string());
        self
    }

    pub fn with_position(mut self, position: &str) -> Self {
        self.position = Some(position.to_string());
        self
    }

    pub fn with_mobile(mut self, mobile: &str) -> Self {
        self.mobile = Some(mobile.to_string());
        self
    }
}

// ============================================================================
// EMAIL AUTOMATION MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub subject_template: String,
    pub body_template: String,
    pub variables: Vec<String>,
    pub category: EmailCategory,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum EmailCategory {
    Introduction,
    FollowUp,
    Cold,
    Warm,
    ProductPitch,
    MeetingRequest,
    ThankYou,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailJob {
    pub id: String,
    pub user_id: String,
    pub contact_id: String,
    pub template_id: String,
    pub recipient_email: String,
    pub subject: String,
    pub body: String,
    pub status: EmailStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub clicked_at: Option<DateTime<Utc>>,
    pub bounced: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum EmailStatus {
    Queued,
    Sent,
    Delivered,
    Opened,
    Clicked,
    Bounced,
    Failed,
}

impl EmailJob {
    pub fn new(user_id: &str, contact_id: &str, template_id: &str, 
               recipient_email: &str, subject: &str, body: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            contact_id: contact_id.to_string(),
            template_id: template_id.to_string(),
            recipient_email: recipient_email.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
            status: EmailStatus::Queued,
            sent_at: None,
            opened_at: None,
            clicked_at: None,
            bounced: false,
            error_message: None,
        }
    }
}

// ============================================================================
// AI EMAIL WRITER
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEmailWriter {
    api_key: String,
    model: String,
    temperature: f32,
}

impl AIEmailWriter {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            model: "gpt-4".to_string(),
            temperature: 0.7,
        }
    }

    /// Generate personalized email using AI
    pub async fn generate_email(
        &self,
        contact: &CollectedContact,
        product_name: &str,
        email_type: &str,
    ) -> Result<(String, String), String> {
        let prompt = format!(
            r#"
Write a professional cold outreach email with the following details:

RECIPIENT:
- Name: {}
- Company: {}
- Position: {}
- Company Website: {}

PRODUCT/ SERVICE:
- Product: {}

EMAIL TYPE: {}

REQUIREMENTS:
1. Keep the email concise (100-150 words)
2. Make it personalized and human-sounding
3. Include a clear call-to-action
4. Match the email type style
5. Use a catchy but professional subject line
6. Do NOT use placeholders - write the actual email

Output format:
SUBJECT: [your subject line]
BODY: [your email body]
"#,
            contact.person_name,
            contact.company_name,
            contact.position.as_deref().unwrap_or("unknown position"),
            contact.company_website.as_deref().unwrap_or("N/A"),
            product_name,
            email_type
        );

        // Call OpenAI API (simplified - in production use reqwest)
        let response = self.call_llm(&prompt).await?;
        
        // Parse response
        let parts: Vec<&str> = response.split("\nBODY: ").collect();
        if parts.len() == 2 {
            let subject = parts[0].replace("SUBJECT: ", "").trim().to_string();
            let body = parts[1].trim().to_string();
            Ok((subject, body))
        } else {
            Err("Failed to parse AI response".to_string())
        }
    }

    /// Generate multiple emails in batch
    pub async fn generate_batch_emails(
        &self,
        contacts: &[CollectedContact],
        product_name: &str,
        email_type: &str,
    ) -> Vec<Result<(String, String), String>> {
        let mut results = Vec::new();
        
        for contact in contacts {
            let result = self.generate_email(contact, product_name, email_type).await;
            results.push(result);
        }
        
        results
    }

    /// Generate A/B test variants
    pub async fn generate_variants(
        &self,
        contact: &CollectedContact,
        product_name: &str,
        num_variants: usize,
    ) -> Vec<(String, String)> {
        let mut variants = Vec::new();
        
        for i in 1..=num_variants {
            let variant_type = match i {
                1 => "Short and direct",
                2 => "Story-telling approach",
                3 => "Value proposition focused",
                _ => "Friendly casual tone",
            };
            
            if let Ok((subject, body)) = self.generate_email(
                contact, product_name, variant_type
            ).await {
                variants.push((subject, body));
            }
        }
        
        variants
    }

    async fn call_llm(&self, prompt: &str) -> Result<String, String> {
        // Simplified - in production use actual API call
        Ok(format!(
            "SUBJECT: Quick question about {} - {}
            
BODY: Hi {},

I noticed {} is doing some exciting work in the {} space, and I thought there might be an opportunity to help streamline some of your processes.

At ScrapeFlux, we've helped companies like yours save significant time on data collection and outreach.

Would you be open to a quick 15-minute call this week to explore if there's a fit?

Best,
Antor",
            "AI-powered data collection",
            "",
            "",
            "",
            "tech"
        ))
    }
}

// ============================================================================
// AUTH SERVICE
// ============================================================================

#[derive(Debug, Clone)]
pub struct AuthService {
    users: HashMap<String, User>,
    sessions: HashMap<String, Session>,
    subscriptions: HashMap<String, Subscription>,
    contacts: HashMap<String, CollectedContact>,
    email_templates: HashMap<String, EmailTemplate>,
    email_jobs: HashMap<String, EmailJob>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
            subscriptions: HashMap::new(),
            contacts: HashMap::new(),
            email_templates: HashMap::new(),
            email_jobs: HashMap::new(),
        }
    }

    // ========== USER AUTHENTICATION ==========

    pub fn register(&mut self, email: &str, password: &str, first_name: &str, last_name: &str) -> Result<User, String> {
        // Check if email exists
        if self.users.values().any(|u| u.email == email) {
            return Err("Email already registered".to_string());
        }

        // Hash password (simplified - use bcrypt in production)
        let password_hash = hash_password(password);

        // Create user
        let mut user = User::new(email, &password_hash, first_name, last_name);
        user.status = UserStatus::Active;

        // Create free subscription
        let subscription = Subscription::new(&user.id, SubscriptionPlan::Free);
        self.subscriptions.insert(subscription.id.clone(), subscription.clone());

        // Save user
        self.users.insert(user.id.clone(), user.clone());

        Ok(user)
    }

    pub fn login(&mut self, email: &str, password: &str) -> Result<(User, Session), String> {
        // Find user
        let user = self.users.values()
            .find(|u| u.email == email)
            .ok_or("Invalid email or password")?;

        // Verify password
        if !verify_password(password, &user.password_hash) {
            return Err("Invalid email or password".to_string());
        }

        // Update last login
        let mut user = user.clone();
        user.last_login = Some(Utc::now());
        user.updated_at = Utc::now();
        self.users.insert(user.id.clone(), user.clone());

        // Create session
        let session = Session {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user.id.clone(),
            token: generate_token(),
            expires_at: Utc::now() + Duration::days(7),
            created_at: Utc::now(),
        };
        self.sessions.insert(session.token.clone(), session.clone());

        Ok((user, session))
    }

    pub fn logout(&mut self, token: &str) -> Result<(), String> {
        self.sessions.remove(token);
        Ok(())
    }

    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    pub fn get_user_by_email(&self, email: &str) -> Option<&User> {
        self.users.values().find(|u| u.email == email)
    }

    // ========== SUBSCRIPTION MANAGEMENT ==========

    pub fn upgrade_subscription(&mut self, user_id: &str, plan: SubscriptionPlan) -> Result<Subscription, String> {
        let subscription = Subscription::new(user_id, plan);
        let subscription_id = subscription.id.clone();
        self.subscriptions.insert(subscription_id, subscription.clone());
        Ok(subscription)
    }

    pub fn get_subscription(&self, user_id: &str) -> Option<&Subscription> {
        self.subscriptions.values()
            .find(|s| s.user_id == user_id)
    }

    pub fn cancel_subscription(&mut self, user_id: &str) -> Result<(), String> {
        if let Some(sub) = self.subscriptions.values_mut().find(|s| s.user_id == user_id) {
            sub.status = SubscriptionStatus::Cancelled;
            Ok(())
        } else {
            Err("No subscription found".to_string())
        }
    }

    // ========== DATA COLLECTION ==========

    pub fn add_contact(&mut self, user_id: &str, contact: CollectedContact) -> CollectedContact {
        let contact_id = contact.id.clone();
        self.contacts.insert(contact_id, contact.clone());
        contact
    }

    pub fn get_contacts(&self, user_id: &str) -> Vec<&CollectedContact> {
        self.contacts.values()
            .filter(|c| c.user_id == user_id)
            .collect()
    }

    pub fn enrich_contact(&mut self, contact_id: &str, enrichment_data: EnrichmentData) -> Result<CollectedContact, String> {
        let contact = self.contacts.get_mut(contact_id)
            .ok_or("Contact not found")?;

        // Apply enrichment
        if let Some(company) = enrichment_data.company_name {
            contact.company_name = company;
        }
        if let Some(position) = enrichment_data.position {
            contact.position = Some(position);
        }
        if let Some(website) = enrichment_data.company_website {
            contact.company_website = Some(website);
        }
        if let Some(linkedin) = enrichment_data.linkedin_url {
            contact.linkedin_url = Some(linkedin);
        }
        contact.enriched_at = Some(Utc::now());

        Ok(contact.clone())
    }

    pub fn import_contacts(&mut self, user_id: &str, contacts: Vec<CollectedContact>) -> usize {
        let mut count = 0;
        for mut contact in contacts {
            contact.user_id = user_id.to_string();
            let id = contact.id.clone();
            self.contacts.insert(id, contact);
            count += 1;
        }
        count
    }

    // ========== EMAIL AUTOMATION ==========

    pub fn create_template(&mut self, user_id: &str, name: &str, category: EmailCategory) -> EmailTemplate {
        let template = EmailTemplate {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: name.to_string(),
            subject_template: String::new(),
            body_template: String::new(),
            variables: vec![
                "person_name".to_string(),
                "company_name".to_string(),
                "position".to_string(),
                "product_name".to_string(),
            ],
            category,
            is_active: true,
            created_at: Utc::now(),
        };
        let id = template.id.clone();
        self.email_templates.insert(id, template.clone());
        template
    }

    pub fn queue_email(&mut self, job: EmailJob) -> EmailJob {
        let id = job.id.clone();
        self.email_jobs.insert(id, job.clone());
        job
    }

    pub fn send_email(&mut self, job_id: &str) -> Result<EmailJob, String> {
        let job = self.email_jobs.get_mut(job_id)
            .ok_or("Email job not found")?;

        job.status = EmailStatus::Sent;
        job.sent_at = Some(Utc::now());

        Ok(job.clone())
    }

    pub fn batch_send(&mut self, job_ids: &[String]) -> Vec<Result<EmailJob, String>> {
        job_ids.iter()
            .map(|id| self.send_email(id))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichmentData {
    pub company_name: Option<String>,
    pub position: Option<String>,
    pub company_website: Option<String>,
    pub linkedin_url: Option<String>,
    pub twitter: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn hash_password(password: &str) -> String {
    // Simplified - use bcrypt in production
    format!("hash_{}", password)
}

fn verify_password(password: &str, hash: &str) -> bool {
    // Simplified - use bcrypt in production
    hash == format!("hash_{}", password)
}

fn generate_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

// ============================================================================
// API ENDPOINTS STRUCTURE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.to_string()),
            message: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeRequest {
    pub plan: SubscriptionPlan,
    pub payment_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddContactRequest {
    pub person_name: String,
    pub email: String,
    pub mobile: Option<String>,
    pub company_name: Option<String>,
    pub company_website: Option<String>,
    pub position: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateEmailRequest {
    pub contact_ids: Vec<String>,
    pub product_name: String,
    pub email_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailRequest {
    pub job_ids: Vec<String>,
}