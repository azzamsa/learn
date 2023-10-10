import scrapy


class QuotesSpider(scrapy.Spider):
    name = "quotes"
    start_urls = ["http://quotes.toscrape.com/"]

    def parse(self, response):
        for quote in response.css("div.quote"):
            yield {
                "author": quote.css("small.author::text").get(),
                "text": quote.css("span.text::text").get(),
                "tags": quote.css("div.tags a.tag::text").getall(),
            }

        next_page = response.css('li.next a::attr("href")').get()
        if next_page is not None:
            yield response.follow(next_page, self.parse)
