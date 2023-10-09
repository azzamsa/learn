import scrapy
from scrapy_splash import SplashRequest


class QuotesJsSpider(scrapy.Spider):
    name = "quotes_js"

    custom_settings = {
        "SPLASH_URL": "http://localhost:8050",
        "DOWNLOADER_MIDDLEWARES": {
            "scrapy_splash.SplashCookiesMiddleware": 723,
            "scrapy_splash.SplashMiddleware": 725,
            "scrapy.downloadermiddlewares.httpcompression.HttpCompressionMiddleware": 810,
        },
        "SPIDER_MIDDLEWARES ": {
            "scrapy_splash.SplashDeduplicateArgsMiddleware": 100,
        },
        "DUPEFILTER_CLASS": "scrapy_splash.SplashAwareDupeFilter",
    }

    def start_requests(self):
        yield SplashRequest("http://quotes.toscrape.com/js", callback=self.parse)

    def parse(self, response):
        for quote in response.css("div.quote"):
            yield {
                "author": quote.css("small.author::text").get(),
                "text": quote.css("span.text::text").get(),
                "tags": quote.css("div.tags a.tag::text").getall(),
            }

        next_url = response.css('li.next a::attr("href")').get()
        next_page = response.urljoin(next_url)
        self.logger.info(f"::: Next page: {next_page}")
        if next_page is not None:
            yield SplashRequest(next_page, self.parse)
