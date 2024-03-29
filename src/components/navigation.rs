use leptos::*;

/// A parameterized incrementing button
#[component]
pub fn Navigation() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    view! {
    <nav class="bg-gray-800 ">
      <div class="flex items-center justify-between p-3">
        <div class="text-2xl text-white center"><a href="">"PJP"</a></div>
        <button on:click=move |_| {
          set_is_open.update(|n| *n = !*n);
          } 
          class="relative group">
          <div class="relative flex overflow-hidden items-center justify-center rounded-full w-[50px] h-[50px] transform transition-all bg-slate-700 ring-0 ring-gray-300 hover:ring-8 ring-opacity-30 duration-200 shadow-md">
            <div class="flex flex-col justify-between w-[20px] h-[20px] transform transition-all duration-300 origin-center overflow-hidden" 
              class=("-rotate-90", move || is_open())>
              <div class="bg-lime-400 h-[2px] w-7 transform transition-all duration-300 origin-left delay-150" class=("rotate-[35deg]", move || is_open())></div>
              <div class="bg-white h-[2px] w-7 rounded transform transition-all duration-300 " class=("translate-x-10", move || is_open())></div>
              <div class="bg-red-400 h-[2px] w-7 transform transition-all duration-300 origin-left delay-100" class=("-rotate-[35deg]", move || is_open())>
            </div>
            </div>
          </div>
        </button>
      </div>
      <div class=("hidden", move || !is_open())>
          <ul class="text-center text-xl text-white py-4">
            <li class="my-3"><a href="#chronologie">"Chronologie"</a></li>
            <li class="my-3"><a href="#objectifs">"Objectifs"</a></li>
          </ul>
      </div>
    </nav>
  } 
}
