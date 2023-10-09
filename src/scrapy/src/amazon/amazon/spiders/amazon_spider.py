import scrapy


class AmazonSpiderSpider(scrapy.Spider):
    name = "amazon_spider"
    start_urls = [
        "https://www.amazon.com/s?k=algorithms&crid=281DGGHSJOMOE&sprefix=algo%2Caps%2C564"
    ]

    def parse(self, response):
        products = response.css(".s-card-border")
        for product in products:
            title = product.css(".a-size-base.a-color-base.a-text-normal::text").get()
            author = product.css(
                ".a-size-base.a-color-base.a-link-normal.s-underline-text.s-underline-link-text::text"
            ).get()

            yield {"title": title, "author": author}
