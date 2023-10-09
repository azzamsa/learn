# Quotes Scraper

A scraper for [quotes.toscrape.com](http://quotes.toscrape.com).

## Lesson Learned

- Scrapy Cloud

## Getting Started

``` shell
$ # activate the virtualenv
$ vf activate global3 # in this case I am using virtualfish

# install dependencies
$ pip install -r requirements.txt

$ # start crawling
$ scrapy crawl quotes
```


## Logs

<details>
  <summary>quotes spider</summary>

```python
2021-10-26 07:35:39 [scrapy.extensions.feedexport] INFO: Stored json feed (100 items) in: quotes.json

2021-10-26 07:35:39 [scrapy.statscollectors] INFO: Dumping Scrapy stats:
{'downloader/request_bytes': 55314,
 'downloader/request_count': 211,
 'downloader/request_method_count/GET': 211,
 'downloader/response_bytes': 271921,
 'downloader/response_count': 211,
 'downloader/response_status_count/200': 110,
 'downloader/response_status_count/308': 100,
 'downloader/response_status_count/404': 1,
 'elapsed_time_seconds': 2.544084,
 'feedexport/success_count/FileFeedStorage': 1,
 'finish_reason': 'finished',
 'finish_time': datetime.datetime(2021, 10, 26, 0, 35, 39, 790778),
 'httpcache/hit': 211,
 'httpcompression/response_bytes': 477498,
 'httpcompression/response_count': 110,
 'item_scraped_count': 100,
 'log_count/DEBUG': 312,
 'log_count/INFO': 20,
 'memusage/max': 57987072,
 'memusage/startup': 57987072,
 'request_depth_max': 10,
 'response_received_count': 111,
 'robotstxt/request_count': 1,
 'robotstxt/response_count': 1,
 'robotstxt/response_status_count/404': 1,
 'scheduler/dequeued': 210,
 'scheduler/dequeued/memory': 210,
 'scheduler/enqueued': 210,
 'scheduler/enqueued/memory': 210,
 'start_time': datetime.datetime(2021, 10, 26, 0, 35, 37, 246694)}
2021-10-26 07:35:39 [scrapy.core.engine] INFO: Spider closed (finished)
```

</details>
