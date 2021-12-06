# SyntaxHighlighter

This is the most important wiki part from the google archive page of the [`SyntaxHighlighter`](https://code.google.com/archive/p/syntaxhighlighter/)


## Functions provided by highligher

### dp.SyntaxHighlighter.HighlightAll()

```js
dp.SyntaxHighlighter.HighlightAll(
  name,
  [showGutter],
  [showControls],
  [collapseAll],
  [firstLine],
  [showColumns]
);
```

| name         | required | Name of <pre\> and <textarea\> elements to use.                                                                                                               |
| ------------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| showGutter   | optional | Turns gutter on or off on all processed code blocks                                                                                                           |
| showControls | optional | Turns controls on or off on all processed <pre>.                                                                                                              |
| collapseAll  | optional | Turns collapse on or off on all processed <pre>. If showControls is false or switched, this value will be ignored.                                            |
| firstLine    | optional | Allows to specify the first line where line numbering starts. This is usefull if you want to illustrate where the code block is located relative to the file. |
| showColumns  | optional | Will show row columns in the first line.                                                                                                                      |

## Configuration Options

| nogutter         | Will display no gutter.                             |
| ---------------- | --------------------------------------------------- |
| nocontrols       | Will display no controls at the top.                |
| collapse         | Will collapse the block by default.                 |
| firstline[value] | Will begin line count at value. Default value is 1. |
| showcolumns      | Will show row columns in the first line.            |

## Languages

| Language    | Aliases                 |
| ----------- | ----------------------- |
| C++         | cpp, c, c++             |
| C#          | c#, c-sharp, csharp     |
| CSS         | css                     |
| Delphi      | delphi, pascal          |
| Java        | java                    |
| Java Script | js, jscript, javascript |
| PHP         | php                     |
| Python      | py, python              |
| Ruby        | rb, ruby, rails, ror    |
| Sql         | sql                     |
| VB          | vb, vb.net              |
| XML/HTML    | xml, html, xhtml, xslt  |

## Blogger Mode

Blogger has a nasty habit of replacing all new lines with <br/> tags which makes it impossible to post code snippets.

To fix the situation version 1.5.1 introduces "Blogger Mode" which can be enabled before a call to HighlightAll() by a call to BloggerMode() like in the example below:

```js
dp.SyntaxHighlighter.BloggerMode();
dp.SyntaxHighlighter.HighlightAll("code");
```

## Copy to clipboard

Version 1.5 makes "copy to clipboard" functionality available to all browsers supporting Flash. It uses a 1x1 Flash movie to copy the code to user's clipboard.

To enable this functionality, you must set ClipboardSwf variable to location of clipboard.swf file. This must be done before calling HighlightAll.

```js
dp.SyntaxHighlighter.ClipboardSwf = "/flash/clipboard.swf";
dp.SyntaxHighlighter.HighlightAll("code");
```
