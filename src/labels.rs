use zed_extension_api::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed_extension_api::{CodeLabel, CodeLabelSpan};

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
            let len = label.len();
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    label.clone(),
                    Some("property".into()),
                )],
                filter_range: (0..len).into(),
                code: Default::default(),
            })
        }

        CompletionKind::Variable | CompletionKind::Value => {
            let len = label.len();
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    label.clone(),
                    Some("variable".into()),
                )],
                filter_range: (0..len).into(),
                code: Default::default(),
            })
        }

        CompletionKind::Class | CompletionKind::Interface => {
            let len = label.len();
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    label.clone(),
                    Some("type".into()),
                )],
                filter_range: (0..len).into(),
                code: Default::default(),
            })
        }

        CompletionKind::Module => {
            let len = label.len();
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    label.clone(),
                    Some("module".into()),
                )],
                filter_range: (0..len).into(),
                code: Default::default(),
            })
        }

        CompletionKind::Enum | CompletionKind::EnumMember => {
            let len = label.len();
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    label.clone(),
                    Some("enum".into()),
                )],
                filter_range: (0..len).into(),
                code: Default::default(),
            })
        }

        CompletionKind::Keyword => {
            let len = label.len();
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    label.clone(),
                    Some("keyword".into()),
                )],
                filter_range: (0..len).into(),
                code: Default::default(),
            })
        }

        _ => None,
    }
}

pub fn label_for_symbol(symbol: Symbol) -> Option<CodeLabel> {
    let (prefix, suffix) = match symbol.kind {
        SymbolKind::Method => ("local function ", "()"),
        SymbolKind::Function => ("local function ", ""),
        SymbolKind::Class => ("local ", " = {}"),
        SymbolKind::Interface => ("local ", " = {}"),
        SymbolKind::Enum => ("local ", " = {}"),
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
