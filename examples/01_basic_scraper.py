"""
ScrapeFlux Examples - Basic Usage
Author: Antor Roy
Email: antorroybabu@gmail.com
"""

# =============================================================================
# EXAMPLE 1: Basic Web Scraping
# =============================================================================

from scrapeflux import Scraper, ExtractionSchema, SchemaField, FieldType

# Create a simple scraper
scraper = Scraper()

# Define what to extract
schema = ExtractionSchema(
    name="SimplePage",
    description="Extract basic info from webpage",
    fields=[
        SchemaField(
            name="title",
            field_type=FieldType.Text,
            description="Page title"
        ),
        SchemaField(
            name="headings",
            field_type=FieldType.Array,
            description="All H1-H3 headings"
        ),
        SchemaField(
            name="links",
            field_type=FieldType.Array,
            description="All links on page"
        ),
    ]
)

# Scrape a webpage
async def basic_example():
    result = await scraper.scrape(
        url="https://example.com",
        schema=schema
    )
    
    print(f"Title: {result.data['title']}")
    print(f"Found {len(result.data['headings'])} headings")
    print(f"Found {len(result.data['links'])} links")
    print(f"Confidence: {result.confidence}")

# =============================================================================
# EXAMPLE 2: Product Extraction
# =============================================================================

from scrapeflux import Scraper, ExtractionSchema, SchemaField, FieldType

scraper = Scraper()

# Define product extraction schema
product_schema = ExtractionSchema(
    name="Product",
    description="Extract product information",
    fields=[
        SchemaField(name="name", field_type=FieldType.Text, description="Product name"),
        SchemaField(name="price", field_type=FieldType.Price, description="Product price"),
        SchemaField(name="description", field_type=FieldType.Text, description="Product description"),
        SchemaField(name="images", field_type=FieldType.Array, description="Product images"),
        SchemaField(name="rating", field_type=FieldType.Number, description="Product rating"),
        SchemaField(name="reviews_count", field_type=FieldType.Number, description="Number of reviews"),
        SchemaField(name="availability", field_type=FieldType.Text, description="Stock status"),
    ]
)

async def product_example():
    result = await scraper.scrape(
        url="https://shop.example.com/products/123",
        schema=product_schema,
        use_llm=True  # Use LLM for better extraction
    )
    
    print(f"Product: {result.data['name']}")
    print(f"Price: ${result.data['price']}")
    print(f"Rating: {result.data['rating']}/5 ({result.data['reviews_count']} reviews)")
    print(f"In stock: {result.data['availability']}")

# =============================================================================
# EXAMPLE 3: Multi-Page Crawling
# =============================================================================

from scrapeflux import SuperCrawler, CrawlConfig

config = CrawlConfig(
    max_depth=3,
    max_pages=100,
    concurrency=10,
    respect_robots_txt=True,
    deduplicate=True
)

crawler = SuperCrawler(config)

async def crawl_example():
    result = await crawler.crawl("https://blog.example.com")
    
    print(f"Crawled {len(result.pages)} pages")
    print(f"Total links found: {len(result.all_links)}")
    print(f"Total images found: {len(result.all_images)}")
    print(f"Time taken: {result.metrics.crawl_time}s")
    print(f"Pages per second: {result.metrics.pages_per_second}")

# =============================================================================
# EXAMPLE 4: AI Agent (Autonomous Scraping)
# =============================================================================

from scrapeflux import AIAgent, AgentConfig, AgentTask

config = AgentConfig(
    model={"provider": "openai", "model_name": "gpt-4"},
    max_steps=20,
    reasoning_enabled=True,
    self_correction_enabled=True
)

agent = AIAgent(config)

async def agent_example():
    # Define the task
    task = AgentTask(
        description="Find all job listings for Python developers",
        goal="Extract job title, company, salary, and location"
    )
    task.with_context("url", "https://jobs.example.com")
    task.with_constraint("Only extract Python-related jobs")
    task.with_success_criteria("Extracted 50+ jobs")
    
    # Execute the autonomous task
    result = await agent.execute(task)
    
    print(f"Task completed: {result.task_completed}")
    print(f"Steps taken: {result.total_steps}")
    print(f"Success rate: {result.success_rate:.1%}")
    print(f"Extracted data: {result.final_result}")

# =============================================================================
# EXAMPLE 5: Browser Automation
# =============================================================================

from scrapeflux import BrowserAutomation, BrowserConfig, ActionType

config = BrowserConfig(
    headless=True,
    viewport={"width": 1920, "height": 1080}
)

browser = BrowserAutomation(config)

async def browser_example():
    await browser.launch()
    
    # Navigate
    await browser.navigate("https://form.example.com")
    
    # Fill form
    await browser.type("#name", "John Doe")
    await browser.type("#email", "john@example.com")
    await browser.select_option("#country", "US")
    await browser.click("#submit")
    
    # Wait for response
    await browser.wait_for_selector(".success-message", timeout=5000)
    
    # Take screenshot
    screenshot = await browser.screenshot(full_page=True)
    screenshot.save("result.png")
    
    await browser.close()

# =============================================================================
# EXAMPLE 6: LLM-Powered Extraction
# =============================================================================

from scrapeflux import LLMEngine, ExtractionSchema

engine = LLMEngine(api_key="your-api-key")

async def llm_extraction_example():
    # Create schema from description
    schema = await engine.generate_schema(
        description="Extract news article information including title, author, date, content, and tags"
    )
    
    # Extract with LLM
    result = await engine.extract(
        url="https://news.example.com/article/123",
        schema=schema,
        use_few_shot=True
    )
    
    print(f"Extracted with confidence: {result.confidence}")
    print(f"Data: {result.data}")

# =============================================================================
# EXAMPLE 7: Advanced Data Structures
# =============================================================================

from scrapeflux import Trie, BloomFilter, SimHash, MinHash

# Trie for URL path matching
trie = Trie()
await trie.insert("/api/v1/users", {"method": "GET"})
await trie.insert("/api/v1/products", {"method": "GET"})
await trie.insert("/api/v1/orders", {"method": "POST"})

if await trie.search("/api/v1/users"):
    print("Path exists!")

# Bloom filter for deduplication
bloom = BloomFilter(expected_items=10000, false_positive_rate=0.001)
bloom.add("https://example.com/page1")
bloom.add("https://example.com/page2")

if bloom.check("https://example.com/page1"):
    print("Already crawled!")

# SimHash for content fingerprinting
content1 = "This is a sample article about technology..."
content2 = "This is a sample article about technology..."

hash1 = SimHash()
hash1.from_text(content1)

hash2 = SimHash()
hash2.from_text(content2)

distance = hash1.hamming_distance(hash2)
print(f"Similarity: {1 - distance/64:.2%}")

# =============================================================================
# EXAMPLE 8: Export to Multiple Formats
# =============================================================================

from scrapeflux import Scraper, MultiFormatExporter

scraper = Scraper()

async def export_example():
    result = await scraper.scrape(
        url="https://example.com/products",
        schema=product_schema
    )
    
    # Export to different formats
    json_output = MultiFormatExporter.to_json(result.data, pretty=True)
    xml_output = MultiFormatExporter.to_xml(result.data, "products")
    csv_output = MultiFormatExporter.to_csv(result.data)
    md_output = MultiFormatExporter.to_markdown(result.data)
    html_output = MultiFormatExporter.to_html(result.data)
    
    # Save to files
    with open("products.json", "w") as f:
        f.write(json_output)
    with open("products.xml", "w") as f:
        f.write(xml_output)
    with open("products.csv", "w") as f:
        f.write(csv_output)

# =============================================================================
# EXAMPLE 9: Rate Limiting & Circuit Breaker
# =============================================================================

from scrapeflux import TokenBucket, CircuitBreaker

# Token bucket for rate limiting
rate_limiter = TokenBucket(capacity=100, refill_rate=10.0)  # 100 requests burst, 10/s refill

async def rate_limited_request():
    if rate_limiter.try_acquire(1):  # Try to acquire 1 token
        # Make the request
        response = await http_client.get("https://api.example.com/data")
        return response
    else:
        print("Rate limited! Waiting...")
        await asyncio.sleep(1)
        return None

# Circuit breaker for fault tolerance
breaker = CircuitBreaker(failure_threshold=5, timeout=60)

async def protected_request():
    result = breaker.call(lambda: risky_api_call())
    return result

# =============================================================================
# EXAMPLE 10: Complete Pipeline
# =============================================================================

from scrapeflux import (
    SuperCrawler, 
    AIAgent,
    TokenBucket,
    CircuitBreaker,
    LLMEngine,
    MultiFormatExporter
)

async def complete_pipeline():
    # Setup
    crawler = SuperCrawler(max_depth=5, max_pages=1000)
    agent = AIAgent()
    llm = LLMEngine()
    rate_limiter = TokenBucket(capacity=50, refill_rate=10)
    
    # Start crawling
    crawl_result = await crawler.crawl("https://shop.example.com")
    
    # Use agent for detailed extraction on each page
    tasks = []
    for page in crawl_result.pages[:10]:  # Process first 10 pages
        task = AgentTask(
            description=f"Extract products from {page.url}",
            goal="Extract all product information"
        )
        task.with_context("html", page.html)
        tasks.append(task)
    
    # Execute with rate limiting
    results = []
    for task in tasks:
        rate_limiter.try_acquire(1)
        result = await agent.execute(task)
        results.append(result)
    
    # Merge and export
    all_products = [r.final_result for r in results if r.final_result]
    
    final_data = {"products": all_products}
    json_output = MultiFormatExporter.to_json(final_data, pretty=True)
    
    with open("all_products.json", "w") as f:
        f.write(json_output)
    
    print(f"Pipeline complete! Extracted {len(all_products)} products")

if __name__ == "__main__":
    import asyncio
    asyncio.run(complete_pipeline())