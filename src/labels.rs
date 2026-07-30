use zed_extension_api::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed_extension_api::{CodeLabel, CodeLabelSpan};

pub fn label_for_completion(completion: Completion) -> Option<CodeLabel> {
    match completion.kind? {
        CompletionKind::Method | CompletionKind::Function => {
            let name_len = completion.label.find('(').unwrap_or(completion.label.len());
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(0..completion.label.len())],
                filter_range: (0..name_len).into(),
                code: completion.label,
            })
        }
        CompletionKind::Field => Some(CodeLabel {
            spans: vec![CodeLabelSpan::literal(
                completion.label.clone(),
                Some("property".into()),
            )],
            filter_range: (0..completion.label.len()).into(),
            code: Default::default(),
        }),
        _ => None,
    }
}

pub fn label_for_symbol(symbol: Symbol) -> Option<CodeLabel> {
    let prefix = "let a = ";
    let suffix = match symbol.kind {
        SymbolKind::Method => "()",
        _ => "",
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
