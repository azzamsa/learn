import scrapy

from ..items import QuotesItem


class QuotesSpiderSpider(scrapy.Spider):
    name = "quotes_spider"
    start_urls = ["http://quotes.toscrape.com/"]

    def parse(self, response):
        for quote in response.css("div.quote"):
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

        next_page = response.css("li.next a::attr(href)").get()
        if next_page is not None:
            self.logger.info(f"Next page: {next_page}")
            yield response.follow(next_page, callback=self.parse)

    def parse_author(self, response, author_name, text, tags):
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
