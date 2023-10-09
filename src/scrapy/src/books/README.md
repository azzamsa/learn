# Books Scraper

A scraper for [books.toscrape.com](http://books.toscrape.com).

## Lesson Learned

- CSS selector
- Parsing nested page

## Getting Started

``` shell
$ # activate the virtualenv, then install required dependencies

# create project skeleton
$ scrapy startproject books
$ scrapy genspider book_spider books.toscrape.com

$ # start crawling
$ scrapy crawl book_spider -O books.json
```


## Logs

<details>
  <summary>books spider</summary>

```python
2021-11-06 13:12:13 [scrapy.extensions.feedexport] INFO: Stored json feed (1000 items) in: books.json

2021-11-06 13:12:13 [scrapy.statscollectors] INFO: Dumping Scrapy stats:
{'downloader/request_bytes': 354623,
 'downloader/request_count': 1051,
 'downloader/request_method_count/GET': 1051,
 'downloader/response_bytes': 4346120,
 'downloader/response_count': 1051,
 'downloader/response_status_count/200': 1050,
 'downloader/response_status_count/404': 1,
 'elapsed_time_seconds': 52.865169,
 'feedexport/success_count/FileFeedStorage': 1,
 'finish_reason': 'finished',
 'finish_time': datetime.datetime(2021, 11, 6, 6, 12, 13, 742237),
 'httpcompression/response_bytes': 21921006,
 'httpcompression/response_count': 1050,
 'item_scraped_count': 1000,
 'log_count/DEBUG': 2051,
 'log_count/INFO': 11,
 'memusage/max': 57294848,
 'memusage/startup': 57294848,
 'request_depth_max': 50,
 'response_received_count': 1051,
 'robotstxt/request_count': 1,
 'robotstxt/response_count': 1,
 'robotstxt/response_status_count/404': 1,
 'scheduler/dequeued': 1050,
 'scheduler/dequeued/memory': 1050,
 'scheduler/enqueued': 1050,
 'scheduler/enqueued/memory': 1050,
 'start_time': datetime.datetime(2021, 11, 6, 6, 11, 20, 877068)}
2021-11-06 13:12:13 [scrapy.core.engine] INFO: Spider closed (finished)
```

</details>
