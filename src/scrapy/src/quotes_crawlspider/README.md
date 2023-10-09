# Quotes Scraper

A scraper for [quotes.toscrape.com](http://quotes.toscrape.com) using `CrawlSpider`.

## Lesson Learned

- [CrawlSpider](https://docs.scrapy.org/en/latest/topics/spiders.html#crawlspider).
- [Link Extractors](https://docs.scrapy.org/en/latest/topics/link-extractors.html#topics-link-extractors).
- [Spiders Contracts](https://docs.scrapy.org/en/latest/topics/contracts.html)

## Getting Started

``` shell
$ # start crawling
$ scrapy crawl quotes_spider -O output/quotes.json
```


## Logs

<details>
  <summary>quotes spider</summary>

```python
2021-11-11 09:07:10 [scrapy.extensions.feedexport] INFO: Stored json feed (100 items) in: output/quotes.json

2021-11-11 09:07:10 [scrapy.statscollectors] INFO: Dumping Scrapy stats:
{'downloader/request_bytes': 60056,
 'downloader/request_count': 212,
 'downloader/request_method_count/GET': 212,
 'downloader/response_bytes': 267037,
 'downloader/response_count': 212,
 'downloader/response_status_count/200': 111,
 'downloader/response_status_count/308': 100,
 'downloader/response_status_count/404': 1,
 'dupefilter/filtered': 9,
 'elapsed_time_seconds': 1.921259,
 'feedexport/success_count/FileFeedStorage': 1,
 'finish_reason': 'finished',
 'finish_time': datetime.datetime(2021, 11, 11, 2, 7, 10, 469964),
 'httpcache/hit': 212,
 'httpcompression/response_bytes': 488551,
 'httpcompression/response_count': 111,
 'item_scraped_count': 100,
 'log_count/DEBUG': 314,
 'log_count/INFO': 111,
 'memusage/max': 57786368,
 'memusage/startup': 57786368,
 'request_depth_max': 10,
 'response_received_count': 112,
 'robotstxt/request_count': 1,
 'robotstxt/response_count': 1,
 'robotstxt/response_status_count/404': 1,
 'scheduler/dequeued': 211,
 'scheduler/dequeued/memory': 211,
 'scheduler/enqueued': 211,
 'scheduler/enqueued/memory': 211,
 'start_time': datetime.datetime(2021, 11, 11, 2, 7, 8, 548705)}
```

</details>
