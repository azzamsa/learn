import scrapy


class QuotesScrollSpider(scrapy.Spider):
    name = "quotes_scroll_spider"
    api_url = "https://quotes.toscrape.com/api/quotes?page="
    start_urls = [f"{api_url}1"]

    def parse(self, response):
        page_data = response.json()
        for quote in page_data["quotes"]:
            yield {
                "author": quote["author"]["name"],
                "text": quote["text"],
                "tags": quote["tags"],
            }

        if page_data["has_next"]:
            next_page = page_data["page"] + 1
            yield response.follow(f"{self.api_url}{next_page}", callback=self.parse)
