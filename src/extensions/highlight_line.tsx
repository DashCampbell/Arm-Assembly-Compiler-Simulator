import { Facet } from "@codemirror/state"
import { EditorView } from 'codemirror'
import { Extension, RangeSetBuilder } from "@codemirror/state"
import { Decoration, ViewPlugin, DecorationSet, ViewUpdate } from "@codemirror/view"

const highlightTheme = EditorView.baseTheme({
    ".cm-highlight": { backgroundColor: "palegreen !important", color: "dimgray" }
});

const highlight_line_index = Facet.define<number, number>({
    combine: values => values[0]
});

const highlight_dec = Decoration.line({
    attributes: { class: "cm-highlight" }
});

function highlight_line(view: EditorView) {
    let index = view.state.facet(highlight_line_index);
    let builder = new RangeSetBuilder<Decoration>();
    for (let i = 1; i <= view.state.doc.lines; i++) {
        if (index === i) {
            const line = view.state.doc.line(i);
            builder.add(line.from, line.from, highlight_dec)
            return builder.finish();
        }
    }
    return builder.finish();
}

const show_highlight = ViewPlugin.fromClass(class {
    decorations: DecorationSet

    constructor(view: EditorView) {
        this.decorations = highlight_line(view)
    }

    update(update: ViewUpdate) {
        this.decorations = highlight_line(update.view);
    }
}, {
    decorations: v => v.decorations
});

export function hightlight(line?: number): Extension {
    return [
        highlightTheme,
        highlight_line_index.of(line ?? 0),
        show_highlight
    ]
}