// This file is included in `bindings.rs`

export const highlight = () => {
    console.log(0, dp.SyntaxHighlighter.HighlightAll);
    dp.SyntaxHighlighter.HighlightAll('code', false);
    console.log(1);
}