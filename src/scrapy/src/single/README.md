# Single File Scraper

The collection of one single file scrapy scraper.

List of the scrapers:
- `quotes.py`: Scrape all the quotes from [quotes.toscrape.com](https://quotes.toscrape.com/). The most basic example.
- `quotes_author.py`: Scrape all the quotes and the author details. Featuring parsing nested page.
- `quotes_formdata.py`: Scrape all the quotes using `FormRequest` authentification.
- `quotes_scroll.py`: Scrape all the quotes from AJAX-based infinite scrolling pagination.
- `books_category.py`: Scrape all the books from [books.toscrape.com](https://books.toscrape.com/) via its category. Featuring three-level nested parsings.

## Getting Started

``` shell
$ # activate the virtualenv
$ vf activate global3 # in this case I am using virtualfish

# install dependencies
$ pip install -r requirements.txt

$ # start crawling
$ scrapy runspider quotes.py -O output/quotes.json

$ # other crawlers
$ scrapy runspider quotes_author.py -O output/quotes_author.json
$ scrapy runspider quotes_formdata.py -O output/quotes_formdata.json
$ scrapy runspider books_category.py -O output/books_category.json
```

The `quotes.json` will contain exactly 100 quotes. Otherwise, it doesn't work properly.

## Logs

Instead of putting the output into git, compare your log output to the log below.
To see if yours are correct.

Some important lines are:
- `Stored json feed (100 items)`. Check whether your crawled items are the same.
- `request_method_count/GET`.

<details>
  <summary>quotes.py</summary>

```python
2021-10-26 07:03:32 [scrapy.extensions.feedexport] INFO: Stored json feed (100 items) in: quotes.json

2021-10-26 07:03:32 [scrapy.statscollectors] INFO: Dumping Scrapy stats:
{'downloader/request_bytes': 2652,
 'downloader/request_count': 10,
 'downloader/request_method_count/GET': 10,
 'downloader/response_bytes': 23065,
 'downloader/response_count': 10,
 'downloader/response_status_count/200': 10,
 'elapsed_time_seconds': 5.565168,
 'feedexport/success_count/FileFeedStorage': 1,
 'finish_reason': 'finished',
 'finish_time': datetime.datetime(2021, 10, 26, 0, 3, 32, 194542),
 'httpcompression/response_bytes': 108561,
 'httpcompression/response_count': 10,
 'item_scraped_count': 100,
 'log_count/DEBUG': 110,
 'log_count/INFO': 11,
 'request_depth_max': 9,
 'response_received_count': 10,
```

</details>

<details>
  <summary>quotes_author.py</summary>

```python
2021-10-26 07:25:57 [scrapy.extensions.feedexport] INFO: Stored json feed (100 items) in: quotes_author.json

2021-10-26 07:25:57 [scrapy.statscollectors] INFO: Dumping Scrapy stats:
{'downloader/request_bytes': 59416,
 'downloader/request_count': 210,
 'downloader/request_method_count/GET': 210,
 'downloader/response_bytes': 264437,
 'downloader/response_count': 210,
 'downloader/response_status_count/200': 110,
 'downloader/response_status_count/308': 100,
 'elapsed_time_seconds': 9.131498,
 'feedexport/success_count/FileFeedStorage': 1,
 'finish_reason': 'finished',
 'httpcompression/response_bytes': 477498,
 'httpcompression/response_count': 110,
 'item_scraped_count': 100,
 'log_count/DEBUG': 310,
 'log_count/INFO': 11,
 'request_depth_max': 10,
 'response_received_count': 110,
```

</details>

<details>
  <summary>quotes_formdata.py</summary>

``` python
2021-11-06 06:54:44 [scrapy.extensions.feedexport] INFO: Stored json feed (100 items) in: quotes_formdata.json

2021-11-06 06:54:44 [scrapy.statscollectors] INFO: Dumping Scrapy stats:
{'downloader/request_bytes': 5240,
 'downloader/request_count': 12,
 'downloader/request_method_count/GET': 11,
 'downloader/request_method_count/POST': 1,
 'downloader/response_bytes': 26203,
 'downloader/response_count': 12,
 'downloader/response_status_count/200': 11,
 'downloader/response_status_count/302': 1,
 'elapsed_time_seconds': 5.875017,
 'feedexport/success_count/FileFeedStorage': 1,
 'finish_reason': 'finished',
 'finish_time': datetime.datetime(2021, 11, 5, 23, 54, 44, 955492),
 'httpcompression/response_bytes': 119041,
 'httpcompression/response_count': 11,
 'item_scraped_count': 100,
 'log_count/DEBUG': 112,
 'log_count/INFO': 22,
 'memusage/max': 57581568,
 'memusage/startup': 57581568,
 'request_depth_max': 10,
 'response_received_count': 11,
 'scheduler/dequeued': 12,
 'scheduler/dequeued/memory': 12,
 'scheduler/enqueued': 12,
 'scheduler/enqueued/memory': 12,
 'start_time': datetime.datetime(2021, 11, 5, 23, 54, 39, 80475)}
2021-11-06 06:54:44 [scrapy.core.engine] INFO: Spider closed (finished)
```

</details>


<details>
  <summary>books_category.py</summary>

``` python
2021-11-07 06:40:33 [scrapy.extensions.feedexport] INFO: Stored json feed (1000 items) in: output/books_category.json

2021-11-07 06:40:33 [scrapy.statscollectors] INFO: Dumping Scrapy stats:
{'downloader/request_bytes': 392955,
 'downloader/request_count': 1081,
 'downloader/request_method_count/GET': 1081,
 'downloader/response_bytes': 4446627,
 'downloader/response_count': 1081,
 'downloader/response_status_count/200': 1081,
 'elapsed_time_seconds': 50.523608,
 'feedexport/success_count/FileFeedStorage': 1,
 'finish_reason': 'finished',
 'finish_time': datetime.datetime(2021, 11, 6, 23, 40, 33, 683622),
 'httpcompression/response_bytes': 22569527,
 'httpcompression/response_count': 1081,
 'item_scraped_count': 1000,
 'log_count/DEBUG': 2081,
 'log_count/INFO': 1091,
 'memusage/max': 57569280,
 'memusage/startup': 57569280,
 'request_depth_max': 9,
 'response_received_count': 1081,
 'scheduler/dequeued': 1081,
 'scheduler/dequeued/memory': 1081,
 'scheduler/enqueued': 1081,
 'scheduler/enqueued/memory': 1081,
 'start_time': datetime.datetime(2021, 11, 6, 23, 39, 43, 160014)}
2021-11-07 06:40:33 [scrapy.core.engine] INFO: Spider closed (finished)
```

</details>


<details>
  <summary>quotes_scroll.py</summary>

``` python
2021-11-07 15:19:10 [scrapy.extensions.feedexport] INFO: Stored json feed (100 items) in: output/quotes_scroll.json

2021-11-07 15:19:10 [scrapy.statscollectors] INFO: Dumping Scrapy stats:
{'downloader/request_bytes': 2865,
 'downloader/request_count': 10,
 'downloader/request_method_count/GET': 10,
 'downloader/response_bytes': 14797,
 'downloader/response_count': 10,
 'downloader/response_status_count/200': 10,
 'elapsed_time_seconds': 6.074668,
 'feedexport/success_count/FileFeedStorage': 1,
 'finish_reason': 'finished',
 'finish_time': datetime.datetime(2021, 11, 7, 8, 19, 10, 821333),
 'httpcompression/response_bytes': 30904,
 'httpcompression/response_count': 10,
 'item_scraped_count': 100,
 'log_count/DEBUG': 110,
 'log_count/INFO': 11,
 'memusage/max': 57372672,
 'memusage/startup': 57372672,
 'request_depth_max': 9,
 'response_received_count': 10,
 'scheduler/dequeued': 10,
 'scheduler/dequeued/memory': 10,
 'scheduler/enqueued': 10,
 'scheduler/enqueued/memory': 10,
 'start_time': datetime.datetime(2021, 11, 7, 8, 19, 4, 746665)}
2021-11-07 15:19:10 [scrapy.core.engine] INFO: Spider closed (finished)
```

</details>
