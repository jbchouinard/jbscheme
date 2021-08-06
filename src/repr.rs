use crate::*;

pub fn repr(expr: &JVal) -> String {
    match &*expr {
        JVal::Nil => "()".to_string(),
        JVal::Int(n) => format!("{}", n),
        JVal::Bool(b) => (if *b { "true" } else { "false" }).to_string(),
        JVal::Symbol(s) => s.to_string(),
        JVal::String(s) => format!("\"{}\"", s),
        JVal::Error(e) => format!("#[error {}]", e),
        JVal::Builtin(b) => format!("#[function {}]", b),
        JVal::SpecialForm(b) => format!("#[specialform {}]", b),
        JVal::Lambda(_) => "#[lambda]".to_string(),
        JVal::Macro(_) => "#[macro]".to_string(),
        JVal::Pair(c) => repr_cell(c),
        JVal::Quote(val) => format!("'{}", repr(&*val)),
    }
}

fn repr_cell(cell: &JPair) -> String {
    match cell.iter() {
        Ok(iterator) => {
            let mut parts = vec!["(".to_string()];
            for val in iterator {
                parts.push(repr(&val));
                parts.push(" ".to_string());
            }
            if parts.len() > 1 {
                parts.pop();
            }
            parts.push(")".to_string());
            parts.join("")
        }
        // Not a list
        Err(_) => format!("({} . {})", repr(&cell.car()), repr(&cell.cdr())),
    }
}
