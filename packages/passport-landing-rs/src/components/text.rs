use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HeadingProps {
    #[prop_or("h2".to_string())]
    pub as_element: String,
    #[prop_or(false)]
    pub dark: bool,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Heading)]
pub fn heading(
    HeadingProps {
        as_element,
        class,
        dark,
        children,
    }: &HeadingProps,
) -> Html {
    let class_names = format!("{class} text-pretty text-4xl font-medium tracking-tighter text-gray-950 data-[dark]:text-white sm:text-6xl");
    let dark_data = if *dark { Some("true") } else { None };
    html! {
        <@{as_element.to_owned()}
            class={class_names}
            dark-data={dark_data}
        >
            {children.clone()}
        </@>
    }
}

#[function_component(Subheading)]
pub fn subheading(HeadingProps {
    as_element,
    class,
    dark,
    children,
}: &HeadingProps) -> Html {
    let class_names = format!("{class} font-mono text-xs/5 font-semibold uppercase tracking-widest text-gray-500 data-[dark]:text-gray-400");
    let dark_data = if *dark { Some("true") } else { None };
    html! {
        <@{as_element.to_owned()}
            class={class_names}
            dark-data={dark_data}
        >
            {children.clone()}
        </@>
    }
}

#[derive(PartialEq, Properties)]
pub struct LeadProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: String,
}


#[function_component(Lead)]
pub fn lead(
    LeadProps {
        children,
        class,
    }: &LeadProps
) -> Html {
    let class_names = format!("{class} text-2xl font-medium text-gray-500");
    html! {
        <p
            class={class_names}
        >
            {children.clone()}
        </p>
    }
}
