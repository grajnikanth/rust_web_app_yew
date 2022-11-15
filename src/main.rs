use yew::prelude::*;

struct Model {
    value: i64
}

// appears app() function is for yew to use to render the webpage
// we are returing Html in this function
// function_component 
#[function_component(App)]
fn app() -> Html {
    // The below is similar to the hooks in react programming to store state
    // use_state - we are initializing using a callback function
    let state = use_state(|| Model {
        value: 0
    });

    // onclick function defined here is used in the html which when clicked will 
    //execute the function here. It is in the form of codeblock. Below we will be
    // returning the Callback function which in itself has a closure function
    // where are incremeting the counter. So this increment function will be executed
    // when we click
    let onclick = {
        let state = state.clone();

        // move was used so that closure function takes ownership of the state variable
        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    };

    // we rreturn the html shown below using the html! macro
    // 
    html! {
        <div>
            <button {onclick}> {"+1"} </button>
            <p> {state.value}</p>
        </div>
    }

}

fn main() {
    yew::start_app::<App>();
}