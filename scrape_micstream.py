import json
import os
import requests
from scrapling import Fetcher, Selector

urls = [
    "https://micstream.io/",
    "https://micstream.io/download",
    "https://micstream.io/guides",
    "https://micstream.io/guides/micstream-vs-audiorelay-vs-wo-mic",
    "https://micstream.io/guides/phone-mic-for-obs",
    "https://micstream.io/guides/use-iphone-as-pc-mic",
    "https://micstream.io/guides/use-phone-as-mic-linux",
    "https://micstream.io/guides/use-phone-as-mic-windows",
    "https://micstream.io/press",
    "https://micstream.io/privacy",
    "https://micstream.io/terms",
]

fetcher = Fetcher()
results = {}

for url in urls:
    print(f"Fetching {url}...")
    try:
        res = fetcher.get(url)
        sel = Selector(res.text)
        title = sel.css("title::text").get()
        meta_desc = sel.css('meta[name="description"]::attr(content)').get()
        # Extract main text
        headings = [h.strip() for h in sel.css("h1::text, h2::text, h3::text, h4::text").getall() if h.strip()]
        paragraphs = [p.strip() for p in sel.css("p::text, li::text").getall() if p.strip()]
        
        # Save raw text and structure
        results[url] = {
            "status": res.status,
            "title": title,
            "meta_description": meta_desc,
            "headings": headings,
            "paragraphs": paragraphs,
            "html": res.text
        }
    except Exception as e:
        print(f"Error fetching {url}: {e}")

# Also scrape app store pages if possible
app_urls = {
    "playstore": "https://play.google.com/store/apps/details?id=com.totech.phoneasmic",
    "appstore": "https://apps.apple.com/app/id6791630989"
}

headers = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
}

for name, url in app_urls.items():
    print(f"Fetching app info {name}...")
    try:
        r = requests.get(url, headers=headers, timeout=15)
        sel = Selector(r.text)
        results[name] = {
            "status": r.status_code,
            "title": sel.css("title::text").get(),
            "text": [p.strip() for p in sel.css("p::text, div::text, span::text").getall() if len(p.strip()) > 20][:100],
            "html_len": len(r.text)
        }
    except Exception as e:
        print(f"Error fetching {name}: {e}")

os.makedirs("scraped_data", exist_ok=True)
with open("scraped_data/micstream_all.json", "w", encoding="utf-8") as f:
    json.dump({k: {k2: v2 for k2, v2 in v.items() if k2 != "html"} for k, v in results.items()}, f, indent=2, ensure_ascii=False)

# Save individual htmls
for url, data in results.items():
    if "html" in data:
        filename = url.replace("https://micstream.io/", "").replace("/", "_") or "home"
        with open(f"scraped_data/{filename}.html", "w", encoding="utf-8") as f:
            f.write(data["html"])

print("Scraping completed!")
