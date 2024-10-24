use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct LogoCloudProps {
    #[prop_or_default]
    pub class: String,
}

#[function_component(LogoCloud)]
pub fn logo_cloud(LogoCloudProps { class }: &LogoCloudProps) -> Html {
    let class_names = format!("{class} text-2xl font-medium text-gray-500");
    html! {
        <div class={class_names}>
            <img
                alt="SavvyCal"
                src="/logo-cloud/savvycal.svg"
                className="h-9 max-sm:mx-auto sm:h-8 lg:h-12"
            />
            <img
                alt="Laravel"
                src="/logo-cloud/laravel.svg"
                className="h-9 max-sm:mx-auto sm:h-8 lg:h-12"
            />
            <img
                alt="Tuple"
                src="/logo-cloud/tuple.svg"
                className="h-9 max-sm:mx-auto sm:h-8 lg:h-12"
            />
            <img
                alt="Transistor"
                src="/logo-cloud/transistor.svg"
                className="h-9 max-sm:mx-auto sm:h-8 lg:h-12"
            />
            <img
                alt="Statamic"
                src="/logo-cloud/statamic.svg"
                className="h-9 max-sm:mx-auto sm:h-8 lg:h-12"
            />
        </div>
    }
}
