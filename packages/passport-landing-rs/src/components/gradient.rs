use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GradientProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Gradient)]
pub fn gradient(
    GradientProps {
        children,
        class,
    }: &GradientProps
) -> Html {
    let class_names = format!("{class} bg-[linear-gradient(115deg,var(--tw-gradient-stops))] from-[#fff1be] from-[28%] via-[#ee87cb] via-[70%] to-[#b060ff] sm:bg-[linear-gradient(145deg,var(--tw-gradient-stops))]");
    html! {
        <div
            class={class_names}
        >
            {children.clone()}
        </div>
    }
}

#[function_component(GradientBackground)]
pub fn gradient_background() -> Html {
    let class_names = r#"
        absolute -right-60 -top-44 h-60 w-[36rem] transform-gpu md:right-0
        bg-[linear-gradient(115deg,var(--tw-gradient-stops))] from-[#fff1be] from-[28%] via-[#ee87cb] via-[70%] to-[#b060ff]
        rotate-[-10deg] rounded-full blur-3xl
    "#;
    html! {
        <div class="relative mx-auto max-w-7xl">
            <div
                class={class_names}
            />
        </div>
    }
}
