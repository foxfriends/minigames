use maud::{html, Markup};

pub fn field(label: &str, name: &str, children: Markup) -> Markup {
    html! {
        .flex.flex-col."gap-1" {
            label.uppercase.text-xs.font-semibold for=(name) {
                (label)
            }
            (children)
        }
    }
}

pub fn info_field(label: &str, children: Markup) -> Markup {
    html! {
        .flex.flex-col."gap-1" {
            span.uppercase.text-xs.font-semibold {
                (label)
            }
            (children)
        }
    }
}

pub fn text_input(name: &str, placeholder: &str, value: &str) -> Markup {
    html! {
        input.outline-none.border.border-divider-dark.bg-background-secondary."focus:border-blurple-default".text-text-input.rounded-sm."p-2".transition-colors #(name) name=(name) placeholder=(placeholder) value=(value);
    }
}

pub fn switch(name: &str, checked: bool) -> Markup {
    html! {
        input.hidden #(name) type="checkbox" checked[checked] name=(name);
        label.switch for=(name) { .switch-toggle {} }
    }
}
