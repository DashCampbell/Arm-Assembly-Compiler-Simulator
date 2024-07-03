import { EditorView, gutter, GutterMarker } from "@codemirror/view"
import { EditorState, StateField, StateEffect, RangeSet, Extension, Facet } from "@codemirror/state"

const breakpointEffect = StateEffect.define<{ pos: number, on: boolean }>({
    map: (val, mapping) => ({ pos: mapping.mapPos(val.pos), on: val.on })
})

const breakpointState = StateField.define<RangeSet<GutterMarker>>({
    create(state) {
        let set = RangeSet.empty;
        const breakpoints = state.facet(file_breakpoints);
        const max_lines = state.doc.lines;
        // if breakpoint within document turn on breakpoint marker
        for (let bk of breakpoints) {
            if (0 < bk && bk <= max_lines)
                set = set.update({ add: [breakpointMarker.range(state.doc.line(bk).from)] });
        }
        return set;
    },
    update(set, transaction) {
        set = set.map(transaction.changes);
        for (let e of transaction.effects) {
            if (e.is(breakpointEffect)) {
                if (e.value.on)
                    set = set.update({ add: [breakpointMarker.range(e.value.pos)] })
                else
                    set = set.update({ filter: from => from != e.value.pos })
            }
        }
        return set
    }
});


function toggleBreakpoint(view: EditorView, pos: number) {
    let breakpoints = view.state.field(breakpointState);
    let hasBreakpoint = false;
    breakpoints.between(pos, pos, () => { hasBreakpoint = true });
    view.dispatch({
        effects: breakpointEffect.of({ pos, on: !hasBreakpoint })
    });
}

const breakpointMarker = new class extends GutterMarker {
    toDOM() { return document.createTextNode("●") }
}

const file_breakpoints = Facet.define<Array<number>, Array<number>>({
    combine: values => values.length ? values[0] : []
});

export function getBreakpoints(state: EditorState) {
    let breakpoints = [];
    for (let range = state.field(breakpointState).iter(); range.to <= state.doc.length; range.next())
        breakpoints.push(state.doc.lineAt(range.from).number);
    return breakpoints;
}
export function breakpointGutter(breakpoints: Array<number> = []): Extension {
    return [
        file_breakpoints.of(breakpoints),
        breakpointState,
        gutter({
            class: "cm-breakpoint-gutter",
            markers: v => v.state.field(breakpointState),
            initialSpacer: () => breakpointMarker,
            renderEmptyElements: true,
            domEventHandlers: {
                mousedown(view, line) {
                    toggleBreakpoint(view, line.from);
                    return true;
                }
            },
        }),
        EditorView.baseTheme({
            ".cm-breakpoint-gutter .cm-gutterElement": {
                color: "red",
                padding: "0 3px",
                cursor: "pointer"
            },
            ".cm-breakpoint-gutter .cm-gutterElement:hover": {
                backgroundColor: "#b6b6b6",
            }
        })
    ];
}

export function noFold(): Extension {
    return [
        EditorView.baseTheme({
            ".cm-foldGutter": {
                display: "none !important",
                backgroundColor: "white",
            }
        })
    ]
} 