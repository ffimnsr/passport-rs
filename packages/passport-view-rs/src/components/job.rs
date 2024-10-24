use chrono::DateTime;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::{prelude::*, suspense::use_future};
use yew_router::prelude::*;
use bounce::prelude::*;
use crate::components::{realize_contract_type, realize_date, title_case};
use crate::model::*;
use crate::markdown;
use crate::route::{Route, JobsRoute};
use crate::state::State;
use super::Modal;

const BACKEND: Option<&str> = option_env!("BACKEND");

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
struct WorkFunctionsContainer {
    pub work_functions: WorkFunctions,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
struct WorkIndustriesContainer {
    pub work_industries: WorkIndustries,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct JobContainer {
    pub jobs: Jobs,
}

#[derive(PartialEq, Properties)]
struct JobsListProps {
    pub jobs: Jobs,
}

#[function_component(JobsList)]
fn jobs_list(JobsListProps { jobs }: &JobsListProps) -> Html {
    let state = use_atom_value::<State>();
    let jobs_len = jobs.len();
    let posts = jobs.iter().map(|job| html! {
        <li key={job.id.clone()}>
            <Link<JobsRoute> to={JobsRoute::JobDetail { job_id: job.id.clone() }}>
                <div class="px-4 py-4 sm:px-6">
                    <div class="flex items-center justify-between">
                        <div class="truncate text-sm font-medium text-indigo-600">{title_case(&job.title.clone())}</div>
                        <div class="ml-2 flex flex-shrink-0">
                            <span class="inline-flex items-center rounded-full bg-green-50 px-2 py-1 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20">
                                {realize_contract_type(job.work_contract_type.clone())}
                            </span>
                        </div>
                    </div>
                    <div class="mt-2 flex justify-between">
                        <div class="sm:flex">
                            <div class="flex items-center text-sm text-gray-500">
                                <svg class="mr-1.5 h-4 w-4 flex-shrink-0 text-gray-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                    <path d="M7 8a3 3 0 100-6 3 3 0 000 6zM14.5 9a2.5 2.5 0 100-5 2.5 2.5 0 000 5zM1.615 16.428a1.224 1.224 0 01-.569-1.175 6.002 6.002 0 0111.908 0c.058.467-.172.92-.57 1.174A9.953 9.953 0 017 18a9.953 9.953 0 01-5.385-1.572zM14.5 16h-.106c.07-.297.088-.611.048-.933a7.47 7.47 0 00-1.588-3.755 4.502 4.502 0 015.874 2.636.818.818 0 01-.36.98A7.465 7.465 0 0114.5 16z"></path>
                                </svg>
                                {state.get_industry_by_id(job.industry_id)}
                            </div>
                        </div>
                        <div class="ml-2 flex items-center text-sm text-gray-500">
                            <svg class="mr-1.5 h-4 w-4 flex-shrink-0 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                            </svg>
                            {realize_date(&job.created_at.clone())}
                        </div>
                    </div>
                </div>
            </Link<JobsRoute>>
        </li>
    }).collect::<Vec<_>>();

    html! {
        if jobs_len > 0 {
            <ul role="list" class="divide-y divide-gray-200">
                {for posts}
            </ul>
        } else {
            <div class="p-10 text-center">
                <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                    <path vector-effect="non-scaling-stroke" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
                </svg>
                <h3 class="mt-2 text-sm font-semibold text-gray-900">{"No jobs found"}</h3>
                <p class="mt-1 text-sm text-gray-500">{"Get started by creating a new job post."}</p>
                <div class="mt-6">
                    <a href="https://t.me/osslocal"
                        type="button"
                        class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                        <svg class="-ml-0.5 mr-1.5 h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true"
                            data-slot="icon">
                            <path
                                d="M10.75 4.75a.75.75 0 0 0-1.5 0v4.5h-4.5a.75.75 0 0 0 0 1.5h4.5v4.5a.75.75 0 0 0 1.5 0v-4.5h4.5a.75.75 0 0 0 0-1.5h-4.5v-4.5Z" />
                        </svg>
                        {"New Job Post"}
                    </a>
                </div>
            </div>
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct JobsSearchModalProps {
    pub onclick_modal_show_button: Callback<MouseEvent>,
}

#[function_component(JobsSearchModal)]
pub fn jobs_search_modal(props: &JobsSearchModalProps) -> Html {
    html! {
        <Modal>
            <div class="relative z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" aria-hidden="true"></div>
                <div class="fixed inset-0 z-10 w-screen overflow-y-auto dark:invert">
                    <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                        <div class="relative transform overflow-hidden rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm sm:p-6">
                            <div>
                                <label for="search" class="block text-sm font-medium leading-6 text-gray-900">{"Quick search"}</label>
                                <div class="relative mt-2 flex items-center">
                                    <input
                                        type="text"
                                        name="search" 
                                        id="search"
                                        class="block w-full rounded-md border-0 py-1.5 pr-14 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                                    <div class="absolute inset-y-0 right-0 flex py-1.5 pr-1.5">
                                        <kbd class="inline-flex items-center rounded border border-gray-200 px-1 font-sans text-xs text-gray-400">{"⌘K"}</kbd>
                                    </div>
                                </div>
                            </div>
                            <div class="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
                                <button onclick={props.onclick_modal_show_button.clone()} type="button" class="inline-flex w-full justify-center rounded-md bg-green-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-green-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-green-600 sm:col-start-2">
                                    {"Search"}
                                </button>
                                <button onclick={props.onclick_modal_show_button.clone()} type="button" class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:col-start-1 sm:mt-0">
                                    {"Cancel"}
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Modal>
    }
}

#[function_component(JobsViewHeading)]
pub fn jobs_view_heading() -> Html {
    let modal_show = use_state(|| false);
    let toggle_modal_show = {
        let modal_show = modal_show.clone();
        Callback::from(move |_: MouseEvent| modal_show.set(!*modal_show))
    };

    html! {
        <div class="border-b border-gray-200 bg-white px-4 py-5 sm:px-6">
            <div class="-ml-4 -mt-4 flex flex-wrap items-center justify-between sm:flex-nowrap">
                <div class="ml-4 mt-4">
                    <h3 class="text-base font-semibold leading-6 text-gray-900">{"Job Postings"}</h3>
                    <p class="mt-1 text-sm text-gray-500">
                        {"Find your next remote career opportunity with us!"}
                    </p>
                </div>
                <div class="ml-4 mt-4 flex-shrink-0">
                    <button onclick={toggle_modal_show.clone()} type="button" class="relative inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                        {"Search Job"}
                    </button>
                    if *modal_show {
                        <JobsSearchModal onclick_modal_show_button={toggle_modal_show} />
                    }
                </div>
            </div>
        </div>
    }
}

#[function_component(JobsView)]
pub fn jobs_view() -> HtmlResult {
    let gstate = use_atom::<State>();
    
    let jobs = use_future(|| async {
        let base_url = BACKEND.expect("BACKEND environment variable not set");
        let url: String = format!("{base_url}/jobs",);
        Request::get(&url)
            .send()
            .await?
            .json::<JobContainer>()
            .await
    })?;

    {
        let work_functions = use_future(|| async {
            let base_url = BACKEND.expect("BACKEND environment variable not set");
            let url: String = format!("{base_url}/work/functions",);
            Request::get(&url)
                .send()
                .await?
                .json::<WorkFunctionsContainer>()
                .await
        })?;
    
        let work_industries = use_future(|| async {
            let base_url = BACKEND.expect("BACKEND environment variable not set");
            let url: String = format!("{base_url}/work/industries",);
            Request::get(&url)
                .send()
                .await?
                .json::<WorkIndustriesContainer>()
                .await
        })?;

        gstate.set(State {
            work_functions: work_functions
                .as_ref()
                .unwrap_or(&WorkFunctionsContainer::default())
                .work_functions
                .clone(),
            work_industries: work_industries
                .as_ref()
                .unwrap_or(&WorkIndustriesContainer::default())
                .work_industries
                .clone(),
        });
    }

    let jobs = jobs.as_ref().ok();
    let jobs = jobs
        .unwrap_or(&JobContainer { jobs: vec![] })
        .jobs
        .clone();

    Ok(html! {
        <div class="overflow-hidden bg-white sm:rounded-lg sm:shadow">
            <JobsViewHeading />
            <JobsList jobs={jobs} />
        </div>
    })
}

#[function_component(JobsViewLoading)]
pub fn jobs_view_loading() -> Html {
    let items = (1..=4).into_iter().map(|_| html! {
        <li>
            <a href="#" class="block hover:bg-gray-50">
                <div class="px-4 py-4 sm:px-6">
                    <div class="flex items-center justify-between">
                        <div class="truncate text-sm font-medium text-indigo-600 h-5 w-64 rounded bg-slate-700"></div>
                        <div class="ml-2 flex flex-shrink-0 h-5 w-24 rounded bg-slate-700"></div>
                    </div>
                    <div class="mt-2 flex justify-between">
                        <div class="sm:flex">
                            <div class="flex items-center text-sm text-gray-500 h-5 w-28 rounded bg-slate-700"></div>
                        </div>
                        <div class="ml-2 flex items-center text-sm text-gray-500 h-5 w-20 rounded bg-slate-700"></div>
                    </div>
                </div>
            </a>
        </li>
    }).collect::<Vec<_>>();

    html! {
        <div class="overflow-hidden bg-white sm:rounded-lg sm:shadow">
            <JobsViewHeading />
            <ul role="list" class="animate-pulse divide-y divide-gray-200">
                {for items}
            </ul>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct JobDetailViewProps {
    pub job_id: String,
}

#[function_component(JobDetailView)]
pub fn job_detail_view(props: &JobDetailViewProps) -> HtmlResult {
    let job_id = props.job_id.clone();
    let jobs = use_future(move || async move {
        let base_url = BACKEND.expect("BACKEND environment variable not set");
        let url: String = format!("{}/jobs/{}", base_url, &job_id);
        Request::get(&url).send().await?.json::<Job>().await
    })?;

    let location_href = web_sys::window()
        .and_then(|w| w.location().href().ok())
        .unwrap_or_else(|| "Unknown".to_string());

    let location_href = url::form_urlencoded::byte_serialize(location_href.as_bytes())
        .collect::<String>();

    let fb_share_url = format!(
        "https://www.facebook.com/sharer/sharer.php?u={}",
        location_href
    );

    let in_share_url = format!(
        "https://www.linkedin.com/shareArticle?mini=true&url={}",
        location_href
    );

    let job = jobs.as_ref();
    if job.is_err() {
        return Ok(html! {
            <Redirect<Route> to={Route::NotFound} />
        });
    }
    let job = job.unwrap();
    let Job {
        title,
        description,
        work_experience_level,
        work_contract_type,
        has_timetracker,
        created_at,
        ..
    } = job.clone();
    let dt = DateTime::parse_from_rfc3339(&created_at).unwrap();
    let created_at = dt.format("%Y-%m-%d").to_string();

    let experience_level = match work_experience_level {
        Some(experience_level) => html! {
            <span class="inline-flex items-center gap-x-1.5 rounded-md bg-gray-100 px-2 py-1 text-xs font-medium text-gray-600">
                <svg class="h-1.5 w-1.5 fill-gray-400" viewBox="0 0 6 6" aria-hidden="true">
                    <circle cx="3" cy="3" r="3" />
                </svg>
                {experience_level.to_uppercase()}
            </span>
        },
        _ => html! {
            {"Not Specified"}
        },
    };

    let work_type = match work_contract_type {
        Some(work_type) => html! {
            <span class="inline-flex items-center gap-x-1.5 rounded-md bg-gray-100 px-2 py-1 text-xs font-medium text-gray-600">
                <svg class="h-1.5 w-1.5 fill-gray-400" viewBox="0 0 6 6" aria-hidden="true">
                    <circle cx="3" cy="3" r="3" />
                </svg>
                {work_type.to_uppercase()}
            </span>
        },
        _ => html! {
            {"Not Specified"}
        },
    };

    let has_timetracker = match has_timetracker {
        true => html! {
            <span class="inline-flex items-center gap-x-1.5 rounded-md dark:invert dark:text-white bg-green-400 px-2 py-1 text-xs font-medium text-gray-600">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-4">
                    <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z" clip-rule="evenodd" />
                </svg>
                {" YES"}
            </span>
        },
        false => html! {
            <span class="inline-flex items-center gap-x-1.5 rounded-md dark:invert dark:text-white bg-red-400 px-2 py-1 text-xs font-medium text-gray-600">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-4">
                    <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25Zm-1.72 6.97a.75.75 0 1 0-1.06 1.06L10.94 12l-1.72 1.72a.75.75 0 1 0 1.06 1.06L12 13.06l1.72 1.72a.75.75 0 1 0 1.06-1.06L13.06 12l1.72-1.72a.75.75 0 1 0-1.06-1.06L12 10.94l-1.72-1.72Z" clip-rule="evenodd" />
                </svg>
                {" NO"}
            </span>
        },
    };

    let description = description.as_str();
    let description = html! {
        markdown::render_markdown(description)
    };

    Ok(html! {
        <>
            <div class="overflow-hidden bg-white shadow sm:rounded-lg">
                <div class="px-4 py-6 sm:px-6">
                    <h3 class="text-base font-semibold leading-7 text-gray-900">{"Job Information"}</h3>
                    <p class="mt-1 max-w-2xl text-sm leading-6 text-gray-500">{"Job description and role information."}</p>
                </div>
                <div class="border-t border-gray-100">
                    <dl class="divide-y divide-gray-100">
                        <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-900">{"Job title"}</dt>
                            <dd class="mt-1 text-sm leading-6 text-gray-700 sm:col-span-2 sm:mt-0">{title_case(&title)}</dd>
                        </div>
                        <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-900">{"Company name"}</dt>
                            <dd class="mt-1 text-sm leading-6 text-gray-700 sm:col-span-2 sm:mt-0">{"Margot Foster"}</dd>
                        </div>
                        <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-900">{"Experience level"}</dt>
                            <dd class="mt-1 text-sm leading-6 text-gray-700 sm:col-span-2 sm:mt-0">{experience_level}</dd>
                        </div>
                        <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-900">{"Work type"}</dt>
                            <dd class="mt-1 text-sm leading-6 text-gray-700 sm:col-span-2 sm:mt-0">{work_type}</dd>
                        </div>
                        <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-900">{"Salary range"}</dt>
                            <dd class="mt-1 text-sm leading-6 text-gray-700 sm:col-span-2 sm:mt-0">{"USD 120,000 - 140,000"}</dd>
                        </div>
                        <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-900">{"Uses timetracker?"}</dt>
                            <dd class="mt-1 text-sm leading-6 text-gray-700 sm:col-span-2 sm:mt-0">{has_timetracker}</dd>
                        </div>
                        <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-900">{"Resource person"}</dt>
                            <dd class="mt-1 text-sm leading-6 text-gray-700 sm:col-span-2 sm:mt-0">{"margotfoster@example.com"}</dd>
                        </div>
                        <div class="px-4 py-6 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-900">{"Post date"}</dt>
                            <dd class="mt-1 text-sm leading-6 text-gray-700 sm:col-span-2 sm:mt-0">{created_at}</dd>
                        </div>
                    </dl>
                </div>
            </div>
            <div class="divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow mt-4">
                <div class="px-4 py-6 sm:px-6">
                    <h3 class="text-base font-semibold leading-7 text-gray-900">{"Job Description"}</h3>
                </div>
                <div class="px-4 py-6 sm:p-6">
                    {description}
                </div>
            </div>
            <div class="mt-10 flex flex-1 gap-x-2">
                <span>{"Share to: "}</span>
                <a class="hover:text-blue-500" href={fb_share_url} target="_blank">
                    <svg viewBox="0 0 448 512" fill="currentColor" class="size-8">
                        <path d="M64 32C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64h98.2V334.2H109.4V256h52.8V222.3c0-87.1 39.4-127.5 125-127.5c16.2 0 44.2 3.2 55.7 6.4V172c-6-.6-16.5-1-29.6-1c-42 0-58.2 15.9-58.2 57.2V256h83.6l-14.4 78.2H255V480H384c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H64z"/>
                    </svg>
                </a>
                <a class="hover:text-blue-500" href={in_share_url} target="_blank">
                    <svg viewBox="0 0 448 512" fill="currentColor" class="size-8">
                        <path d="M416 32H31.9C14.3 32 0 46.5 0 64.3v383.4C0 465.5 14.3 480 31.9 480H416c17.6 0 32-14.5 32-32.3V64.3c0-17.8-14.4-32.3-32-32.3zM135.4 416H69V202.2h66.5V416zm-33.2-243c-21.3 0-38.5-17.3-38.5-38.5S80.9 96 102.2 96c21.2 0 38.5 17.3 38.5 38.5 0 21.3-17.2 38.5-38.5 38.5zm282.1 243h-66.4V312c0-24.8-.5-56.7-34.5-56.7-34.6 0-39.9 27-39.9 54.9V416h-66.4V202.2h63.7v29.2h.9c8.9-16.8 30.6-34.5 62.9-34.5 67.2 0 79.7 44.3 79.7 101.9V416z"/>
                    </svg>
                </a>
            </div>
        </>
    })
}

#[function_component(JobDetailViewLoading)]
pub fn job_detail_view_loading() -> Html {
    let items = (1..=4).into_iter().map(|_| html! {
        <li>
            <a href="#" class="block hover:bg-gray-50">
                <div class="px-4 py-4 sm:px-6">
                    <div class="flex items-center justify-between">
                        <div class="truncate text-sm font-medium text-indigo-600 h-5 w-64 rounded bg-slate-700"></div>
                        <div class="ml-2 flex flex-shrink-0 h-5 w-24 rounded bg-slate-700"></div>
                    </div>
                    <div class="mt-2 flex justify-between">
                        <div class="sm:flex">
                            <div class="flex items-center text-sm text-gray-500 h-5 w-28 rounded bg-slate-700"></div>
                        </div>
                        <div class="ml-2 flex items-center text-sm text-gray-500 h-5 w-20 rounded bg-slate-700"></div>
                    </div>
                </div>
            </a>
        </li>
    }).collect::<Vec<_>>();

    html! {
        <div class="overflow-hidden bg-white sm:rounded-lg sm:shadow">
            <JobsViewHeading />
            <ul role="list" class="animate-pulse divide-y divide-gray-200">
                {for items}
            </ul>
        </div>
    }
}
