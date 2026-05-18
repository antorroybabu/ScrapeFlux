/*
 * =============================================================================
 * Module: Multi-Platform Data Extractors
 * Project: ScrapeFlux - Ultimate Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * GitHub: https://github.com/antorroybabu/ScrapeFlux
 * License: MIT
 * 
 * Description:
 *     Custom extractors for Google, Social Media, Google Maps, and more.
 *     This is ScrapeFlux's own implementation with multi-IP rotation.
 * =============================================================================
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::advanced_data::{TokenBucket, CircuitBreaker};

// ============================================================================
// GOOGLE EXTRACTOR
// ============================================================================

#[derive(Debug, Clone)]
pub struct GoogleExtractor {
    client: MultiIPClient,
    rate_limiter: TokenBucket,
    circuit_breaker: CircuitBreaker,
}

impl GoogleExtractor {
    pub fn new(proxies: Vec<ProxyConfig>) -> Self {
        Self {
            client: MultiIPClient::new(proxies),
            rate_limiter: TokenBucket::new(50, 10.0),
            circuit_breaker: CircuitBreaker::new(10, 60),
        }
    }

    pub async fn search(&self, query: &str, num_results: usize) -> Result<Vec<SearchResult>, String> {
        self.rate_limiter.try_acquire(1);
        
        let search_url = format!(
            "https://www.google.com/search?q={}&num={}",
            urlencoding(query),
            num_results.min(100)
        );
        
        let html = self.client.get(&search_url).await?;
        self.parse_search_results(&html)
    }

    pub async fn search_news(&self, query: &str, days: u32) -> Result<Vec<NewsResult>, String> {
        self.rate_limiter.try_acquire(1);
        
        let news_url = format!(
            "https://www.google.com/search?q={}&tbs={}d&tbm=nws",
            urlencoding(query),
            days
        );
        
        let html = self.client.get(&news_url).await?;
        self.parse_news_results(&html)
    }

    pub async fn search_images(&self, query: &str, num: usize) -> Result<Vec<ImageResult>, String> {
        self.rate_limiter.try_acquire(1);
        
        let images_url = format!(
            "https://www.google.com/search?q={}&tbm=isch",
            urlencoding(query)
        );
        
        let html = self.client.get(&images_url).await?;
        self.parse_image_results(&html)
    }

    pub async fn search_maps(&self, query: &str, location: &str) -> Result<Vec<MapPlace>, String> {
        self.rate_limiter.try_acquire(1);
        
        let maps_url = format!(
            "https://www.google.com/maps/search/{}/?{}",
            urlencoding(query),
            urlencoding(location)
        );
        
        let html = self.client.get(&maps_url).await?;
        self.parse_map_places(&html)
    }

    fn parse_search_results(&self, html: &str) -> Result<Vec<SearchResult>, String> {
        let mut results = Vec::new();
        // Custom parsing logic for Google search results
        // Extract title, URL, snippet
        results
    }

    fn parse_news_results(&self, html: &str) -> Result<Vec<NewsResult>, String> {
        let mut results = Vec::new();
        // Custom parsing for news articles
        results
    }

    fn parse_image_results(&self, html: &str) -> Result<Vec<ImageResult>, String> {
        let mut results = Vec::new();
        // Custom parsing for images
        results
    }

    fn parse_map_places(&self, html: &str) -> Result<Vec<MapPlace>, String> {
        let mut places = Vec::new();
        // Custom parsing for Google Maps places
        places
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub position: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsResult {
    pub title: String,
    pub url: String,
    pub source: String,
    pub date: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResult {
    pub title: String,
    pub url: String,
    pub source_url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapPlace {
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub rating: Option<f32>,
    pub reviews_count: Option<u32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub place_id: String,
    pub category: String,
    pub hours: Option<HashMap<String, String>>,
    pub photos: Vec<String>,
    pub reviews: Vec<Review>,
    pub website: Option<String>,
}

// ============================================================================
// SOCIAL MEDIA EXTRACTORS
// ============================================================================

#[derive(Debug, Clone)]
pub struct TwitterExtractor {
    client: MultiIPClient,
    api_key: Option<String>,
    rate_limiter: TokenBucket,
}

impl TwitterExtractor {
    pub fn new(api_key: Option<String>, proxies: Vec<ProxyConfig>) -> Self {
        Self {
            client: MultiIPClient::new(proxies),
            api_key,
            rate_limiter: TokenBucket::new(100, 20.0),
        }
    }

    pub async fn get_tweets(&self, query: &str, count: usize) -> Result<Vec<Tweet>, String> {
        self.rate_limiter.try_acquire(1);
        
        // Twitter search URL
        let url = format!(
            "https://twitter.com/search?q={}&src=typed_query",
            urlencoding(query)
        );
        
        let html = self.client.get(&url).await?;
        self.parse_tweets(&html)
    }

    pub async fn get_user_profile(&self, username: &str) -> Result<TwitterUser, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!("https://twitter.com/{}", username);
        let html = self.client.get(&url).await?;
        self.parse_user_profile(&html)
    }

    fn parse_tweets(&self, html: &str) -> Result<Vec<Tweet>, String> {
        let mut tweets = Vec::new();
        // Custom tweet parsing
        tweets
    }

    fn parse_user_profile(&self, html: &str) -> Result<TwitterUser, String> {
        // Parse user profile
        Ok(TwitterUser {
            username: String::new(),
            display_name: String::new(),
            bio: String::new(),
            followers: 0,
            following: 0,
            tweets_count: 0,
            created_at: String::new(),
            verified: false,
            profile_image: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub text: String,
    pub created_at: String,
    pub likes: u32,
    pub retweets: u32,
    pub replies: u32,
    pub is_retweet: bool,
    pub is_reply: bool,
    pub media: Vec<String>,
    pub hashtags: Vec<String>,
    pub mentions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterUser {
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub followers: u64,
    pub following: u64,
    pub tweets_count: u64,
    pub created_at: String,
    pub verified: bool,
    pub profile_image: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FacebookExtractor {
    client: MultiIPClient,
    rate_limiter: TokenBucket,
}

impl FacebookExtractor {
    pub fn new(proxies: Vec<ProxyConfig>) -> Self {
        Self {
            client: MultiIPClient::new(proxies),
            rate_limiter: TokenBucket::new(30, 5.0),
        }
    }

    pub async fn get_page_posts(&self, page_name: &str) -> Result<Vec<FacebookPost>, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!("https://www.facebook.com/{}/posts", page_name);
        let html = self.client.get(&url).await?;
        self.parse_posts(&html)
    }

    pub async fn get_post_comments(&self, post_id: &str) -> Result<Vec<Comment>, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!("https://www.facebook.com/{}/comments", post_id);
        let html = self.client.get(&url).await?;
        self.parse_comments(&html)
    }

    fn parse_posts(&self, html: &str) -> Result<Vec<FacebookPost>, String> {
        Ok(Vec::new())
    }

    fn parse_comments(&self, html: &str) -> Result<Vec<Comment>, String> {
        Ok(Vec::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacebookPost {
    pub id: String,
    pub author: String,
    pub text: String,
    pub created_at: String,
    pub likes: u32,
    pub comments: u32,
    pub shares: u32,
    pub media: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub author: String,
    pub text: String,
    pub created_at: String,
    pub likes: u32,
    pub replies: u32,
}

#[derive(Debug, Clone)]
pub struct InstagramExtractor {
    client: MultiIPClient,
    rate_limiter: TokenBucket,
}

impl InstagramExtractor {
    pub fn new(proxies: Vec<ProxyConfig>) -> Self {
        Self {
            client: MultiIPClient::new(proxies),
            rate_limiter: TokenBucket::new(30, 5.0),
        }
    }

    pub async fn get_hashtag_posts(&self, hashtag: &str, count: usize) -> Result<Vec<InstagramPost>, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!("https://www.instagram.com/explore/tags/{}", hashtag);
        let html = self.client.get(&url).await?;
        self.parse_posts(&html)
    }

    pub async fn get_profile(&self, username: &str) -> Result<InstagramProfile, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!("https://www.instagram.com/{}", username);
        let html = self.client.get(&url).await?;
        self.parse_profile(&html)
    }

    fn parse_posts(&self, html: &str) -> Result<Vec<InstagramPost>, String> {
        Ok(Vec::new())
    }

    fn parse_profile(&self, html: &str) -> Result<InstagramProfile, String> {
        Ok(InstagramProfile {
            username: String::new(),
            full_name: String::new(),
            bio: String::new(),
            followers: 0,
            following: 0,
            posts_count: 0,
            is_verified: false,
            profile_image: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstagramPost {
    pub id: String,
    pub code: String,
    pub username: String,
    pub caption: Option<String>,
    pub created_at: String,
    pub likes: u32,
    pub comments: u32,
    pub media_type: String,
    pub media_urls: Vec<String>,
    pub hashtags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstagramProfile {
    pub username: String,
    pub full_name: String,
    pub bio: String,
    pub followers: u64,
    pub following: u64,
    pub posts_count: u64,
    pub is_verified: bool,
    pub profile_image: Option<String>,
}

// ============================================================================
// GOOGLE MAPS EXTRACTOR
// ============================================================================

#[derive(Debug, Clone)]
pub struct MapsExtractor {
    client: MultiIPClient,
    rate_limiter: TokenBucket,
}

impl MapsExtractor {
    pub fn new(proxies: Vec<ProxyConfig>) -> Self {
        Self {
            client: MultiIPClient::new(proxies),
            rate_limiter: TokenBucket::new(50, 10.0),
        }
    }

    pub async fn search_places(&self, query: &str, location: &str, radius: &str) -> Result<Vec<MapPlace>, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!(
            "https://www.google.com/maps/search/{}/@{},15z",
            urlencoding(query),
            location
        );
        
        let html = self.client.get(&url).await?;
        self.parse_places(&html)
    }

    pub async fn get_place_details(&self, place_id: &str) -> Result<MapPlace, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!("https://www.google.com/maps/place/{}", place_id);
        let html = self.client.get(&url).await?;
        self.parse_place_details(&html)
    }

    pub async fn get_reviews(&self, place_id: &str, min_rating: Option<u32>) -> Result<Vec<Review>, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!("https://www.google.com/maps/place/{}/reviews", place_id);
        let html = self.client.get(&url).await?;
        self.parse_reviews(&html, min_rating)
    }

    pub async fn get_place_photos(&self, place_id: &str) -> Result<Vec<String>, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!("https://www.google.com/maps/place/{}/photos", place_id);
        let html = self.client.get(&url).await?;
        self.parse_photos(&html)
    }

    pub async fn get_directions(&self, origin: &str, destination: &str) -> Result<Directions, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!(
            "https://www.google.com/maps/dir/{}/{}",
            urlencoding(origin),
            urlencoding(destination)
        );
        
        let html = self.client.get(&url).await?;
        self.parse_directions(&html)
    }

    fn parse_places(&self, html: &str) -> Result<Vec<MapPlace>, String> {
        Ok(Vec::new())
    }

    fn parse_place_details(&self, html: &str) -> Result<MapPlace, String> {
        Ok(MapPlace {
            name: String::new(),
            address: String::new(),
            phone: None,
            rating: None,
            reviews_count: None,
            latitude: None,
            longitude: None,
            place_id: String::new(),
            category: String::new(),
            hours: None,
            photos: Vec::new(),
            reviews: Vec::new(),
            website: None,
        })
    }

    fn parse_reviews(&self, html: &str, _min_rating: Option<u32>) -> Result<Vec<Review>, String> {
        Ok(Vec::new())
    }

    fn parse_photos(&self, html: &str) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    fn parse_directions(&self, html: &str) -> Result<Directions, String> {
        Ok(Directions {
            origin: String::new(),
            destination: String::new(),
            distance: String::new(),
            duration: String::new(),
            steps: Vec::new(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub author: String,
    pub rating: u32,
    pub text: String,
    pub date: String,
    pub helpful: u32,
    pub response: Option<String>,
    pub photos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directions {
    pub origin: String,
    pub destination: String,
    pub distance: String,
    pub duration: String,
    pub steps: Vec<DirectionStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectionStep {
    pub instruction: String,
    pub distance: String,
    pub duration: String,
    pub start_location: (f64, f64),
    pub end_location: (f64, f64),
}

// ============================================================================
// MULTI-IP CLIENT
// ============================================================================

#[derive(Debug, Clone)]
pub struct MultiIPClient {
    proxy_pool: ProxyPool,
    current_index: usize,
    requests_per_ip: usize,
    request_counts: HashMap<usize, usize>,
}

impl MultiIPClient {
    pub fn new(proxies: Vec<ProxyConfig>) -> Self {
        Self {
            proxy_pool: ProxyPool::new(proxies),
            current_index: 0,
            requests_per_ip: 50,
            request_counts: HashMap::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<String, String> {
        let proxy = self.get_next_proxy();
        
        // Make request through proxy
        // In real implementation, this would use reqwest with proxy config
        Ok(String::new())
    }

    fn get_next_proxy(&mut self) -> &ProxyConfig {
        // Check if current IP has exceeded limit
        let count = self.request_counts.get(&self.current_index).unwrap_or(&0);
        
        if *count >= self.requests_per_ip {
            // Rotate to next proxy
            self.current_index = (self.current_index + 1) % self.proxy_pool.len();
            self.request_counts.insert(self.current_index, 0);
        }
        
        self.request_counts.insert(self.current_index, count + 1);
        
        self.proxy_pool.get(self.current_index)
    }

    pub fn set_requests_per_ip(&mut self, count: usize) {
        self.requests_per_ip = count;
    }
}

#[derive(Debug, Clone)]
pub struct ProxyPool {
    proxies: Vec<ProxyConfig>,
}

impl ProxyPool {
    pub fn new(proxies: Vec<ProxyConfig>) -> Self {
        Self { proxies }
    }

    pub fn get(&self, index: usize) -> &ProxyConfig {
        &self.proxies[index % self.proxies.len()]
    }

    pub fn len(&self) -> usize {
        self.proxies.len()
    }

    pub fn add(&mut self, proxy: ProxyConfig) {
        self.proxies.push(proxy);
    }

    pub fn remove(&mut self, index: usize) {
        self.proxies.remove(index);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub proxy_type: ProxyType,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub country: Option<String>,
    pub api_key: Option<String>,
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum ProxyType {
    Http,
    Socks5,
    Residential,
    Mobile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: String,
    pub author: String,
    pub rating: u32,
    pub text: String,
    pub date: String,
    pub helpful: u32,
    pub response: Option<String>,
    pub photos: Vec<String>,
}

// ============================================================================
// DATA SEPARATOR
// ============================================================================

#[derive(Debug, Clone)]
pub struct DataSeparator {
    output_dir: String,
}

impl DataSeparator {
    pub fn new(output_dir: &str) -> Self {
        Self {
            output_dir: output_dir.to_string(),
        }
    }

    pub fn separate_google_data(&self, data: GoogleData) -> HashMap<String, serde_json::Value> {
        let mut output = HashMap::new();
        
        if !data.search_results.is_empty() {
            output.insert("google_search.json".to_string(), serde_json::to_value(&data.search_results).unwrap());
        }
        if !data.maps_places.is_empty() {
            output.insert("google_maps.json".to_string(), serde_json::to_value(&data.maps_places).unwrap());
        }
        if !data.news_articles.is_empty() {
            output.insert("google_news.json".to_string(), serde_json::to_value(&data.news_articles).unwrap());
        }
        
        output
    }

    pub fn separate_social_data(&self, data: SocialData) -> HashMap<String, serde_json::Value> {
        let mut output = HashMap::new();
        
        if !data.twitter.is_empty() {
            output.insert("twitter.json".to_string(), serde_json::to_value(&data.twitter).unwrap());
        }
        if !data.facebook.is_empty() {
            output.insert("facebook.json".to_string(), serde_json::to_value(&data.facebook).unwrap());
        }
        if !data.instagram.is_empty() {
            output.insert("instagram.json".to_string(), serde_json::to_value(&data.instagram).unwrap());
        }
        if !data.linkedin.is_empty() {
            output.insert("linkedin.json".to_string(), serde_json::to_value(&data.linkedin).unwrap());
        }
        
        output
    }

    pub fn separate_maps_data(&self, data: MapsData) -> HashMap<String, serde_json::Value> {
        let mut output = HashMap::new();
        
        output.insert("maps_places.json".to_string(), serde_json::to_value(&data.places).unwrap());
        output.insert("maps_reviews.json".to_string(), serde_json::to_value(&data.reviews).unwrap());
        output.insert("maps_photos.json".to_string(), serde_json::to_value(&data.photos).unwrap());
        output.insert("maps_directions.json".to_string(), serde_json::to_value(&data.directions).unwrap());
        
        output
    }

    pub fn separate_website_data(&self, data: WebsiteData) -> HashMap<String, serde_json::Value> {
        let mut output = HashMap::new();
        
        if let Some(ecommerce) = data.ecommerce {
            output.insert("website_ecommerce.json".to_string(), serde_json::to_value(&ecommerce).unwrap());
        }
        if let Some(news) = data.news {
            output.insert("website_news.json".to_string(), serde_json::to_value(&news).unwrap());
        }
        if let Some(business) = data.business {
            output.insert("website_business.json".to_string(), serde_json::to_value(&business).unwrap());
        }
        
        output
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleData {
    pub search_results: Vec<SearchResult>,
    pub maps_places: Vec<MapPlace>,
    pub news_articles: Vec<NewsResult>,
    pub images: Vec<ImageResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialData {
    pub twitter: Vec<Tweet>,
    pub facebook: Vec<FacebookPost>,
    pub instagram: Vec<InstagramPost>,
    pub linkedin: Vec<JobPosting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapsData {
    pub places: Vec<MapPlace>,
    pub reviews: Vec<Review>,
    pub photos: Vec<String>,
    pub directions: Vec<Directions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteData {
    pub ecommerce: Option<EcommerceData>,
    pub news: Option<NewsSiteData>,
    pub business: Option<BusinessData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcommerceData {
    pub products: Vec<Product>,
    pub prices: Vec<Price>,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsSiteData {
    pub articles: Vec<Article>,
    pub headlines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessData {
    pub info: BusinessInfo,
    pub contact: ContactInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub images: Vec<String>,
    pub category: String,
    pub stock: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub product_id: String,
    pub price: f64,
    pub currency: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub author: String,
    pub date: String,
    pub url: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessInfo {
    pub name: String,
    pub description: String,
    pub founded: Option<String>,
    pub employees: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Vec<String>,
    pub phone: Vec<String>,
    pub address: String,
    pub social_links: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedInExtractor {
    client: MultiIPClient,
    rate_limiter: TokenBucket,
}

impl LinkedInExtractor {
    pub fn new(proxies: Vec<ProxyConfig>) -> Self {
        Self {
            client: MultiIPClient::new(proxies),
            rate_limiter: TokenBucket::new(20, 5.0),
        }
    }

    pub async fn search_jobs(&self, query: &str, location: &str) -> Result<Vec<JobPosting>, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!(
            "https://www.linkedin.com/jobs/search/?keywords={}&location={}",
            urlencoding(query),
            urlencoding(location)
        );
        
        let html = self.client.get(&url).await?;
        self.parse_jobs(&html)
    }

    pub async fn search_people(&self, query: &str) -> Result<Vec<LinkedInProfile>, String> {
        self.rate_limiter.try_acquire(1);
        
        let url = format!(
            "https://www.linkedin.com/search/results/all/?keywords={}",
            urlencoding(query)
        );
        
        let html = self.client.get(&url).await?;
        self.parse_profiles(&html)
    }

    fn parse_jobs(&self, html: &str) -> Result<Vec<JobPosting>, String> {
        Ok(Vec::new())
    }

    fn parse_profiles(&self, html: &str) -> Result<Vec<LinkedInProfile>, String> {
        Ok(Vec::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPosting {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub description: String,
    pub salary: Option<String>,
    pub posted_date: String,
    pub job_type: String,
    pub seniority: String,
    pub skills: Vec<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedInProfile {
    pub name: String,
    pub headline: String,
    pub location: String,
    pub current_title: Option<String>,
    pub current_company: Option<String>,
    pub connections: Option<u32>,
    pub url: String,
}

// ============================================================================
// HELPER FUNCTION
// ============================================================================

fn urlencoding(s: &str) -> String {
    s.replace(' ', "+")
        .replace('&', "%26")
        .replace('#', "%23")
        .replace('"', "%22")
}