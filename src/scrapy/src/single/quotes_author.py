import scrapy


class QuotesSpider(scrapy.Spider):
    name = "quotes_author"
    start_urls = ["http://quotes.toscrape.com/"]

    def parse(self, response):
        for quote in response.css("div.quote"):
            author = quote.css("small.author::text").get()
            text = quote.css("span.text::text").get()
            tags = quote.css("div.tags a.tag::text").getall()

            author_page = quote.css(".author + a ::attr(href)").get()
            yield response.follow(
                author_page,
                callback=self.parse_author,
                dont_filter=True,
                cb_kwargs={"author": author, "text": text, "tags": tags},
            )

        next_page = response.css('li.next a::attr("href")').get()
        if next_page is not None:
            yield response.follow(next_page, self.parse)

    def parse_author(self, response, author, text, tags):
        date = response.css(".author-born-date::text").get()
        location = response.css(".author-born-location::text").get()
        location = location.replace("in ", "")  # strip `in`

        yield {
            "author": author,
            "date": date,
            "location": location,
            "text": text,
            "tags": tags,
        }
