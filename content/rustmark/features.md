# ✨ Features

This is an example Markdown document. It demonstrates the range of Markdown
features available, and how to use them.


## Standards and compatibility

The Markdown files added to this repository will be rendered in full compliance
with [CommonMark](https://commonmark.org/) [0.30](https://spec.commonmark.org/0.30/)
as a bare minimum. Some extras such as [tables](#tables), [task lists](#task-lists),
[strikethrough](#strikethrough), and [autolinks](#links) are not supported by
CommonMark, and are instead rendered using the
[GitHub-flavoured Markdown specification](https://github.github.com/gfm/).

For reference, the Markdown parser used is [Comrak](https://github.com/kivikakk/comrak#extensions),
which implements [superscript](#superscript), [header IDs](#headings),
[footnotes](#footnotes), [description lists](#description-lists), and
[shortcodes](#emoji-shortcodes) in addition to the features standardised in the
GitHub-flavoured Markdown specification. These are enabled, but may not work
elsewhere if other Markdown parsers are used.

In addition, [callouts](#callouts) have been implemented using an extension to
the [blockquotes](#blockquotes) syntax.


## Syntax

### Headings

*Quick reference:* `# h1`, `## h2`, `### h3`, `#### h4`, `##### h5`, `###### h6`

As can be seen throughout this document, there is a range of headings available,
from `h1` to `h6`. These are denoted by the number of `#` characters at the
start of the line.

For example, `# Heading 1` is an `h1` heading, and `###### Heading 6` is an `h6`
heading.

> **Note**
> The first `h1` heading in a Markdown document is used as the title of the
> page, if it is also the first piece of content in the file. It is recommended
> that there should only be one `h1` heading per document.

#### Heading ids

Headings are automatically assigned an ID, which is the text of the heading
converted to lowercase, with spaces replaced by hyphens. This is used to create
anchors for the headings, which can be linked to using the `#` character in a
URL.

For example, the ID of this heading is `headings`, so it can be linked to using
`#headings`: [Headings](#headings).

A small `` icon is displayed next to each heading when the mouse hovers over it,
which can be used to copy the link to the heading.

#### Collapsing headings

Headings will show a small arrow icon next to them when the mouse hovers over
them. Clicking on this icon will collapse the heading, hiding the content below
it. Clicking on the icon again will expand the heading, showing the content
below it again.

#### Examples

##### Heading 5

###### Heading 6

### Line breaks

*Quick reference:* `\ `

Actual line breaks are used to hard-wrap text, to make it easier to read in the
source text. They are not rendered in the output, so they can be used to break
up paragraphs without creating a new paragraph.

Most of the time you want to create an actual line break, you will actually be
wanting to create a new paragraph. This is done by adding two newlines between
the lines of text.

If you really do want to add a single newline between lines of text without
creating a new paragraph, you can use two or more trailing spaces `  `, or a
trailing backslash `\ `. The other way to do this is to use HTML tags, such as
`<br />`, but this is not recommended.

The trailing spaces approach, although it works, is not recommended because it
is not obvious what is going on, and editors often strip trailing whitespace
automatically. The trailing backslash approach is therefore better.

#### Examples

Here's a line for us to start with.

This line is separated from the one above by two newlines, so it will be a
*separate paragraph*.

This line is also a separate paragraph, but...
This line is only separated by a single newline, so it's a *continuation* of the
line before, in the *same paragraph*.

This line is another separate paragraph, but...\
This line is separated by a trailing backslash `\ `, so it's a *separate line*
in the *same paragraph*.

### Emphasis

*Quick reference:* `*italic*`, `_italic_`, `**bold**`, `__bold__`

Emphasis can be added to text by surrounding it with `*` or `_` characters. A
single character of either type will make the text italic, and two, bold.

For example, `*This text will be italic*` will be rendered as *This text will be
italic*. Similarly, `**This text will be bold**` will be rendered as **This text
will be bold**.

You can also combine the two, for example `_You **can** combine them_` will be
rendered as _You **can** combine them_.

#### Examples

  - *This text will be italic*
  - _This text will also be italic_
  - **This text will be bold**
  - __This text will also be bold__
  - _You **can** combine them_
  - **You _can_ combine them**

### Strikethrough

*Quick reference:* `~~strikethrough~~`, `~strikethrough~`

Strikethrough text can be added by surrounding it with `~~` characters. Both one
and two characters will work, although using two is more widely-supported.

For example, `~~This is an example~~` will be rendered as
~~This is an example~~.

#### Examples

  - ~~This is an example~~
  - ~This is an example~

### Superscript

*Quick reference:* `^2^`

Superscript text can be added by surrounding it with `^` characters.

For example, `2^nd^` will be rendered as 2^nd^. This is particularly useful for
mathematical notation, such as `e=mc^2^`.

Note that this may not work correctly with other Markdown parsers.

#### Examples

  - 2^nd^
  - e=mc^2^

### Lists

*Quick reference:* `* item`, `+ item`, `- item`, `1. item`, `2. item`, `3. item`

There are two types of lists: unordered and ordered. Unordered lists use `*`,
`+`, or `-` characters as bullets, and ordered lists use numbers.

#### Examples

**Unordered**

  - Item 1
  - Item 2
  - Item 3
      - Item 2a
      - Item 2b

**Ordered**

  1. Item 1
  2. Item 2
  3. Item 3
       1. Item 3a
       2. Item 3b

### Task lists

*Quick reference:* `- [x]`, `- [ ]`

Task lists can be added to Markdown documents using the following syntax:

```markdown
- [x] Write the press release
- [ ] Update the website
- [ ] Contact the media
```

They required list syntax, either unordered or ordered, and the `[ ]` and `[x]`
characters to represent incomplete and complete items, respectively.

#### Examples

- [x] [links](), **formatting**, and <del>tags</del> are supported
- [ ] @mentions and #refs are not currently supported
- [x] List syntax required (any unordered or ordered list supported)
- [x] This is a complete item
- [ ] This is an incomplete item

### Description lists

*Quick reference:* `term` + `: definition`

Description lists can be added to Markdown documents using the following syntax:

```markdown
Term 1

:   This is a definition for term 1.

Term 2 with *inline markup*

:   This is a definition for term 2, which is longer than the first term.

    This is a second paragraph within the definition of term 2.
```

This will get rendered as show in the example below.

They require a term and a definition, and each definition can contain multiple
paragraphs and other block-level content.

#### Examples

Term 1

:   This is a definition for term 1.

Term 2 with *inline markup*

:   This is a definition for term 2, which is longer than the first term.

    This is a second paragraph within the definition of term 2.

### Images

*Quick reference:* `![Alt text](url)`

Images can be added to Markdown documents using the following syntax: `![Alt
text](url)`.

For example, `![Ferris](images/rustacean-flat-happy.png)` will be rendered as
in the example below.

> **Tip**
> If you have a large image and want to hide it away from the main document
> flow, you may wish to use [image callouts](#callouts), which are collapsible,
> or a [details block](#details-and-summary).

#### Examples

![Ferris](images/rustacean-flat-happy.png)

Source: [rustacean.net](https://rustacean.net/) *(Public Domain)*

### Links

*Quick reference:* `[Link text](url)`

Links can be added to Markdown documents using the following syntax: `[Link
text](url)`.

For example, `[Rust](https://www.rust-lang.org/)` will be rendered as [Rust](https://www.rust-lang.org/).

You can also include links on their own without Markdown syntax, and they will
be automatically converted to links where possible. For example,
`https://www.rust-lang.org/` will be rendered as https://www.rust-lang.org/.
However, it is always safest, and therefore advisable, to use the explicit
syntax.

#### Examples

  - [Rust](https://www.rust-lang.org/)
  - https://www.rust-lang.org/
  - www.rust-lang.org

### Blockquotes

*Quick reference:* `> quote`

Blockquotes can be added to Markdown documents using the `>` character at the
start of the line.

For example, `> This is a blockquote` will be rendered as:

> This is a blockquote

> **Tip**
> Other Markdown elements can be included within blockquotes, including other
> blockquotes.

> **Tip**
> There are two additional Markdown features implemented that extend the basic
> blockquote syntax: [callouts](#callouts), and [details and summary](#details-and-summary).

#### Examples

As Albert Einstein said:

> I have no special talent.
> I am only passionately curious.

> This blockquote contains other Markdown elements.
> 
> Some **bold text**, some `inline code`.
> 
> ```
> A code block
> ```
> 
> - A list item
> 
> > A blockquote within a blockquote.

### Details and summary

*Quick reference:* `>-> details` + `> summary`

HTML provides a [`<details>` element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details)
for creating a disclosure widget from a block of text, and a
[`<summary>` element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/summary)
for the title of the disclosure widget. Information within the `<details>`
element is hidden by default.

Standard Markdown does not provide a syntax for these elements, but they can be
added to Markdown documents using raw HTML.

For example, the following HTML:

```html
<details>
    <summary>Summary of the details</summary>
    <p>More details about the thing that was summarised.</p>
</details>
```

...will be rendered as:

<details>
    <summary>Summary of the details</summary>
    <p>Details about the thing that was summarised.</p>
</details>

Rustmark provides a shorthand syntax for this, which is more concise and
readable, as an extension to blockquote syntax. The following Markdown is
equivalent to the HTML above:

```markdown
>-> Summary of the details
> Details about the thing that was summarised.
```

```markdown
> -> Another summary
> More details.
```

> **Note**
> You can use either `> ->` or `>->` as the start of the blockquote to indicate
> that it is a details block. The space is optional, but may be preferred for
> readability.

> **Note**
> The summary may span multiple lines, in which case the `->` prefix should be
> repeated on each line. When the prefix ceases to be repeated, the details
> block will start. Leaving a blank line between the summary and the details
> block will lead to a larger gap, which may be desirable in some cases.

The decision then becomes one of whether to use the standard HTML syntax, or
the Rustmark shorthand syntax. If using another platform to preview the
Markdown on a regular basis then the HTML syntax is recommended, as it will be
rendered correctly everywhere. If using Rustmark to render the Markdown then
the shorthand syntax is recommended, as it is more concise and readable.

> **Tip**
> Other Markdown elements can be included within details and summary blocks,
> including other details and summary blocks.

#### Examples

>-> This is the summary
> These are the details.

> -> This will also work
> 
> These are more details, with a gap.

> -> Another summary — this one is *longer*,
> -> and spans multiple lines
> This details block contains other Markdown elements.
>
> Some **bold text**, some `inline code`.
> 
> ```
> A code block
> ```
> 
> - A list item
> 
> > A blockquote within a details block.
> 
> > -> A details block within a details block.
> > The nested details.

### Callouts

*Quick reference:* `> **Warning**`, `> **Note**`, `> **Info**`, etc.

You can use callouts where appropriate, e.g. to call attention to special notes,
warnings, etc. as per the styles below. They are an extended form of blockquote,
as this is a simple approach and degrades gracefully if rendered without
specific support. You can see them in use throughout this document.

Blockquotes are taken to be callouts if they start with a single, specific word
in bold text, such as `Warning`, i.e. `> **Warning**`. Punctuation will be
ignored, so `Warning!` and `Warning:` are both valid. The following callout
types are supported by default, some of which can be triggered by multiple
words:

  - Blocked / Broken
  - Warning / Caution
  - Note / Tip
  - Complete / Finished / Approved
  - Info
  - Question
  - Todo
  - Image(s) / Screenshot(s)

It is advisable to place the emphasised word on a line of its own, but this is
not strictly necessary. It just makes the Markdown easier to read in some cases.

> **Tip**
> Other Markdown elements can be included within callouts, including other
> callouts.

#### Image titles

Notably, the image/screenshot callouts will be made collapsible, and also
support showing a title alongside the callout heading. This is done by adding
a colon and then the title, on the same line as the callout heading, e.g.
`> **Screenshot**: Ferris's day off`.

If found, the title will continue until a new paragraph is indicated by a blank
line in the usual fashion. This has to work this way because there may be other
elements in the title, such as styled text or icons. The title will then be
shown even when the callout is collapsed, giving a hint as to what it contains.

If there is no title then the callout type name will be extracted without
affecting anything else.

> **Warning**
> When using a title, it is critical to add a blank blockquote line after the
> title and before the image that is considered to be content. Otherwise the
> image will be part of the same paragraph as the title.

> **Broken**
> A [bug in Firefox](https://bugzilla.mozilla.org/show_bug.cgi?id=418039#c56)
> means that the arrows will not work at present. This is a known issue, and
> will be fixed in a future release of Firefox. There is no such issue in
> Chrome and Chrome-based browsers.

#### Examples

Here are the callouts in action:

> **Blocked**
> This is an important note that there is an unresolved obstacle. You can also
> use "Broken".

> **Warning!**
> This is a warning note that requires attention. You can also use "Caution".

> **Note**
> This is a standard note to draw attention to something. You can also use "Tip".

> **Complete**
> This is a note to say something notable has been completed or achieved. You
> can also use "Finished" and "Approved".

> **Info**
> This is general background information.

> **Question**
> This is an unresolved question.

> **Todo**
> This is a note about something unfinished that remains to be done.

> **Image**: Friendly Ferris
>
> This is collapsible, and will be collapsed by default.
> ![Ferris](images/rustacean-flat-happy.png)

> **Screenshot**
> This is also collapsible, and will also be collapsed by default.
> ![Ferris](images/rustacean-flat-happy.png)

They can contain multiple paragraphs, and other Markdown elements:

> **Note**
> This note is quite long on purpose, to show how the wrapping behaves. We'll
> put some more filler text here, to illustrate this fully.
>
> This callout contains other Markdown elements.
>
> Some **bold text**, some `inline code`.
> 
> ```
> A code block
> ```
> 
> - A list item
> 
> > A blockquote within a callout.
> 
> > **Info**
> > This is a nested callout.
>
> **And...**
> It also has another piece of bold text, in the same style as the callout
> heading, to show how that behaves.

Image titles can contain formatting and other Markdown elements:

> **Image**: Even *more* Ferris
>
> ![Ferris](images/rustacean-flat-happy.png)
> 
> > **Info**
> > This is a nested callout.

Callouts don't have to have any content at all:

> **Complete**

### Inline code

*Quick reference:* `` `code` ``

Inline code can be added to Markdown documents using the backtick `` ` ``
character surrounding the code.

For example, `` `code` `` will be rendered as `code`.

Inline code supports Font Awesome icons and the other icons included in the Nerd
Font range of fonts.

#### Examples

  - In Rust, you can use the `?` operator to easily propagate errors.
  - The `main()` function is the entry point into a Rust program.
  - You can use Nerd Font icons in your code, such as ``.

### Code blocks

*Quick reference:* ```` ``` ``` ````, ```` ~~~ ~~~ ````

Code blocks can be added to Markdown documents using three backtick `` ` ``
characters on the line before and after the code block. Alternatively, you can
use three tilde `~` characters.

Code blocks support Font Awesome icons and the other icons included in the Nerd
Font range of fonts.

#### Syntax highlighting

Code blocks can have their syntax highlighted. To do this, add the name of the
language immediately after the opening code block characters.

For example, ```` ```rust ```` will highlight the code block according to the
Rust syntax.

#### Examples

```
//   Code blocks support Font Awesome icons and the other icons included in the
//  Nerd Font range of fonts.
```

**JavaScript**
```javascript
function fancyAlert(arg) {
  if(arg) {
    $.facebox({div:'#foo'})
  }
}
```

**Rust**
```rust
fn main() {
    println!("Hello, world!");
}
```

**Python**
```python
def foo():
    if not bar:
        return True
```

### Horizontal rule

*Quick reference:* `---`

A horizontal rule can be added to Markdown documents using three or more `-`
characters on a line by themselves.

For example, `---` will be rendered as per the example below.

#### Examples

---

### Tables

*Quick reference:* `| Header | Header |` + `| --- | --- |` + `| Row | Row |`

Tables can be added to Markdown documents using the following syntax:

```markdown
| Header | Title |
| --- | --- |
| Paragraph | Text |
```

#### Examples

| Syntax      | Description |
| ----------- | ----------- |
| Header      | Title       |
| Paragraph   | Text        |

### Footnotes

*Quick reference:* `[^1]`

Footnotes can be added to Markdown documents using the following syntax:

```markdown
Here is a footnote reference[^1], and another[^longnote].

[^1]: Here is the footnote.
[^longnote]: Here's one with multiple blocks.

    Subsequent paragraphs are indented to show that they
    belong to the previous footnote.

    The whole paragraph can be indented, or just the first
    line. In this way, multi-paragraph footnotes work like
    multi-paragraph list items.
```

This will get rendered as show in the example below.

#### Examples

Here is a footnote reference[^1], and another[^longnote].

[^1]: Here is the footnote.
[^longnote]: Here's one with multiple blocks.

    Subsequent paragraphs are indented to show that they
    belong to the previous footnote.

    The whole paragraph can be indented, or just the first
    line. In this way, multi-paragraph footnotes work like
    multi-paragraph list items.

### Unicode symbols

Unicode symbols can be used in Markdown documents. For example, `✨` will be
rendered as ✨. Quite often these can be preferable to using [Emoji shortcodes](#emoji-shortcodes),
which are not always supported by all Markdown renderers.

#### Examples

  - 👾

### Emoji shortcodes

*Quick reference:* `:shortcode:`

Emoji shortcodes can be used in Markdown documents. For example, `:sparkles:`
will be rendered as :sparkles:.

For a complete list, see the [Emojis reference](#emojis) below.

#### Examples

  - This PR looks great :thumbsup: — it's ready to merge! :rocket: :smile:
  - :sparkles: :camel: :boom:

### HTML

It is possible to use raw HTML in your Markdown documents. However, this should
be used sparingly, as it is not supported by all Markdown renderers.

#### Examples

<dl>
  <dt>Definition list</dt>
  <dd>Is something people use sometimes.</dd>

  <dt>Markdown in HTML</dt>
  <dd>Does *not* work. Use HTML <em>tags</em>.</dd>
</dl>


## Reference

### Emojis

The GitHub emoji list is here:

  - https://github.com/github/gemoji/blob/master/db/emoji.json

To extract emojis:

```bash
i=1
for x in $(\
    curl https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json \
    | jq '.[]."emoji"'
    | sed 's/"//g'\
); do
    echo -n "$x "
    i=$((i+1))
    if [[ $i -gt 20 ]]; then
        echo
        i=1
    fi
done
```

To extract shortcodes:

```bash
i=1
for x in $(\
    curl https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json \
    | jq '.[]."aliases"[]'
    | sed 's/"/:/g'\
); do
    echo -n "$x "
    i=$((i+1))
    if [[ $i -gt 20 ]]; then
        echo
        i=1
    fi
done
```

#### Glyphs

Below is a list of all the glyphs that can be used in Markdown documents.

😀 😃 😄 😁 😆 😅 🤣 😂 🙂 🙃 🫠 😉 😊 😇 🥰 😍 🤩 😘 😗 ☺️
😚 😙 🥲 😋 😛 😜 🤪 😝 🤑 🤗 🤭 🫢 🫣 🤫 🤔 🫡 🤐 🤨 😐 😑
😶 🫥 😶‍🌫️ 😏 😒 🙄 😬 😮‍💨 🤥 🫨 😌 😔 😪 🤤 😴 😷 🤒 🤕 🤢 🤮
🤧 🥵 🥶 🥴 😵 😵‍💫 🤯 🤠 🥳 🥸 😎 🤓 🧐 😕 🫤 😟 🙁 ☹️ 😮 😯
😲 😳 🥺 🥹 😦 😧 😨 😰 😥 😢 😭 😱 😖 😣 😞 😓 😩 😫 🥱 😤
😡 😠 🤬 😈 👿 💀 ☠️ 💩 🤡 👹 👺 👻 👽 👾 🤖 😺 😸 😹 😻 😼
😽 🙀 😿 😾 🙈 🙉 🙊 💌 💘 💝 💖 💗 💓 💞 💕 💟 ❣️ 💔 ❤️‍🔥 ❤️‍🩹
❤️ 🩷 🧡 💛 💚 💙 🩵 💜 🤎 🖤 🩶 🤍 💋 💯 💢 💥 💫 💦 💨 🕳️
💬 👁️‍🗨️ 🗨️ 🗯️ 💭 💤 👋 🤚 🖐️ ✋ 🖖 🫱 🫲 🫳 🫴 🫷 🫸 👌 🤌 🤏
✌️ 🤞 🫰 🤟 🤘 🤙 👈 👉 👆 🖕 👇 ☝️ 🫵 👍 👎 ✊ 👊 🤛 🤜 👏
🙌 🫶 👐 🤲 🤝 🙏 ✍️ 💅 🤳 💪 🦾 🦿 🦵 🦶 👂 🦻 👃 🧠 🫀 🫁
🦷 🦴 👀 👁️ 👅 👄 🫦 👶 🧒 👦 👧 🧑 👱 👨 🧔 🧔‍♂️ 🧔‍♀️ 👨‍🦰 👨‍🦱 👨‍🦳
👨‍🦲 👩 👩‍🦰 🧑‍🦰 👩‍🦱 🧑‍🦱 👩‍🦳 🧑‍🦳 👩‍🦲 🧑‍🦲 👱‍♀️ 👱‍♂️ 🧓 👴 👵 🙍 🙍‍♂️ 🙍‍♀️ 🙎 🙎‍♂️
🙎‍♀️ 🙅 🙅‍♂️ 🙅‍♀️ 🙆 🙆‍♂️ 🙆‍♀️ 💁 💁‍♂️ 💁‍♀️ 🙋 🙋‍♂️ 🙋‍♀️ 🧏 🧏‍♂️ 🧏‍♀️ 🙇 🙇‍♂️ 🙇‍♀️ 🤦
🤦‍♂️ 🤦‍♀️ 🤷 🤷‍♂️ 🤷‍♀️ 🧑‍⚕️ 👨‍⚕️ 👩‍⚕️ 🧑‍🎓 👨‍🎓 👩‍🎓 🧑‍🏫 👨‍🏫 👩‍🏫 🧑‍⚖️ 👨‍⚖️ 👩‍⚖️ 🧑‍🌾 👨‍🌾 👩‍🌾
🧑‍🍳 👨‍🍳 👩‍🍳 🧑‍🔧 👨‍🔧 👩‍🔧 🧑‍🏭 👨‍🏭 👩‍🏭 🧑‍💼 👨‍💼 👩‍💼 🧑‍🔬 👨‍🔬 👩‍🔬 🧑‍💻 👨‍💻 👩‍💻 🧑‍🎤 👨‍🎤
👩‍🎤 🧑‍🎨 👨‍🎨 👩‍🎨 🧑‍✈️ 👨‍✈️ 👩‍✈️ 🧑‍🚀 👨‍🚀 👩‍🚀 🧑‍🚒 👨‍🚒 👩‍🚒 👮 👮‍♂️ 👮‍♀️ 🕵️ 🕵️‍♂️ 🕵️‍♀️ 💂
💂‍♂️ 💂‍♀️ 🥷 👷 👷‍♂️ 👷‍♀️ 🫅 🤴 👸 👳 👳‍♂️ 👳‍♀️ 👲 🧕 🤵 🤵‍♂️ 🤵‍♀️ 👰 👰‍♂️ 👰‍♀️
🤰 🫃 🫄 🤱 👩‍🍼 👨‍🍼 🧑‍🍼 👼 🎅 🤶 🧑‍🎄 🦸 🦸‍♂️ 🦸‍♀️ 🦹 🦹‍♂️ 🦹‍♀️ 🧙 🧙‍♂️ 🧙‍♀️
🧚 🧚‍♂️ 🧚‍♀️ 🧛 🧛‍♂️ 🧛‍♀️ 🧜 🧜‍♂️ 🧜‍♀️ 🧝 🧝‍♂️ 🧝‍♀️ 🧞 🧞‍♂️ 🧞‍♀️ 🧟 🧟‍♂️ 🧟‍♀️ 🧌 💆
💆‍♂️ 💆‍♀️ 💇 💇‍♂️ 💇‍♀️ 🚶 🚶‍♂️ 🚶‍♀️ 🧍 🧍‍♂️ 🧍‍♀️ 🧎 🧎‍♂️ 🧎‍♀️ 🧑‍🦯 👨‍🦯 👩‍🦯 🧑‍🦼 👨‍🦼 👩‍🦼
🧑‍🦽 👨‍🦽 👩‍🦽 🏃 🏃‍♂️ 🏃‍♀️ 💃 🕺 🕴️ 👯 👯‍♂️ 👯‍♀️ 🧖 🧖‍♂️ 🧖‍♀️ 🧗 🧗‍♂️ 🧗‍♀️ 🤺 🏇
⛷️ 🏂 🏌️ 🏌️‍♂️ 🏌️‍♀️ 🏄 🏄‍♂️ 🏄‍♀️ 🚣 🚣‍♂️ 🚣‍♀️ 🏊 🏊‍♂️ 🏊‍♀️ ⛹️ ⛹️‍♂️ ⛹️‍♀️ 🏋️ 🏋️‍♂️ 🏋️‍♀️
🚴 🚴‍♂️ 🚴‍♀️ 🚵 🚵‍♂️ 🚵‍♀️ 🤸 🤸‍♂️ 🤸‍♀️ 🤼 🤼‍♂️ 🤼‍♀️ 🤽 🤽‍♂️ 🤽‍♀️ 🤾 🤾‍♂️ 🤾‍♀️ 🤹 🤹‍♂️
🤹‍♀️ 🧘 🧘‍♂️ 🧘‍♀️ 🛀 🛌 🧑‍🤝‍🧑 👭 👫 👬 💏 👩‍❤️‍💋‍👨 👨‍❤️‍💋‍👨 👩‍❤️‍💋‍👩 💑 👩‍❤️‍👨 👨‍❤️‍👨 👩‍❤️‍👩 👪 👨‍👩‍👦
👨‍👩‍👧 👨‍👩‍👧‍👦 👨‍👩‍👦‍👦 👨‍👩‍👧‍👧 👨‍👨‍👦 👨‍👨‍👧 👨‍👨‍👧‍👦 👨‍👨‍👦‍👦 👨‍👨‍👧‍👧 👩‍👩‍👦 👩‍👩‍👧 👩‍👩‍👧‍👦 👩‍👩‍👦‍👦 👩‍👩‍👧‍👧 👨‍👦 👨‍👦‍👦 👨‍👧 👨‍👧‍👦 👨‍👧‍👧 👩‍👦
👩‍👦‍👦 👩‍👧 👩‍👧‍👦 👩‍👧‍👧 🗣️ 👤 👥 🫂 👣 🐵 🐒 🦍 🦧 🐶 🐕 🦮 🐕‍🦺 🐩 🐺 🦊
🦝 🐱 🐈 🐈‍⬛ 🦁 🐯 🐅 🐆 🐴 🫎 🫏 🐎 🦄 🦓 🦌 🦬 🐮 🐂 🐃 🐄
🐷 🐖 🐗 🐽 🐏 🐑 🐐 🐪 🐫 🦙 🦒 🐘 🦣 🦏 🦛 🐭 🐁 🐀 🐹 🐰
🐇 🐿️ 🦫 🦔 🦇 🐻 🐻‍❄️ 🐨 🐼 🦥 🦦 🦨 🦘 🦡 🐾 🦃 🐔 🐓 🐣 🐤
🐥 🐦 🐧 🕊️ 🦅 🦆 🦢 🦉 🦤 🪶 🦩 🦚 🦜 🪽 🐦‍⬛ 🪿 🐸 🐊 🐢 🦎
🐍 🐲 🐉 🦕 🦖 🐳 🐋 🐬 🦭 🐟 🐠 🐡 🦈 🐙 🐚 🪸 🪼 🐌 🦋 🐛
🐜 🐝 🪲 🐞 🦗 🪳 🕷️ 🕸️ 🦂 🦟 🪰 🪱 🦠 💐 🌸 💮 🪷 🏵️ 🌹 🥀
🌺 🌻 🌼 🌷 🪻 🌱 🪴 🌲 🌳 🌴 🌵 🌾 🌿 ☘️ 🍀 🍁 🍂 🍃 🪹 🪺
🍄 🍇 🍈 🍉 🍊 🍋 🍌 🍍 🥭 🍎 🍏 🍐 🍑 🍒 🍓 🫐 🥝 🍅 🫒 🥥
🥑 🍆 🥔 🥕 🌽 🌶️ 🫑 🥒 🥬 🥦 🧄 🧅 🥜 🫘 🌰 🫚 🫛 🍞 🥐 🥖
🫓 🥨 🥯 🥞 🧇 🧀 🍖 🍗 🥩 🥓 🍔 🍟 🍕 🌭 🥪 🌮 🌯 🫔 🥙 🧆
🥚 🍳 🥘 🍲 🫕 🥣 🥗 🍿 🧈 🧂 🥫 🍱 🍘 🍙 🍚 🍛 🍜 🍝 🍠 🍢
🍣 🍤 🍥 🥮 🍡 🥟 🥠 🥡 🦀 🦞 🦐 🦑 🦪 🍦 🍧 🍨 🍩 🍪 🎂 🍰
🧁 🥧 🍫 🍬 🍭 🍮 🍯 🍼 🥛 ☕ 🫖 🍵 🍶 🍾 🍷 🍸 🍹 🍺 🍻 🥂
🥃 🫗 🥤 🧋 🧃 🧉 🧊 🥢 🍽️ 🍴 🥄 🔪 🫙 🏺 🌍 🌎 🌏 🌐 🗺️ 🗾
🧭 🏔️ ⛰️ 🌋 🗻 🏕️ 🏖️ 🏜️ 🏝️ 🏞️ 🏟️ 🏛️ 🏗️ 🧱 🪨 🪵 🛖 🏘️ 🏚️ 🏠
🏡 🏢 🏣 🏤 🏥 🏦 🏨 🏩 🏪 🏫 🏬 🏭 🏯 🏰 💒 🗼 🗽 ⛪ 🕌 🛕
🕍 ⛩️ 🕋 ⛲ ⛺ 🌁 🌃 🏙️ 🌄 🌅 🌆 🌇 🌉 ♨️ 🎠 🛝 🎡 🎢 💈 🎪
🚂 🚃 🚄 🚅 🚆 🚇 🚈 🚉 🚊 🚝 🚞 🚋 🚌 🚍 🚎 🚐 🚑 🚒 🚓 🚔
🚕 🚖 🚗 🚘 🚙 🛻 🚚 🚛 🚜 🏎️ 🏍️ 🛵 🦽 🦼 🛺 🚲 🛴 🛹 🛼 🚏
🛣️ 🛤️ 🛢️ ⛽ 🛞 🚨 🚥 🚦 🛑 🚧 ⚓ 🛟 ⛵ 🛶 🚤 🛳️ ⛴️ 🛥️ 🚢 ✈️
🛩️ 🛫 🛬 🪂 💺 🚁 🚟 🚠 🚡 🛰️ 🚀 🛸 🛎️ 🧳 ⌛ ⏳ ⌚ ⏰ ⏱️ ⏲️
🕰️ 🕛 🕧 🕐 🕜 🕑 🕝 🕒 🕞 🕓 🕟 🕔 🕠 🕕 🕡 🕖 🕢 🕗 🕣 🕘
🕤 🕙 🕥 🕚 🕦 🌑 🌒 🌓 🌔 🌕 🌖 🌗 🌘 🌙 🌚 🌛 🌜 🌡️ ☀️ 🌝
🌞 🪐 ⭐ 🌟 🌠 🌌 ☁️ ⛅ ⛈️ 🌤️ 🌥️ 🌦️ 🌧️ 🌨️ 🌩️ 🌪️ 🌫️ 🌬️ 🌀 🌈
🌂 ☂️ ☔ ⛱️ ⚡ ❄️ ☃️ ⛄ ☄️ 🔥 💧 🌊 🎃 🎄 🎆 🎇 🧨 ✨ 🎈 🎉
🎊 🎋 🎍 🎎 🎏 🎐 🎑 🧧 🎀 🎁 🎗️ 🎟️ 🎫 🎖️ 🏆 🏅 🥇 🥈 🥉 ⚽
⚾ 🥎 🏀 🏐 🏈 🏉 🎾 🥏 🎳 🏏 🏑 🏒 🥍 🏓 🏸 🥊 🥋 🥅 ⛳ ⛸️
🎣 🤿 🎽 🎿 🛷 🥌 🎯 🪀 🪁 🔫 🎱 🔮 🪄 🎮 🕹️ 🎰 🎲 🧩 🧸 🪅
🪩 🪆 ♠️ ♥️ ♦️ ♣️ ♟️ 🃏 🀄 🎴 🎭 🖼️ 🎨 🧵 🪡 🧶 🪢 👓 🕶️ 🥽
🥼 🦺 👔 👕 👖 🧣 🧤 🧥 🧦 👗 👘 🥻 🩱 🩲 🩳 👙 👚 🪭 👛 👜
👝 🛍️ 🎒 🩴 👞 👟 🥾 🥿 👠 👡 🩰 👢 🪮 👑 👒 🎩 🎓 🧢 🪖 ⛑️
📿 💄 💍 💎 🔇 🔈 🔉 🔊 📢 📣 📯 🔔 🔕 🎼 🎵 🎶 🎙️ 🎚️ 🎛️ 🎤
🎧 📻 🎷 🪗 🎸 🎹 🎺 🎻 🪕 🥁 🪘 🪇 🪈 📱 📲 ☎️ 📞 📟 📠 🔋
🪫 🔌 💻 🖥️ 🖨️ ⌨️ 🖱️ 🖲️ 💽 💾 💿 📀 🧮 🎥 🎞️ 📽️ 🎬 📺 📷 📸
📹 📼 🔍 🔎 🕯️ 💡 🔦 🏮 🪔 📔 📕 📖 📗 📘 📙 📚 📓 📒 📃 📜
📄 📰 🗞️ 📑 🔖 🏷️ 💰 🪙 💴 💵 💶 💷 💸 💳 🧾 💹 ✉️ 📧 📨 📩
📤 📥 📦 📫 📪 📬 📭 📮 🗳️ ✏️ ✒️ 🖋️ 🖊️ 🖌️ 🖍️ 📝 💼 📁 📂 🗂️
📅 📆 🗒️ 🗓️ 📇 📈 📉 📊 📋 📌 📍 📎 🖇️ 📏 📐 ✂️ 🗃️ 🗄️ 🗑️ 🔒
🔓 🔏 🔐 🔑 🗝️ 🔨 🪓 ⛏️ ⚒️ 🛠️ 🗡️ ⚔️ 💣 🪃 🏹 🛡️ 🪚 🔧 🪛 🔩
⚙️ 🗜️ ⚖️ 🦯 🔗 ⛓️ 🪝 🧰 🧲 🪜 ⚗️ 🧪 🧫 🧬 🔬 🔭 📡 💉 🩸 💊
🩹 🩼 🩺 🩻 🚪 🛗 🪞 🪟 🛏️ 🛋️ 🪑 🚽 🪠 🚿 🛁 🪤 🪒 🧴 🧷 🧹
🧺 🧻 🪣 🧼 🫧 🪥 🧽 🧯 🛒 🚬 ⚰️ 🪦 ⚱️ 🧿 🪬 🗿 🪧 🪪 🏧 🚮
🚰 ♿ 🚹 🚺 🚻 🚼 🚾 🛂 🛃 🛄 🛅 ⚠️ 🚸 ⛔ 🚫 🚳 🚭 🚯 🚱 🚷
📵 🔞 ☢️ ☣️ ⬆️ ↗️ ➡️ ↘️ ⬇️ ↙️ ⬅️ ↖️ ↕️ ↔️ ↩️ ↪️ ⤴️ ⤵️ 🔃 🔄
🔙 🔚 🔛 🔜 🔝 🛐 ⚛️ 🕉️ ✡️ ☸️ ☯️ ✝️ ☦️ ☪️ ☮️ 🕎 🔯 🪯 ♈ ♉
♊ ♋ ♌ ♍ ♎ ♏ ♐ ♑ ♒ ♓ ⛎ 🔀 🔁 🔂 ▶️ ⏩ ⏭️ ⏯️ ◀️ ⏪
⏮️ 🔼 ⏫ 🔽 ⏬ ⏸️ ⏹️ ⏺️ ⏏️ 🎦 🔅 🔆 📶 🛜 📳 📴 ♀️ ♂️ ⚧️ ✖️
➕ ➖ ➗ 🟰 ♾️ ‼️ ⁉️ ❓ ❔ ❕ ❗ 〰️ 💱 💲 ⚕️ ♻️ ⚜️ 🔱 📛 🔰
⭕ ✅ ☑️ ✔️ ❌ ❎ ➰ ➿ 〽️ ✳️ ✴️ ❇️ ©️ ®️ ™️ #️⃣ *️⃣ 0️⃣ 1️⃣ 2️⃣
3️⃣ 4️⃣ 5️⃣ 6️⃣ 7️⃣ 8️⃣ 9️⃣ 🔟 🔠 🔡 🔢 🔣 🔤 🅰️ 🆎 🅱️ 🆑 🆒 🆓 ℹ️
🆔 Ⓜ️ 🆕 🆖 🅾️ 🆗 🅿️ 🆘 🆙 🆚 🈁 🈂️ 🈷️ 🈶 🈯 🉐 🈹 🈚 🈲 🉑
🈸 🈴 🈳 ㊗️ ㊙️ 🈺 🈵 🔴 🟠 🟡 🟢 🔵 🟣 🟤 ⚫ ⚪ 🟥 🟧 🟨 🟩
🟦 🟪 🟫 ⬛ ⬜ ◼️ ◻️ ◾ ◽ ▪️ ▫️ 🔶 🔷 🔸 🔹 🔺 🔻 💠 🔘 🔳
🔲 🏁 🚩 🎌 🏴 🏳️ 🏳️‍🌈 🏳️‍⚧️ 🏴‍☠️ 🇦🇨 🇦🇩 🇦🇪 🇦🇫 🇦🇬 🇦🇮 🇦🇱 🇦🇲 🇦🇴 🇦🇶 🇦🇷
🇦🇸 🇦🇹 🇦🇺 🇦🇼 🇦🇽 🇦🇿 🇧🇦 🇧🇧 🇧🇩 🇧🇪 🇧🇫 🇧🇬 🇧🇭 🇧🇮 🇧🇯 🇧🇱 🇧🇲 🇧🇳 🇧🇴 🇧🇶
🇧🇷 🇧🇸 🇧🇹 🇧🇻 🇧🇼 🇧🇾 🇧🇿 🇨🇦 🇨🇨 🇨🇩 🇨🇫 🇨🇬 🇨🇭 🇨🇮 🇨🇰 🇨🇱 🇨🇲 🇨🇳 🇨🇴 🇨🇵
🇨🇷 🇨🇺 🇨🇻 🇨🇼 🇨🇽 🇨🇾 🇨🇿 🇩🇪 🇩🇬 🇩🇯 🇩🇰 🇩🇲 🇩🇴 🇩🇿 🇪🇦 🇪🇨 🇪🇪 🇪🇬 🇪🇭 🇪🇷
🇪🇸 🇪🇹 🇪🇺 🇫🇮 🇫🇯 🇫🇰 🇫🇲 🇫🇴 🇫🇷 🇬🇦 🇬🇧 🇬🇩 🇬🇪 🇬🇫 🇬🇬 🇬🇭 🇬🇮 🇬🇱 🇬🇲 🇬🇳
🇬🇵 🇬🇶 🇬🇷 🇬🇸 🇬🇹 🇬🇺 🇬🇼 🇬🇾 🇭🇰 🇭🇲 🇭🇳 🇭🇷 🇭🇹 🇭🇺 🇮🇨 🇮🇩 🇮🇪 🇮🇱 🇮🇲 🇮🇳
🇮🇴 🇮🇶 🇮🇷 🇮🇸 🇮🇹 🇯🇪 🇯🇲 🇯🇴 🇯🇵 🇰🇪 🇰🇬 🇰🇭 🇰🇮 🇰🇲 🇰🇳 🇰🇵 🇰🇷 🇰🇼 🇰🇾 🇰🇿
🇱🇦 🇱🇧 🇱🇨 🇱🇮 🇱🇰 🇱🇷 🇱🇸 🇱🇹 🇱🇺 🇱🇻 🇱🇾 🇲🇦 🇲🇨 🇲🇩 🇲🇪 🇲🇫 🇲🇬 🇲🇭 🇲🇰 🇲🇱
🇲🇲 🇲🇳 🇲🇴 🇲🇵 🇲🇶 🇲🇷 🇲🇸 🇲🇹 🇲🇺 🇲🇻 🇲🇼 🇲🇽 🇲🇾 🇲🇿 🇳🇦 🇳🇨 🇳🇪 🇳🇫 🇳🇬 🇳🇮
🇳🇱 🇳🇴 🇳🇵 🇳🇷 🇳🇺 🇳🇿 🇴🇲 🇵🇦 🇵🇪 🇵🇫 🇵🇬 🇵🇭 🇵🇰 🇵🇱 🇵🇲 🇵🇳 🇵🇷 🇵🇸 🇵🇹 🇵🇼
🇵🇾 🇶🇦 🇷🇪 🇷🇴 🇷🇸 🇷🇺 🇷🇼 🇸🇦 🇸🇧 🇸🇨 🇸🇩 🇸🇪 🇸🇬 🇸🇭 🇸🇮 🇸🇯 🇸🇰 🇸🇱 🇸🇲 🇸🇳
🇸🇴 🇸🇷 🇸🇸 🇸🇹 🇸🇻 🇸🇽 🇸🇾 🇸🇿 🇹🇦 🇹🇨 🇹🇩 🇹🇫 🇹🇬 🇹🇭 🇹🇯 🇹🇰 🇹🇱 🇹🇲 🇹🇳 🇹🇴
🇹🇷 🇹🇹 🇹🇻 🇹🇼 🇹🇿 🇺🇦 🇺🇬 🇺🇲 🇺🇳 🇺🇸 🇺🇾 🇺🇿 🇻🇦 🇻🇨 🇻🇪 🇻🇬 🇻🇮 🇻🇳 🇻🇺 🇼🇫
🇼🇸 🇽🇰 🇾🇪 🇾🇹 🇿🇦 🇿🇲 🇿🇼 🏴󠁧󠁢󠁥󠁮󠁧󠁿 🏴󠁧󠁢󠁳󠁣󠁴󠁿 🏴󠁧󠁢󠁷󠁬󠁳󠁿

### Shortcodes

Below is a list of all the shortcodes that can be used in Markdown documents to
generate the glyphs above.

:grinning: :smiley: :smile: :grin: :laughing: :satisfied: :sweat_smile: :rofl: :joy: :slightly_smiling_face: :upside_down_face: :melting_face: :wink: :blush: :innocent: :smiling_face_with_three_hearts: :heart_eyes: :star_struck: :kissing_heart: :kissing:
:relaxed: :kissing_closed_eyes: :kissing_smiling_eyes: :smiling_face_with_tear: :yum: :stuck_out_tongue: :stuck_out_tongue_winking_eye: :zany_face: :stuck_out_tongue_closed_eyes: :money_mouth_face: :hugs: :hand_over_mouth: :face_with_open_eyes_and_hand_over_mouth: :face_with_peeking_eye: :shushing_face: :thinking: :saluting_face: :zipper_mouth_face: :raised_eyebrow: :neutral_face:
:expressionless: :no_mouth: :dotted_line_face: :face_in_clouds: :smirk: :unamused: :roll_eyes: :grimacing: :face_exhaling: :lying_face: :shaking_face: :relieved: :pensive: :sleepy: :drooling_face: :sleeping: :mask: :face_with_thermometer: :face_with_head_bandage: :nauseated_face:
:vomiting_face: :sneezing_face: :hot_face: :cold_face: :woozy_face: :dizzy_face: :face_with_spiral_eyes: :exploding_head: :cowboy_hat_face: :partying_face: :disguised_face: :sunglasses: :nerd_face: :monocle_face: :confused: :face_with_diagonal_mouth: :worried: :slightly_frowning_face: :frowning_face: :open_mouth:
:hushed: :astonished: :flushed: :pleading_face: :face_holding_back_tears: :frowning: :anguished: :fearful: :cold_sweat: :disappointed_relieved: :cry: :sob: :scream: :confounded: :persevere: :disappointed: :sweat: :weary: :tired_face: :yawning_face:
:triumph: :rage: :pout: :angry: :cursing_face: :smiling_imp: :imp: :skull: :skull_and_crossbones: :hankey: :poop: :shit: :clown_face: :japanese_ogre: :japanese_goblin: :ghost: :alien: :space_invader: :robot: :smiley_cat:
:smile_cat: :joy_cat: :heart_eyes_cat: :smirk_cat: :kissing_cat: :scream_cat: :crying_cat_face: :pouting_cat: :see_no_evil: :hear_no_evil: :speak_no_evil: :love_letter: :cupid: :gift_heart: :sparkling_heart: :heartpulse: :heartbeat: :revolving_hearts: :two_hearts: :heart_decoration:
:heavy_heart_exclamation: :broken_heart: :heart_on_fire: :mending_heart: :heart: :pink_heart: :orange_heart: :yellow_heart: :green_heart: :blue_heart: :light_blue_heart: :purple_heart: :brown_heart: :black_heart: :grey_heart: :white_heart: :kiss: :100: :anger: :boom:
:collision: :dizzy: :sweat_drops: :dash: :hole: :speech_balloon: :eye_speech_bubble: :left_speech_bubble: :right_anger_bubble: :thought_balloon: :zzz: :wave: :raised_back_of_hand: :raised_hand_with_fingers_splayed: :hand: :raised_hand: :vulcan_salute: :rightwards_hand: :leftwards_hand: :palm_down_hand:
:palm_up_hand: :leftwards_pushing_hand: :rightwards_pushing_hand: :ok_hand: :pinched_fingers: :pinching_hand: :v: :crossed_fingers: :hand_with_index_finger_and_thumb_crossed: :love_you_gesture: :metal: :call_me_hand: :point_left: :point_right: :point_up_2: :middle_finger: :fu: :point_down: :point_up: :index_pointing_at_the_viewer:
:+1: :thumbsup: :-1: :thumbsdown: :fist_raised: :fist: :fist_oncoming: :facepunch: :punch: :fist_left: :fist_right: :clap: :raised_hands: :heart_hands: :open_hands: :palms_up_together: :handshake: :pray: :writing_hand: :nail_care:
:selfie: :muscle: :mechanical_arm: :mechanical_leg: :leg: :foot: :ear: :ear_with_hearing_aid: :nose: :brain: :anatomical_heart: :lungs: :tooth: :bone: :eyes: :eye: :tongue: :lips: :biting_lip: :baby:
:child: :boy: :girl: :adult: :blond_haired_person: :man: :bearded_person: :man_beard: :woman_beard: :red_haired_man: :curly_haired_man: :white_haired_man: :bald_man: :woman: :red_haired_woman: :person_red_hair: :curly_haired_woman: :person_curly_hair: :white_haired_woman: :person_white_hair:
:bald_woman: :person_bald: :blond_haired_woman: :blonde_woman: :blond_haired_man: :older_adult: :older_man: :older_woman: :frowning_person: :frowning_man: :frowning_woman: :pouting_face: :pouting_man: :pouting_woman: :no_good: :no_good_man: :ng_man: :no_good_woman: :ng_woman: :ok_person:
:ok_man: :ok_woman: :tipping_hand_person: :information_desk_person: :tipping_hand_man: :sassy_man: :tipping_hand_woman: :sassy_woman: :raising_hand: :raising_hand_man: :raising_hand_woman: :deaf_person: :deaf_man: :deaf_woman: :bow: :bowing_man: :bowing_woman: :facepalm: :man_facepalming: :woman_facepalming:
:shrug: :man_shrugging: :woman_shrugging: :health_worker: :man_health_worker: :woman_health_worker: :student: :man_student: :woman_student: :teacher: :man_teacher: :woman_teacher: :judge: :man_judge: :woman_judge: :farmer: :man_farmer: :woman_farmer: :cook: :man_cook:
:woman_cook: :mechanic: :man_mechanic: :woman_mechanic: :factory_worker: :man_factory_worker: :woman_factory_worker: :office_worker: :man_office_worker: :woman_office_worker: :scientist: :man_scientist: :woman_scientist: :technologist: :man_technologist: :woman_technologist: :singer: :man_singer: :woman_singer: :artist:
:man_artist: :woman_artist: :pilot: :man_pilot: :woman_pilot: :astronaut: :man_astronaut: :woman_astronaut: :firefighter: :man_firefighter: :woman_firefighter: :police_officer: :cop: :policeman: :policewoman: :detective: :male_detective: :female_detective: :guard: :guardsman:
:guardswoman: :ninja: :construction_worker: :construction_worker_man: :construction_worker_woman: :person_with_crown: :prince: :princess: :person_with_turban: :man_with_turban: :woman_with_turban: :man_with_gua_pi_mao: :woman_with_headscarf: :person_in_tuxedo: :man_in_tuxedo: :woman_in_tuxedo: :person_with_veil: :man_with_veil: :woman_with_veil: :bride_with_veil:
:pregnant_woman: :pregnant_man: :pregnant_person: :breast_feeding: :woman_feeding_baby: :man_feeding_baby: :person_feeding_baby: :angel: :santa: :mrs_claus: :mx_claus: :superhero: :superhero_man: :superhero_woman: :supervillain: :supervillain_man: :supervillain_woman: :mage: :mage_man: :mage_woman:
:fairy: :fairy_man: :fairy_woman: :vampire: :vampire_man: :vampire_woman: :merperson: :merman: :mermaid: :elf: :elf_man: :elf_woman: :genie: :genie_man: :genie_woman: :zombie: :zombie_man: :zombie_woman: :troll: :massage:
:massage_man: :massage_woman: :haircut: :haircut_man: :haircut_woman: :walking: :walking_man: :walking_woman: :standing_person: :standing_man: :standing_woman: :kneeling_person: :kneeling_man: :kneeling_woman: :person_with_probing_cane: :man_with_probing_cane: :woman_with_probing_cane: :person_in_motorized_wheelchair: :man_in_motorized_wheelchair: :woman_in_motorized_wheelchair:
:person_in_manual_wheelchair: :man_in_manual_wheelchair: :woman_in_manual_wheelchair: :runner: :running: :running_man: :running_woman: :woman_dancing: :dancer: :man_dancing: :business_suit_levitating: :dancers: :dancing_men: :dancing_women: :sauna_person: :sauna_man: :sauna_woman: :climbing: :climbing_man: :climbing_woman:
:person_fencing: :horse_racing: :skier: :snowboarder: :golfing: :golfing_man: :golfing_woman: :surfer: :surfing_man: :surfing_woman: :rowboat: :rowing_man: :rowing_woman: :swimmer: :swimming_man: :swimming_woman: :bouncing_ball_person: :bouncing_ball_man: :basketball_man: :bouncing_ball_woman:
:basketball_woman: :weight_lifting: :weight_lifting_man: :weight_lifting_woman: :bicyclist: :biking_man: :biking_woman: :mountain_bicyclist: :mountain_biking_man: :mountain_biking_woman: :cartwheeling: :man_cartwheeling: :woman_cartwheeling: :wrestling: :men_wrestling: :women_wrestling: :water_polo: :man_playing_water_polo: :woman_playing_water_polo: :handball_person:
:man_playing_handball: :woman_playing_handball: :juggling_person: :man_juggling: :woman_juggling: :lotus_position: :lotus_position_man: :lotus_position_woman: :bath: :sleeping_bed: :people_holding_hands: :two_women_holding_hands: :couple: :two_men_holding_hands: :couplekiss: :couplekiss_man_woman: :couplekiss_man_man: :couplekiss_woman_woman: :couple_with_heart: :couple_with_heart_woman_man:
:couple_with_heart_man_man: :couple_with_heart_woman_woman: :family: :family_man_woman_boy: :family_man_woman_girl: :family_man_woman_girl_boy: :family_man_woman_boy_boy: :family_man_woman_girl_girl: :family_man_man_boy: :family_man_man_girl: :family_man_man_girl_boy: :family_man_man_boy_boy: :family_man_man_girl_girl: :family_woman_woman_boy: :family_woman_woman_girl: :family_woman_woman_girl_boy: :family_woman_woman_boy_boy: :family_woman_woman_girl_girl: :family_man_boy: :family_man_boy_boy:
:family_man_girl: :family_man_girl_boy: :family_man_girl_girl: :family_woman_boy: :family_woman_boy_boy: :family_woman_girl: :family_woman_girl_boy: :family_woman_girl_girl: :speaking_head: :bust_in_silhouette: :busts_in_silhouette: :people_hugging: :footprints: :monkey_face: :monkey: :gorilla: :orangutan: :dog: :dog2: :guide_dog:
:service_dog: :poodle: :wolf: :fox_face: :raccoon: :cat: :cat2: :black_cat: :lion: :tiger: :tiger2: :leopard: :horse: :moose: :donkey: :racehorse: :unicorn: :zebra: :deer: :bison:
:cow: :ox: :water_buffalo: :cow2: :pig: :pig2: :boar: :pig_nose: :ram: :sheep: :goat: :dromedary_camel: :camel: :llama: :giraffe: :elephant: :mammoth: :rhinoceros: :hippopotamus: :mouse:
:mouse2: :rat: :hamster: :rabbit: :rabbit2: :chipmunk: :beaver: :hedgehog: :bat: :bear: :polar_bear: :koala: :panda_face: :sloth: :otter: :skunk: :kangaroo: :badger: :feet: :paw_prints:
:turkey: :chicken: :rooster: :hatching_chick: :baby_chick: :hatched_chick: :bird: :penguin: :dove: :eagle: :duck: :swan: :owl: :dodo: :feather: :flamingo: :peacock: :parrot: :wing: :black_bird:
:goose: :frog: :crocodile: :turtle: :lizard: :snake: :dragon_face: :dragon: :sauropod: :t-rex: :whale: :whale2: :dolphin: :flipper: :seal: :fish: :tropical_fish: :blowfish: :shark: :octopus:
:shell: :coral: :jellyfish: :snail: :butterfly: :bug: :ant: :bee: :honeybee: :beetle: :lady_beetle: :cricket: :cockroach: :spider: :spider_web: :scorpion: :mosquito: :fly: :worm: :microbe:
:bouquet: :cherry_blossom: :white_flower: :lotus: :rosette: :rose: :wilted_flower: :hibiscus: :sunflower: :blossom: :tulip: :hyacinth: :seedling: :potted_plant: :evergreen_tree: :deciduous_tree: :palm_tree: :cactus: :ear_of_rice: :herb:
:shamrock: :four_leaf_clover: :maple_leaf: :fallen_leaf: :leaves: :empty_nest: :nest_with_eggs: :mushroom: :grapes: :melon: :watermelon: :tangerine: :orange: :mandarin: :lemon: :banana: :pineapple: :mango: :apple: :green_apple:
:pear: :peach: :cherries: :strawberry: :blueberries: :kiwi_fruit: :tomato: :olive: :coconut: :avocado: :eggplant: :potato: :carrot: :corn: :hot_pepper: :bell_pepper: :cucumber: :leafy_green: :broccoli: :garlic:
:onion: :peanuts: :beans: :chestnut: :ginger_root: :pea_pod: :bread: :croissant: :baguette_bread: :flatbread: :pretzel: :bagel: :pancakes: :waffle: :cheese: :meat_on_bone: :poultry_leg: :cut_of_meat: :bacon: :hamburger:
:fries: :pizza: :hotdog: :sandwich: :taco: :burrito: :tamale: :stuffed_flatbread: :falafel: :egg: :fried_egg: :shallow_pan_of_food: :stew: :fondue: :bowl_with_spoon: :green_salad: :popcorn: :butter: :salt: :canned_food:
:bento: :rice_cracker: :rice_ball: :rice: :curry: :ramen: :spaghetti: :sweet_potato: :oden: :sushi: :fried_shrimp: :fish_cake: :moon_cake: :dango: :dumpling: :fortune_cookie: :takeout_box: :crab: :lobster: :shrimp:
:squid: :oyster: :icecream: :shaved_ice: :ice_cream: :doughnut: :cookie: :birthday: :cake: :cupcake: :pie: :chocolate_bar: :candy: :lollipop: :custard: :honey_pot: :baby_bottle: :milk_glass: :coffee: :teapot:
:tea: :sake: :champagne: :wine_glass: :cocktail: :tropical_drink: :beer: :beers: :clinking_glasses: :tumbler_glass: :pouring_liquid: :cup_with_straw: :bubble_tea: :beverage_box: :mate: :ice_cube: :chopsticks: :plate_with_cutlery: :fork_and_knife: :spoon:
:hocho: :knife: :jar: :amphora: :earth_africa: :earth_americas: :earth_asia: :globe_with_meridians: :world_map: :japan: :compass: :mountain_snow: :mountain: :volcano: :mount_fuji: :camping: :beach_umbrella: :desert: :desert_island: :national_park:
:stadium: :classical_building: :building_construction: :bricks: :rock: :wood: :hut: :houses: :derelict_house: :house: :house_with_garden: :office: :post_office: :european_post_office: :hospital: :bank: :hotel: :love_hotel: :convenience_store: :school:
:department_store: :factory: :japanese_castle: :european_castle: :wedding: :tokyo_tower: :statue_of_liberty: :church: :mosque: :hindu_temple: :synagogue: :shinto_shrine: :kaaba: :fountain: :tent: :foggy: :night_with_stars: :cityscape: :sunrise_over_mountains: :sunrise:
:city_sunset: :city_sunrise: :bridge_at_night: :hotsprings: :carousel_horse: :playground_slide: :ferris_wheel: :roller_coaster: :barber: :circus_tent: :steam_locomotive: :railway_car: :bullettrain_side: :bullettrain_front: :train2: :metro: :light_rail: :station: :tram: :monorail:
:mountain_railway: :train: :bus: :oncoming_bus: :trolleybus: :minibus: :ambulance: :fire_engine: :police_car: :oncoming_police_car: :taxi: :oncoming_taxi: :car: :red_car: :oncoming_automobile: :blue_car: :pickup_truck: :truck: :articulated_lorry: :tractor:
:racing_car: :motorcycle: :motor_scooter: :manual_wheelchair: :motorized_wheelchair: :auto_rickshaw: :bike: :kick_scooter: :skateboard: :roller_skate: :busstop: :motorway: :railway_track: :oil_drum: :fuelpump: :wheel: :rotating_light: :traffic_light: :vertical_traffic_light: :stop_sign:
:construction: :anchor: :ring_buoy: :boat: :sailboat: :canoe: :speedboat: :passenger_ship: :ferry: :motor_boat: :ship: :airplane: :small_airplane: :flight_departure: :flight_arrival: :parachute: :seat: :helicopter: :suspension_railway: :mountain_cableway:
:aerial_tramway: :artificial_satellite: :rocket: :flying_saucer: :bellhop_bell: :luggage: :hourglass: :hourglass_flowing_sand: :watch: :alarm_clock: :stopwatch: :timer_clock: :mantelpiece_clock: :clock12: :clock1230: :clock1: :clock130: :clock2: :clock230: :clock3:
:clock330: :clock4: :clock430: :clock5: :clock530: :clock6: :clock630: :clock7: :clock730: :clock8: :clock830: :clock9: :clock930: :clock10: :clock1030: :clock11: :clock1130: :new_moon: :waxing_crescent_moon: :first_quarter_moon:
:moon: :waxing_gibbous_moon: :full_moon: :waning_gibbous_moon: :last_quarter_moon: :waning_crescent_moon: :crescent_moon: :new_moon_with_face: :first_quarter_moon_with_face: :last_quarter_moon_with_face: :thermometer: :sunny: :full_moon_with_face: :sun_with_face: :ringed_planet: :star: :star2: :stars: :milky_way: :cloud:
:partly_sunny: :cloud_with_lightning_and_rain: :sun_behind_small_cloud: :sun_behind_large_cloud: :sun_behind_rain_cloud: :cloud_with_rain: :cloud_with_snow: :cloud_with_lightning: :tornado: :fog: :wind_face: :cyclone: :rainbow: :closed_umbrella: :open_umbrella: :umbrella: :parasol_on_ground: :zap: :snowflake: :snowman_with_snow:
:snowman: :comet: :fire: :droplet: :ocean: :jack_o_lantern: :christmas_tree: :fireworks: :sparkler: :firecracker: :sparkles: :balloon: :tada: :confetti_ball: :tanabata_tree: :bamboo: :dolls: :flags: :wind_chime: :rice_scene:
:red_envelope: :ribbon: :gift: :reminder_ribbon: :tickets: :ticket: :medal_military: :trophy: :medal_sports: :1st_place_medal: :2nd_place_medal: :3rd_place_medal: :soccer: :baseball: :softball: :basketball: :volleyball: :football: :rugby_football: :tennis:
:flying_disc: :bowling: :cricket_game: :field_hockey: :ice_hockey: :lacrosse: :ping_pong: :badminton: :boxing_glove: :martial_arts_uniform: :goal_net: :golf: :ice_skate: :fishing_pole_and_fish: :diving_mask: :running_shirt_with_sash: :ski: :sled: :curling_stone: :dart:
:yo_yo: :kite: :gun: :8ball: :crystal_ball: :magic_wand: :video_game: :joystick: :slot_machine: :game_die: :jigsaw: :teddy_bear: :pinata: :mirror_ball: :nesting_dolls: :spades: :hearts: :diamonds: :clubs: :chess_pawn:
:black_joker: :mahjong: :flower_playing_cards: :performing_arts: :framed_picture: :art: :thread: :sewing_needle: :yarn: :knot: :eyeglasses: :dark_sunglasses: :goggles: :lab_coat: :safety_vest: :necktie: :shirt: :tshirt: :jeans: :scarf:
:gloves: :coat: :socks: :dress: :kimono: :sari: :one_piece_swimsuit: :swim_brief: :shorts: :bikini: :womans_clothes: :folding_hand_fan: :purse: :handbag: :pouch: :shopping: :school_satchel: :thong_sandal: :mans_shoe: :shoe:
:athletic_shoe: :hiking_boot: :flat_shoe: :high_heel: :sandal: :ballet_shoes: :boot: :hair_pick: :crown: :womans_hat: :tophat: :mortar_board: :billed_cap: :military_helmet: :rescue_worker_helmet: :prayer_beads: :lipstick: :ring: :gem: :mute:
:speaker: :sound: :loud_sound: :loudspeaker: :mega: :postal_horn: :bell: :no_bell: :musical_score: :musical_note: :notes: :studio_microphone: :level_slider: :control_knobs: :microphone: :headphones: :radio: :saxophone: :accordion: :guitar:
:musical_keyboard: :trumpet: :violin: :banjo: :drum: :long_drum: :maracas: :flute: :iphone: :calling: :phone: :telephone: :telephone_receiver: :pager: :fax: :battery: :low_battery: :electric_plug: :computer: :desktop_computer:
:printer: :keyboard: :computer_mouse: :trackball: :minidisc: :floppy_disk: :cd: :dvd: :abacus: :movie_camera: :film_strip: :film_projector: :clapper: :tv: :camera: :camera_flash: :video_camera: :vhs: :mag: :mag_right:
:candle: :bulb: :flashlight: :izakaya_lantern: :lantern: :diya_lamp: :notebook_with_decorative_cover: :closed_book: :book: :open_book: :green_book: :blue_book: :orange_book: :books: :notebook: :ledger: :page_with_curl: :scroll: :page_facing_up: :newspaper:
:newspaper_roll: :bookmark_tabs: :bookmark: :label: :moneybag: :coin: :yen: :dollar: :euro: :pound: :money_with_wings: :credit_card: :receipt: :chart: :envelope: :email: :e-mail: :incoming_envelope: :envelope_with_arrow: :outbox_tray:
:inbox_tray: :package: :mailbox: :mailbox_closed: :mailbox_with_mail: :mailbox_with_no_mail: :postbox: :ballot_box: :pencil2: :black_nib: :fountain_pen: :pen: :paintbrush: :crayon: :memo: :pencil: :briefcase: :file_folder: :open_file_folder: :card_index_dividers:
:date: :calendar: :spiral_notepad: :spiral_calendar: :card_index: :chart_with_upwards_trend: :chart_with_downwards_trend: :bar_chart: :clipboard: :pushpin: :round_pushpin: :paperclip: :paperclips: :straight_ruler: :triangular_ruler: :scissors: :card_file_box: :file_cabinet: :wastebasket: :lock:
:unlock: :lock_with_ink_pen: :closed_lock_with_key: :key: :old_key: :hammer: :axe: :pick: :hammer_and_pick: :hammer_and_wrench: :dagger: :crossed_swords: :bomb: :boomerang: :bow_and_arrow: :shield: :carpentry_saw: :wrench: :screwdriver: :nut_and_bolt:
:gear: :clamp: :balance_scale: :probing_cane: :link: :chains: :hook: :toolbox: :magnet: :ladder: :alembic: :test_tube: :petri_dish: :dna: :microscope: :telescope: :satellite: :syringe: :drop_of_blood: :pill:
:adhesive_bandage: :crutch: :stethoscope: :x_ray: :door: :elevator: :mirror: :window: :bed: :couch_and_lamp: :chair: :toilet: :plunger: :shower: :bathtub: :mouse_trap: :razor: :lotion_bottle: :safety_pin: :broom:
:basket: :roll_of_paper: :bucket: :soap: :bubbles: :toothbrush: :sponge: :fire_extinguisher: :shopping_cart: :smoking: :coffin: :headstone: :funeral_urn: :nazar_amulet: :hamsa: :moyai: :placard: :identification_card: :atm: :put_litter_in_its_place:
:potable_water: :wheelchair: :mens: :womens: :restroom: :baby_symbol: :wc: :passport_control: :customs: :baggage_claim: :left_luggage: :warning: :children_crossing: :no_entry: :no_entry_sign: :no_bicycles: :no_smoking: :do_not_litter: :non-potable_water: :no_pedestrians:
:no_mobile_phones: :underage: :radioactive: :biohazard: :arrow_up: :arrow_upper_right: :arrow_right: :arrow_lower_right: :arrow_down: :arrow_lower_left: :arrow_left: :arrow_upper_left: :arrow_up_down: :left_right_arrow: :leftwards_arrow_with_hook: :arrow_right_hook: :arrow_heading_up: :arrow_heading_down: :arrows_clockwise: :arrows_counterclockwise:
:back: :end: :on: :soon: :top: :place_of_worship: :atom_symbol: :om: :star_of_david: :wheel_of_dharma: :yin_yang: :latin_cross: :orthodox_cross: :star_and_crescent: :peace_symbol: :menorah: :six_pointed_star: :khanda: :aries: :taurus:
:gemini: :cancer: :leo: :virgo: :libra: :scorpius: :sagittarius: :capricorn: :aquarius: :pisces: :ophiuchus: :twisted_rightwards_arrows: :repeat: :repeat_one: :arrow_forward: :fast_forward: :next_track_button: :play_or_pause_button: :arrow_backward: :rewind:
:previous_track_button: :arrow_up_small: :arrow_double_up: :arrow_down_small: :arrow_double_down: :pause_button: :stop_button: :record_button: :eject_button: :cinema: :low_brightness: :high_brightness: :signal_strength: :wireless: :vibration_mode: :mobile_phone_off: :female_sign: :male_sign: :transgender_symbol: :heavy_multiplication_x:
:heavy_plus_sign: :heavy_minus_sign: :heavy_division_sign: :heavy_equals_sign: :infinity: :bangbang: :interrobang: :question: :grey_question: :grey_exclamation: :exclamation: :heavy_exclamation_mark: :wavy_dash: :currency_exchange: :heavy_dollar_sign: :medical_symbol: :recycle: :fleur_de_lis: :trident: :name_badge:
:beginner: :o: :white_check_mark: :ballot_box_with_check: :heavy_check_mark: :x: :negative_squared_cross_mark: :curly_loop: :loop: :part_alternation_mark: :eight_spoked_asterisk: :eight_pointed_black_star: :sparkle: :copyright: :registered: :tm: :hash: :asterisk: :zero: :one:
:two: :three: :four: :five: :six: :seven: :eight: :nine: :keycap_ten: :capital_abcd: :abcd: :1234: :symbols: :abc: :a: :ab: :b: :cl: :cool: :free:
:information_source: :id: :m: :new: :ng: :o2: :ok: :parking: :sos: :up: :vs: :koko: :sa: :u6708: :u6709: :u6307: :ideograph_advantage: :u5272: :u7121: :u7981:
:accept: :u7533: :u5408: :u7a7a: :congratulations: :secret: :u55b6: :u6e80: :red_circle: :orange_circle: :yellow_circle: :green_circle: :large_blue_circle: :purple_circle: :brown_circle: :black_circle: :white_circle: :red_square: :orange_square: :yellow_square:
:green_square: :blue_square: :purple_square: :brown_square: :black_large_square: :white_large_square: :black_medium_square: :white_medium_square: :black_medium_small_square: :white_medium_small_square: :black_small_square: :white_small_square: :large_orange_diamond: :large_blue_diamond: :small_orange_diamond: :small_blue_diamond: :small_red_triangle: :small_red_triangle_down: :diamond_shape_with_a_dot_inside: :radio_button:
:white_square_button: :black_square_button: :checkered_flag: :triangular_flag_on_post: :crossed_flags: :black_flag: :white_flag: :rainbow_flag: :transgender_flag: :pirate_flag: :ascension_island: :andorra: :united_arab_emirates: :afghanistan: :antigua_barbuda: :anguilla: :albania: :armenia: :angola: :antarctica:
:argentina: :american_samoa: :austria: :australia: :aruba: :aland_islands: :azerbaijan: :bosnia_herzegovina: :barbados: :bangladesh: :belgium: :burkina_faso: :bulgaria: :bahrain: :burundi: :benin: :st_barthelemy: :bermuda: :brunei: :bolivia:
:caribbean_netherlands: :brazil: :bahamas: :bhutan: :bouvet_island: :botswana: :belarus: :belize: :canada: :cocos_islands: :congo_kinshasa: :central_african_republic: :congo_brazzaville: :switzerland: :cote_divoire: :cook_islands: :chile: :cameroon: :cn: :colombia:
:clipperton_island: :costa_rica: :cuba: :cape_verde: :curacao: :christmas_island: :cyprus: :czech_republic: :de: :diego_garcia: :djibouti: :denmark: :dominica: :dominican_republic: :algeria: :ceuta_melilla: :ecuador: :estonia: :egypt: :western_sahara:
:eritrea: :es: :ethiopia: :eu: :european_union: :finland: :fiji: :falkland_islands: :micronesia: :faroe_islands: :fr: :gabon: :gb: :uk: :grenada: :georgia: :french_guiana: :guernsey: :ghana: :gibraltar:
:greenland: :gambia: :guinea: :guadeloupe: :equatorial_guinea: :greece: :south_georgia_south_sandwich_islands: :guatemala: :guam: :guinea_bissau: :guyana: :hong_kong: :heard_mcdonald_islands: :honduras: :croatia: :haiti: :hungary: :canary_islands: :indonesia: :ireland:
:israel: :isle_of_man: :india: :british_indian_ocean_territory: :iraq: :iran: :iceland: :it: :jersey: :jamaica: :jordan: :jp: :kenya: :kyrgyzstan: :cambodia: :kiribati: :comoros: :st_kitts_nevis: :north_korea: :kr:
:kuwait: :cayman_islands: :kazakhstan: :laos: :lebanon: :st_lucia: :liechtenstein: :sri_lanka: :liberia: :lesotho: :lithuania: :luxembourg: :latvia: :libya: :morocco: :monaco: :moldova: :montenegro: :st_martin: :madagascar:
:marshall_islands: :macedonia: :mali: :myanmar: :mongolia: :macau: :northern_mariana_islands: :martinique: :mauritania: :montserrat: :malta: :mauritius: :maldives: :malawi: :mexico: :malaysia: :mozambique: :namibia: :new_caledonia: :niger:
:norfolk_island: :nigeria: :nicaragua: :netherlands: :norway: :nepal: :nauru: :niue: :new_zealand: :oman: :panama: :peru: :french_polynesia: :papua_new_guinea: :philippines: :pakistan: :poland: :st_pierre_miquelon: :pitcairn_islands: :puerto_rico:
:palestinian_territories: :portugal: :palau: :paraguay: :qatar: :reunion: :romania: :serbia: :ru: :rwanda: :saudi_arabia: :solomon_islands: :seychelles: :sudan: :sweden: :singapore: :st_helena: :slovenia: :svalbard_jan_mayen: :slovakia:
:sierra_leone: :san_marino: :senegal: :somalia: :suriname: :south_sudan: :sao_tome_principe: :el_salvador: :sint_maarten: :syria: :swaziland: :tristan_da_cunha: :turks_caicos_islands: :chad: :french_southern_territories: :togo: :thailand: :tajikistan: :tokelau: :timor_leste:
:turkmenistan: :tunisia: :tonga: :tr: :trinidad_tobago: :tuvalu: :taiwan: :tanzania: :ukraine: :uganda: :us_outlying_islands: :united_nations: :us: :uruguay: :uzbekistan: :vatican_city: :st_vincent_grenadines: :venezuela: :british_virgin_islands: :us_virgin_islands:
:vietnam: :vanuatu: :wallis_futuna: :samoa: :kosovo: :yemen: :mayotte: :south_africa: :zambia: :zimbabwe: :england: :scotland: :wales: 


