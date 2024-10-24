use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Container)]
pub fn container(
    ContainerProps {
        children,
        class,
    }: &ContainerProps,
) -> Html {
    let class_names = format!("{class} px-6 lg:px-8");
    html! {
        <div class={class_names}>
            <div class="mx-auto max-w-2xl lg:max-w-7xl">
                {children.clone()}
            </div>
        </div>
    }
}
