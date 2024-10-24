use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: String,
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub href: Option<AttrValue>,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
}

fn get_variant_classes(variant: &ButtonVariant) -> String {
    match variant {
        &ButtonVariant::Primary => {
            r#"
            inline-flex items-center justify-center px-4 py-[calc(theme(spacing.2)-1px)]
            rounded-full border border-transparent bg-gray-950 shadow-md
            whitespace-nowrap text-base font-medium text-white
            data-[disabled]:bg-gray-950 data-[hover]:bg-gray-800 data-[disabled]:opacity-40
            "#.to_string()
        }
        &ButtonVariant::Secondary => {
            r#"
            relative inline-flex items-center justify-center px-4 py-[calc(theme(spacing.2)-1px)]
            rounded-full border border-transparent bg-white/15 shadow-md ring-1 ring-[#D15052]/15
            after:absolute after:inset-0 after:rounded-full after:shadow-[inset_0_0_2px_1px_#ffffff4d]
            whitespace-nowrap text-base font-medium text-gray-950
            data-[disabled]:bg-white/15 data-[hover]:bg-white/20 data-[disabled]:opacity-40
            "#.to_string()
        }
        &ButtonVariant::Outline => {
            r#"
            inline-flex items-center justify-center px-2 py-[calc(theme(spacing.[1.5])-1px)]
            rounded-lg border border-transparent shadow ring-1 ring-black/10
            whitespace-nowrap text-sm font-medium text-gray-950
            data-[disabled]:bg-transparent data-[hover]:bg-gray-50 data-[disabled]:opacity-40
            "#.to_string()
        }
    }
}

#[function_component(Button)]
pub fn button(
    ButtonProps {
        children,
        class,
        variant,
        href,
    }: &ButtonProps,
) -> Html {
    let variant_class_names = get_variant_classes(variant);
    let class_names = format!("{class} {variant_class_names}");

    html! {
        // <a href={href} class={class_names}>
        //     {children.clone()}
        // </a>
        if let Some(href) = href {
            <a href={href} class={class_names}>
                {children.clone()}
            </a>
        } else {
            <button type="button" class={class_names}>
                {children.clone()}
            </button>
        }
    }
}

