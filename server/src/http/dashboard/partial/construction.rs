use maud::{html, Markup};

#[allow(dead_code)]
pub fn construction(warning: &str) -> Markup {
    html! {
        .bg-background-floating.rounded-md.flex.items-center."gap-8"."p-8".shadow-xl.max-w-sm {
            ."text-4xl" { "🚧" }
            .flex.flex-col."gap-2" {
                p { strong.text-text-heading { "Danger due to..." } }
                p { (warning) }
            }
        }
    }
}
