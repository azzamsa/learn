# Guide

Starting a new project.

``` shell
scrapy startproject quotes
```

Minimal working example.

``` python
import scrapy

class QuotesSpider(scrapy.Spider):
    name = "quotes"
    start_urls = [
        'http://quotes.toscrape.com/page/1/',
        'http://quotes.toscrape.com/page/2/',
    ]

    def parse(self, response):
        page = response.url.split("/")[-2]
        filename = f'quotes-{page}.html'
        with open(filename, 'wb') as f:
            f.write(response.body)
```

`start_urls` is a shortcut to ` start_requests()` function.

Run the spider using:

``` shell
scrapy crawl quotes
```

## Scrapy Shell

To have a playground for selector, use Scrapy shell:

``` shell
scrapy shell 'http://quotes.toscrape.com/page/1/'
```

``` shell
>>> response.css('title')
[<Selector xpath='descendant-or-self::title' data='<title>Quotes to Scrape</title>'>]

>>> # `::text` is scrapy extension to css selector
>>> response.css('title::text').get()
'Quotes to Scrape'

>>> # you can use regex too
>>> response.css('title::text').re(r'Quotes.*')
['Quotes to Scrape']
```

## Selector

A selector is a mechanism for extracting data, either by XPath or CSS expressions.
Scrapy uses `parsel` under the hood, which in turn uses `lxml`. A popular python XML parsing library.

``` python
>>> response.xpath('//span/text()').get()
'good'

>>> response.css('span::text').get()
'good'
```

`.get()` always returns a single result; if there are several matches, the content of
a first match is returned; if there are no matches, `None` is returned. `.getall()`
returns a list with all results.

To avoid having `None` from `.get()`, use `default=''`

``` python
>>> response.css('img::text').get(default='')
```

📝 `.get()` and `.getall()` is the new scrapy method that are equivalent to
`.extract_first()` and `.extract()`. The difference is that the former always
return predictable result, single and list.

## CSS Selector

``` python
>>> response.css('title').getall()
['<title>Quotes to Scrape</title>']

>>> response.css('title::text').getall()
['Quotes to Scrape']
```

The basics of CSS selectors are tags, class, id, and attribute.
Tags selected using its name, such `p` and `h1`. Class selected using a dot, id using a hashtag, and attribute using `.attrib`.

Some examples:
- `div.quote`: select `quote` class in `div` tag.
- `div.tags a.tag`: space means a descendant. Select a `tag` class in `a` tag under `div.tags`.

Other than using Scrapy shell to find an item using CSS selector, [SelectorGadget](https://selectorgadget.com/) is a handy visual alternative.

## Extensions to CSS Selectors

📝 Take a note that as W3C standards, CSS selectors do not support selecting text
nodes or attribute values. `::text` and `::attr(name)` is [custom Scrapy
(parsel) extension][css-extension] to CSS selectors.  It will not work with
other libraries like `lxml` or `PyQuery`.

- `title::text` Selects children text nodes.
- `*::text` Selects all descendant text nodes.
- `a::attr(href)` Selects the href attribute value.

## XPath Selector

``` python
>>> response.xpath('//title/text()').get()
'Quotes to Scrape'
```

In contrast to Scrapy custom CSS selector such `::attr(...)`, XPath has the
same built-in feature.

``` python
>>> response.xpath("//a/@href").getall()
['image1.html', 'image2.html', 'image3.html', 'image4.html', 'image5.html']

>>> response.css('a::attr(href)').getall()
['image1.html', 'image2.html', 'image3.html', 'image4.html', 'image5.html']

>>> # if you not prefer both, use python `attrib`
>>> [a.attrib['href'] for a in response.css('a')]
['image1.html', 'image2.html', 'image3.html', 'image4.html', 'image5.html']
```

Some Xpath expression:

- `/foo/bar/baz`: Select all `baz` element that are childern of `/foo/bar`.
- `//foo/bar/baz`: Select all `baz` element that are childern of `/bar`.
- `/foo/bar/*`: Select all element under `/foo/bar`.
- `/foo/bar/baz[1]`: Select the first `baz` child of element `/foo/bar`.
- `/foo/bar/baz[last]`: Select the first `last` child of element `/foo/bar`.
- `//bar[@id='bar1']`: Select all `bar` element which attribute id is `bar1`.
  - other attribute value selector are: `@name`.
- `//*[count(bar)=2]`: Select element that has two `bar` children.
- `//*[count(*)=2]`: selecet any element that has two children.
- `//*[name()='bar']`: Select any `bar` element. Simmiliar with `//bar`.
- `//*[string-length(name()) = 3]`: Select any element with 3 letter character.
  - we can use other comparison here: `<`, `>`.
- `/foo/bar/descendant::*`: Select all descendant of `/foo/bar`.
- `//foo/descendant::*`: Select all elements which have `foo` among *its ancestors*.
- `//baz/parent::*`: Select all parents of `baz` element.
- `/foo/bar/baz/ancestor::*`: Select all acestor of `baz`.
- `/foo/bar/following-sibling::*`: Select following sibling of `bar`.
- `/foo/bar/preceding-sibling::*`: Select preceding sibling of `bar`.
- `/foo/bar/following::*`: Select the axis after `bar`.
- `/foo/bar/preceding::*`: Select the axis before `bar`.
- `/foo/bar/descendant-or-self::*`: Select the context node and the descendant.
- `/foo/bar/ancestor-or-self::*`: Select the context node and the ancestor.

Other interesting functions are:

- `//bar[normalize-space(@name)='b1']`: remove leading and trailing spaces before comparison
- `//*[starts-with(name(),'b')]`: Select any element *starts* with b.
- `//*[contains(name(),'b')]`: Select any element which *contain* c.
- `//bar[position() mod 2 = 0 ]`: Select even `bar` elements.

The expression also can be combined using `|`:

- `//foo | //bar`: Select all element matching both expression

📝 At this point you realize that using XPath is too verbose. Scrapy selectors
allow you to chain selectors, so most of the time you can just select by class
using CSS and then switch to XPath when needed.

Most of the time, we will be working with relative XPath.
Here are some tips:

- To get the `<p>` element inside `<div>`:

``` python
>>> divs = response.xpath('//div')
>>> for p in divs.xpath('.//p'):  # without the '.', it will get the `<p>` from whole document
    print(p.get())
```

## Regular Expressions Selector

``` python
>>> response.xpath('//a[contains(@href, "image")]/text()').re(r'Name:\s*(.*)')
['My image 1',
 'My image 2',
 'My image 3',
 'My image 4',
 'My image 5']
```

To get the first matching result, use `.re_first()`.

## Storing the scraped data

Scrapy can export the data to a local or remote file with several formats. Such
as JSON, JSON lines, CSV, and XML. Currently supported remote storage are FTP,
S3 and Google Cloud Storage (GCS).

Now we store the result in a local file with JSON format:

``` shell
scrapy crawl quotes -O quotes.json
```

## Following links

To follow the links, get the next-page link using a selector, then build a
absolute link using `urljoin(url)`.

`response.follow` is a shortcut for `scrapy.Request()` that automatically build
the absolute URL for you.

Other than automatic absolute URL builder, it is also able to receive a selector as an argument.

``` python
for href in response.css('ul.pager a::attr(href)'):
    yield response.follow(href, callback=self.parse)
```

and able to extract `href` automatically.

``` python
for a in response.css('ul.pager a'):
    yield response.follow(a, callback=self.parse)
```

To create multiple requests for iterable, use `follow_all()`.



## Reference

- [Official Scrapy Tutorial](https://docs.scrapy.org/en/latest/intro/tutorial.html)
- [Zvon.org XPath 1.0 Tutorial](http://www.zvon.org/comp/r/tut-XPath_1.html#intro)
- [Scrapy Selector Documentation](https://docs.scrapy.org/en/latest/topics/selectors.html)

[scrapy-tutorial]: https://docs.scrapy.org/en/latest/intro/tutorial.html
[css-extension]: https://docs.scrapy.org/en/latest/topics/selectors.html#extensions-to-css-selectors
