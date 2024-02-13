<h1>
    <a href="https://github.com/friendlymatthew/leptos-hotkeys">
    <em>leptos-hotkeys</em>    
</a>
</h1>

Declaratively create and pair keybindings with callbacks for Leptos applications. 

[![crates](https://img.shields.io/badge/📦_crates-0.1.3-%20green)](https://crates.io/crates/leptos_hotkeys)
[![discord](https://img.shields.io/badge/Join-Discord-%235865F2.svg)](https://discord.gg/XhVbKk38ux)
<!-- [![version](https://img.shields.io/badge/version-0.1.3-purple)](https://materialize.com/s/chat) -->


<a href="https://github.com/friendlymatthew/leptos-hotkeys">
    <img width="570" alt="Screen Shot 2024-01-07 at 4 13 48 PM" src="https://github.com/friendlymatthew/leptos_hotkeys/assets/38759997/f3c7b6ee-e6fd-4c0d-90be-ad26ca4e2ec6">
</a>

> [!NOTE]
> 
> This library is ready for use.
> If you're curious read the [CHANGELOG](#changelog).


## Live example 
Curious to see how it works? [See the demo!](https://leptos-hotkeys.vercel.app/)

To get started, follow the [Quick Start](#quick-start) section. It's worth the read!
## Features

### `use_hotkeys!` Macro
For simplicity and ease, use the `use_hotkeys!` macro to declare global and scoped hotkeys.<br>
We brought some js idioms while maintaining the leptos look.
[Learn more about the macro.](#macro-api) <br>

If you prefer writing out your callbacks the leptos way, we also have non-macro hotkeys. [Learn more about trad hotkeys.](#trad-hotkeys) 

### Global Hotkeys
> This example creates two global hotkeys: `W` and `S`. 
> 
> For more information about how to write your keybindings, check out [Key Grammar](#keybinding-grammar).
>
> *Note: the `*` symbol is reserved for the global scope*

```rust
use leptos_hotkeys::prelude::*;

#[component]
pub fn SomeComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0); 

    // creating a global scope for the W key
    use_hotkeys!(("w") => move |_| {
        logging::log!("s has been pressed"); 
        set_count.update(|c| *c += 1);
    });

    // this is also a global scope for the S key!
    use_hotkeys!(("s", "*") => move |_| {
        logging::log!("t has been pressed");
        set_count.update(|c| *c -= 1);
    });

    view! {
        <p>Current count: {count}</p> 
    }
}
```

### Scoped Hotkeys

> Assign hotkeys specific to individual sections without collisions using scopes.<br/>
> Use functions in `HotkeysContext` for scope management.
>
> This example shows an inner and outer scope and hotkeys that switch between the scopes.
>
> For more information about how to write your keybindings, check out [Key Grammar](#keybinding-grammar).
> 
> *Note: scopes are case-insensitive. That means `wef_scope` and `WEf_sCoPe` are considered the same scope.*
> 
```rust
use leptos_hotkeys::prelude::*;

#[component]
pub fn SomeComponent() -> impl IntoView {

    let HotkeysContext { enable_scope, disable_scope, .. } = use_hotkeys_context();

    // switch into the inner scope 
    use_hotkeys!(("i", "outer") => move |_| {
        disable_scope("outer");
        enable_scope("inner");
    });

    // switch into the outer scope
    use_hotkeys!(("o", "inner") => move |_| {
        disable_scope("inner");
        enable_scope("outer"); 
    });

    view! {
        <div id="outer">
            //...some outer scope html...
            <div id="inner">
            //...some inner scope html...
            </div>
            //...some outer scope html.... 
        </div>
    }
}
```


### Focus trapped Hotkeys
> Embed a hotkey with an `HtmlElement` and the hotkey will only fire if the element is focused and the scope is enabled.

```rust
use leptos_hotkeys::prelude::*;

#[component] 
pub fn SomeComponent() -> impl IntoView {

    let p_ref = use_hotkeys_ref!(("K", "*") => move |_| {
        // some logic 
    });

    view! {
        <p
            tabIndex=-1
            _ref=p_ref 
        >
            p tag with node ref 
        </p> 
    }
}
```

## Quick Start

### Installation
```shell
cargo add leptos_hotkeys
```

### Hotkey Provider
Wrap your project with `<HotkeysProvider />`:
```html
view! {
    <HotkeysProvider>
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/:else" view=ErrorPage/>
            </Routes>
        </Router>
    </HotkeysProvider>
}
```

### Initialize scopes
If you're using [scopes](#scoped-hotkeys), you can initialize with a specific scope.
```rust
use leptos_hotkeys::scopes;

view! {
    <HotkeysProvider
        initially_active_scopes=scopes!("some_scope_id") 
    >
        <Router>
            //... routes
        </Router>
    </HotkeysProvider>
}
```

Thats it! Start creating [hotkeys](#features)!

### Keybinding Grammar
`leptos_hotkeys` matches key values from [KeyboardEvent's](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key) `key` property.
<br />
For reference, here's a list of [all key values for keyboard events](https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values).
<br /><br />
You can bind multiple hotkeys to a callback. For example:
```
"G+R,meta+O,control+k"
```
The above example creates three hotkeys: `G+R`, [Meta](https://www.basketball-reference.com/players/a/artesro01.html)`+O`, and `Control+K`.
The `+` symbol is used to create a combo hotkey. A combo hotkey is a keybinding requiring more than one key press. 
Note that keys are case-agnostic and whitespace-agnostic. You use the `,` as a delimiter in a sequence of multiple hotkeys.



## Macro API
We wanted to strip the verbosity that comes with `str` and `String` type handling.<br> 
We kept leptos best practices in mind, keeping the `move |_|` idiom in our macro.

### `use_hotkeys!()`
Here is a general look at the macro:
```rust
use leptos_hotkeys::prelude::*;

use_hotkeys!(("keys", "scope") => move |_| {
    // callback logic here 
});
```

For global hotkeys, you can omit the second parameter as it will implicitly add the global scope.
```rust
use_hotkeys!(("key") => move |_| {
    // callback logic here 
});
```

### `use_hotkeys_ref!()`
This macro is used when you want to focus trap with a specific html element.

```rust
use leptos_hotkeys::prelude::*;

#[component]
pub fn SomeComponent() -> impl IntoView {
    let some_ref = use_hotkeys_ref!(("key", "scope") => move |_| {
        // callback logic here 
    });

    view! {
        <div tabIndex=-1 _ref=some_ref>
        </div>
    }
}
```

### `scopes!()`
Maybe you want to initialize a certain scope upon load, that's where the prop `initially_active_scopes` comes into play.
Instead of having to create a `vec!["scope_name".to_string()]`, use the `scopes!()` macro.

```rust
use leptos_hotkeys::prelude::*;

view! {
    <HotkeysProvider
        initially_active_scopes=scopes!("scope_a", "settings_scope");
    >
        // pages here...
    </HotkeysProvider>
}
```


## API
### `<HotkeysProvider />`

| Prop Name                 | Type              | Default Value                 | Description                                                                                                                                                                          |
|---------------------------|-------------------|-------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `allow_blur_event`        | `bool`            | `false`                       | Determines if the component should reset `pressed_keys` when a blur event occurs on the window. This is useful for resetting the state when the user navigates away from the window. |
| `initially_active_scopes` | `HashSet<String>` | `scopes!("*")` (Global State) | Specifies the set of scopes that are active when the component mounts. Useful for initializing the component with a predefined set of active hotkey scopes.                          |

### `HotkeysContext`
| Field Name          | Type                            | Description                                                                                           |
|---------------------|---------------------------------|-------------------------------------------------------------------------------------------------------|
| `pressed_keys`      | `RwSignal<HashSet<String>>`     | A reactive signal tracking the set of keys currently pressed by the user.                             |
| `active_ref_target` | `RwSignal<Option<EventTarget>>` | A reactive signal holding the currently active event target, useful for focusing events.              |
| `set_ref_target`    | `Callback<Option<EventTarget>>` | A method to update the currently active event target.                                                 |
| `active_scopes`     | `RwSignal<HashSet<String>>`     | A reactive signal tracking the set of currently active scopes, allowing for scoped hotkey management. |
| `enable_scope`      | `Callback<String>`              | A method to activate a given hotkey scope.                                                            |
| `disable_scope`     | `Callback<String>`              | A method to deactivate a given hotkey scope.                                                          |
| `toggle_scope`      | `Callback<String>`              | A method to toggle the activation state of a given hotkey scope.                                      |

### Basic Types
#### Keyboard Modifiers
| Field Name | Type  | Description                                                                                                                                                                     |
|------------|-------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `alt`      | `bool`| Indicates if the Alt key modifier is active (true) or not (false).                                                                                                              |
| `ctrl`     | `bool`| Indicates if the Control (Ctrl) key modifier is active (true) or not (false).                                                                                                   |
| `meta`     | `bool`| Indicates if the [Meta](https://www.basketball-reference.com/players/a/artesro01.html) (Command on macOS, Windows key on Windows) key modifier is active (true) or not (false). |
| `shift`    | `bool`| Indicates if the Shift key modifier is active (true) or not (false).                                                                                                            |

#### Hotkey
| Field Name  | Type                | Description                                                                                                                                    |
|-------------|---------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| `modifiers` | `KeyboardModifiers` | The set of key modifiers (Alt, Ctrl, [Meta](https://www.basketball-reference.com/players/a/artesro01.html), Shift) associated with the hotkey. |
| `keys`      | `Vec<String>`       | The list of keys that, along with any modifiers, define the hotkey.                                                                            |
| `description`| `String`           | A human-readable description of what the hotkey does. Intended for future use with scopes.                                                     |

## Trad Hotkeys
If the macro isn't to your liking, we offer three hotkeys: global, scoped, and focus trapped.

### Global: `use_hotkeys_scoped()` where scope = `*`
```rust
use leptos_hotkeys::{use_hotkeys_scoped};

#[component]
fn Component() -> impl IntoView {  
    let (count, set_count) = create_signal(0);

    use_hotkeys_scoped(
        "F", // the F key
        Callback::new(move |_| {
            set_count.update(|count| {
            *count += 1; 
            }) 
        }),
        vec!["*"]
    );

    view! {
        <p>
        Press 'F' to pay respect. 
        {count} times    
        </p>
    } 
}
```

### Scoped - `use_hotkeys_scoped` 

```rust
use leptos_hotkeys::{
    use_hotkeys_scoped, use_hotkeys_context, HotkeysContext
};

#[component]
fn Component() -> impl IntoView {
    let hotkeys_context: HotkeysContext = use_hotkeys_context();

    let toggle = hotkeys_context.toggle_scope;
    let enable = hotkeys_context.enable_scope;
    let disable = hotkeys_context.disable_scope; 

    use_hotkeys_scoped(
        "arrowup",
        Callback::new(move |_| {
            // move character up 
        }),
        vec!["game_scope"]
    );

    use_hotkeys_scoped(
        "arrowdown",
        Callback::new(move |_| {
            // move character down 
        }),
        vec!["game_scope"]
    );

    view! {
        <button
        // activates the 'game_scope' scope  
        on:click=move |_| enable("game_scope")  
        >
            Start game
        </button>

        <button
        // toggles the 'game_scope' from enabled to disabled 
        on:click=move |_| toggle("game_scope") 
        >
            Pause game
        </button>


        <button
            // disables the 'game_scope' scope 
            on:click=move |_| disable("game_scope")  
        >
            End game
        </button>
    }
}
```

### Focus trapped - `use_hotkeys_ref()` 
```rust
use leptos_hotkeys::use_hotkeys_ref;

#[component]
fn Component() -> impl IntoView {
    let node_ref = use_hotkeys_ref("l", Callback::new(move |_| {
        // some logic here 
    }));

    view! {
        <body>
            <div _ref=node_ref>
            // when this div is focused, the "l" hotkey will fire 
            </div>
        </body>
    }
}
```

## Contributions
Check the [issues](https://github.com/friendlymatthew/leptos-hotkeys/issues) page and feel free to post a PR!

## Bugs, Issues, Feature Requests
[Robert](https://github.com/JustBobinAround) and I created `leptos_hotkeys` with the intention of usability. If you encounter any bugs, issues, or feature requests, [please feel free to open an issue.](https://github.com/friendlymatthew/leptos-hotkeys/issues/new)

## CHANGELOG

### Next release: `v.0.2.0` 
[See milestones](https://github.com/friendlymatthew/leptos-hotkeys/milestone/1)

*February 12th, 2024*
- Recognize `meta` key (`v.0.1.3`)
- String cleaning


*February 8th, 2024*
- Elevate `leptos` to v.0.6.5
- Added `event.preventDefault()`