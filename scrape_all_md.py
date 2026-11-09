import os
import json
import requests
from scrapling import Fetcher, Selector

urls = [
    ("home", "https://micstream.io/"),
    ("download", "https://micstream.io/download"),
    ("guides_index", "https://micstream.io/guides"),
    ("guide_comparison", "https://micstream.io/guides/micstream-vs-audiorelay-vs-wo-mic"),
    ("guide_obs", "https://micstream.io/guides/phone-mic-for-obs"),
    ("guide_iphone", "https://micstream.io/guides/use-iphone-as-pc-mic"),
    ("guide_linux", "https://micstream.io/guides/use-phone-as-mic-linux"),
    ("guide_windows", "https://micstream.io/guides/use-phone-as-mic-windows"),
    ("press", "https://micstream.io/press"),
    ("privacy", "https://micstream.io/privacy"),
    ("terms", "https://micstream.io/terms"),
]

os.makedirs("scraped_md", exist_ok=True)
fetcher = Fetcher()

for name, url in urls:
    print(f"Fetching {name} ({url})...")
    try:
        res = fetcher.get(url)
        md = res.markdown()
        with open(f"scraped_md/{name}.md", "w", encoding="utf-8") as f:
            f.write(f"# Source: {url}\n\n" + md)
    except Exception as e:
        print(f"Error fetching {name}: {e}")

# Fetch play store details
print("Fetching Play Store...")
try:
    r = requests.get("https://play.google.com/store/apps/details?id=com.totech.phoneasmic&hl=en_US", headers={"User-Agent": "Mozilla/5.0"})
    sel = Selector(r.text)
    # Extract description and details
    desc = sel.css('div[data-g-id="description"]').markdown() if sel.css('div[data-g-id="description"]') else sel.markdown()
    with open("scraped_md/playstore.md", "w", encoding="utf-8") as f:
        f.write("# Google Play Store Listing\n\n" + desc)
except Exception as e:
    print(f"Playstore error: {e}")

# Fetch app store details
print("Fetching App Store...")
try:
    r = requests.get("https://apps.apple.com/us/app/micstream-phone-as-pc-mic/id6791630989", headers={"User-Agent": "Mozilla/5.0"})
    sel = Selector(r.text)
    with open("scraped_md/appstore.md", "w", encoding="utf-8") as f:
        f.write("# Apple App Store Listing\n\n" + sel.markdown())
except Exception as e:
    print(f"Appstore error: {e}")

print("All scraping finished successfully!")
