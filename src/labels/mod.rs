use zed_extension_api::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed_extension_api::{CodeLabel, CodeLabelSpan};

fn literal_label(label: &str, style: &str) -> CodeLabel {
    let len = label.len();
    CodeLabel {
        spans: vec![CodeLabelSpan::literal(label.to_owned(), Some(style.into()))],
        filter_range: (0..len).into(),
        code: Default::default(),
    }
}

pub fn label_for_completion(completion: Completion) -> Option<CodeLabel> {
    let kind = completion.kind?;
    let label = completion.label;

    match kind {
        CompletionKind::Method | CompletionKind::Function => {
            let name_len = label.find('(').unwrap_or(label.len());
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(0..label.len())],
                filter_range: (0..name_len).into(),
                code: label,
            })
        }
        CompletionKind::Field | CompletionKind::Property => {
            Some(literal_label(&label, "property"))
        }
        CompletionKind::Variable | CompletionKind::Value => {
            Some(literal_label(&label, "variable"))
        }
        CompletionKind::Class | CompletionKind::Interface => {
            Some(literal_label(&label, "type"))
        }
        CompletionKind::Module => Some(literal_label(&label, "module")),
        CompletionKind::Enum | CompletionKind::EnumMember => {
            Some(literal_label(&label, "enum"))
        }
        CompletionKind::Keyword => Some(literal_label(&label, "keyword")),
        _ => None,
    }
}

pub fn label_for_symbol(symbol: Symbol) -> Option<CodeLabel> {
    let (prefix, suffix) = match symbol.kind {
        SymbolKind::Method => ("local function ", "()"),
        SymbolKind::Function => ("local function ", ""),
        SymbolKind::Class | SymbolKind::Interface | SymbolKind::Enum => ("local ", " = {}"),
        SymbolKind::Module => ("require ", ""),
        SymbolKind::Variable | SymbolKind::Constant => ("local ", ""),
        _ => ("local ", ""),
    };

    let code = format!("{prefix}{}{suffix}", symbol.name);

    Some(CodeLabel {
        spans: vec![CodeLabelSpan::code_range(
            prefix.len()..code.len() - suffix.len(),
        )],
        filter_range: (0..symbol.name.len()).into(),
        code,
    })
}
