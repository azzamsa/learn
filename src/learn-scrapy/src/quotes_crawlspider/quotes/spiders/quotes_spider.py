from scrapy.linkextractors import LinkExtractor
from scrapy.spiders import CrawlSpider, Rule

from ..items import QuotesItem


class QuotesSpiderSpider(CrawlSpider):
    name = "quotes_spider"
    start_urls = ["http://quotes.toscrape.com/"]
    rules = (
        Rule(
            LinkExtractor(allow=("page/",), deny=("tag/",)),
            callback="parse_page",
            follow=True,
        ),
    )

    def parse_page(self, response):
        """Gathers the author details.

        @url https://quotes.toscrape.com/page/1/
        @returns items 0 1
        @returns request 10 10
        @scrapes author_name author_date author_location text tags
        """
        for quote in response.css("div.quote"):
            self.logger.info(f"::: Current URL: {response.url}")

            author_name = quote.css("small.author::text").get()
            text = quote.css("span.text::text").get()
            tags = quote.css("div.tags a.tag::text").getall()

            author_page = quote.css(".author + a ::attr(href)").get()
            yield response.follow(
                author_page,
                callback=self.parse_author,
                dont_filter=True,
                cb_kwargs={"author_name": author_name, "text": text, "tags": tags},
            )

    def parse_author(self, response, author_name, text, tags):
        """Gathers the author details.

        @url https://quotes.toscrape.com/author/Harper-Lee/
        @cb_kwargs {"author_name": "author_name", "text": "text", "tags": "tags"}
        @returns items 1 1
        @returns request 0 1
        @scrapes author_name author_date author_location text tags
        """
        item = QuotesItem()

        author_date = response.css(".author-born-date::text").get()
        location = response.css(".author-born-location::text").get()
        author_location = location.replace("in ", "")  # strip `in`

        item["author_name"] = author_name
        item["author_date"] = author_date
        item["author_location"] = author_location
        item["text"] = text
        item["tags"] = tags

        return item
