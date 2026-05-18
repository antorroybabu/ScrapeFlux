/*
 * =============================================================================
 * Project: ScrapeFlux - Advanced Unified Web Scraping Framework
 * Author: Antor Roy
 * Email: antorroybabu@gmail.com
 * License: MIT
 * 
 * Description:
 *     Main entry point for the CLI application
 * =============================================================================
 */

use scrapeflux::{Crawler, Extractor, HttpClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ScrapeFlux - Advanced Web Scraping Framework");
    println!("Author: Antor Roy <antorroybabu@gmail.com>");
    println!("============================================\n");

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "crawl" => {
            if args.len() < 3 {
                println!("Error: URL required for crawling");
                return Ok(());
            }
            let url = &args[2];
            println!("Crawling: {}", url);
            
            let crawler = Crawler::new()?;
            match crawler.crawl(url).await {
                Ok(page) => {
                    println!("\n✓ Crawled successfully!");
                    println!("Title: {:?}", page.title);
                    println!("Status: {}", page.status);
                    println!("Links found: {}", page.links.len());
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        "scrape" => {
            if args.len() < 4 {
                println!("Usage: scrapeflux scrape <url> <selector>");
                return Ok(());
            }
            let url = &args[2];
            let selector = &args[3];
            
            let client = HttpClient::new()?;
            println!("Fetching: {}", url);
            
            match client.get(url).await {
                Ok(response) => {
                    let mut parser = scrapeflux::HtmlParser::new(&response.body);
                    match parser.select(selector) {
                        Ok(elements) => {
                            println!("\n✓ Found {} elements", elements.len());
                            for (i, el) in elements.iter().take(5).enumerate() {
                                println!("{}. {}", i + 1, parser.text(el));
                            }
                        }
                        Err(e) => eprintln!("Selector error: {}", e),
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "batch" => {
            if args.len() < 3 {
                println!("Usage: scrapeflux batch <url1> <url2> ...");
                return Ok(());
            }
            let urls: Vec<String> = args[2..].to_vec();
            let client = HttpClient::new()?;
            
            println!("Fetching {} URLs...", urls.len());
            let results = client.fetch_batch(urls).await;
            
            let success = results.iter().filter(|r| r.is_ok()).count();
            println!("\n✓ Completed: {}/{} successful", success, results.len());
        }
        _ => {
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Usage: scrapeflux <command> [arguments]");
    println!("\nCommands:");
    println!("  crawl <url>          - Crawl a single URL");
    println!("  scrape <url> <selector> - Scrape using selector");
    println!("  batch <url1> <url2>... - Fetch multiple URLs");
    println!("\nExamples:");
    println!("  scrapeflux crawl https://example.com");
    println!("  scrapeflux scrape https://example.com h1");
    println!("  scrapeflux batch https://a.com https://b.com");
}