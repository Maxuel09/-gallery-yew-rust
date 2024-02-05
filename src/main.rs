use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
        // <img src="https://picsum.photos/200/300" alt=""/>
        <Nabvar />
        <Home />
        <Galery />
        </>
    }
}
#[function_component(Nabvar)]
fn navbar() -> Html {
    html! {
            <header>
           <nav class="navbar navbar-expand-lg bg-body-tertiary ">
                <div class="container-fluid mx-5">
        <a class="navbar-brand" href="#">{"Gallery"}</a>
        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
          <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarNav">
          <ul class="navbar-nav">
            <li class="nav-item">
              <a class="nav-link active" aria-current="page" href="#">{"Home"}</a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href="#">{"Features"}</a>
            </li>
            <li class="nav-item">
              <a class="nav-link" href="#">{"Pricing"}</a>
            </li>
            <li class="nav-item">
              <a class="nav-link disabled" aria-disabled="true">{"Disabled"}</a>
            </li>
          </ul>
        </div>
      </div>
    </nav>
    </header>
        }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <>
        <div class="container-fluid d-flex  "  >
        // <div class={css!("color: red;")}>{"Hello World!"}</div>
          <div class="container w-50 text-left  m-5  position-static" style="font-family: 'Playfair Display', serif;">
        <h1 class="display-1 mb-5" >{"La pagina del futuro"}</h1>
        <button class="btn btn-outline-dark btn-lg">{"Bienvenido"}</button>
        </div>

        <div class="w-50 m-5 ">
              <img class="rounded" src="https://picsum.photos/500/600" alt=""/>
        </div>
       </div>
       </>
    }
}

#[function_component(Galery)]
fn galery() -> Html {
    html! {
        <>


        <div class="container-fluid d-flex   "  >
        <div class="grid gap-3">
              <img class="rounded p-2 g-col-6" src="https://picsum.photos/200/200" alt=""/>
              <img class="rounded p-2 g-col-6" src="https://picsum.photos/200/200" alt=""/>
              <img class="rounded p-2 g-col-6" src="https://picsum.photos/200/200" alt=""/>
              <img class="rounded p-2 g-col-6" src="https://picsum.photos/200/200" alt=""/>
              <img class="rounded p-2 g-col-6" src="https://picsum.photos/200/200" alt=""/>
        </div>
       </div>
       </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
