# Javascript Generated Content Scraper

Scrape all the quotes from JavaScript generated content.

## Getting Started

``` shell
# install dependencies
$ pip install -r requirements.txt
$ docker run -p 8050:8050 scrapinghub/splash

$ # start crawling
$ scrapy runspider quotes_js.py -O output/quotes_js.json
```

## Logs

<details>
  <summary>quotes_js.py</summary>

```python
[scrapy.extensions.feedexport] INFO: Stored json feed (100 items) in: output/quotes_js.json

 'item_scraped_count': 100,
```

</details>
