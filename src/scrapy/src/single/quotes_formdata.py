import scrapy
from scrapy.http import FormRequest


class QuotesSpider(scrapy.Spider):
    name = "quotes"
    start_urls = ["http://quotes.toscrape.com/login"]

    def parse(self, response):
        return FormRequest.from_response(
            response,
            formdata={
                "username": "albattani",
                "password": "365days",
            },
            callback=self.scrape,
        )

    def scrape(self, response):
        for quote in response.css("div.quote"):
            yield {
                "author_name": quote.css("small.author::text").get(),
                "author_url": quote.css(
                    "small.author ~ a[href*='goodreads.com']::attr(href)"
                ).get(),
                "text": quote.css("span.text::text").get(),
                "tags": quote.css("div.tags a.tag::text").getall(),
            }

        next_page = response.css('li.next a::attr("href")').get()
        self.logger.info(f"::: Next page: {next_page}")
        if next_page is not None:
            yield response.follow(next_page, self.scrape)
