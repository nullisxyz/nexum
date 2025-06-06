use leptos::prelude::*;
use stylers::style;

#[component]
pub fn SummonFrameButton() -> impl IntoView {
    let styler_class = style! { "SummonFrameButton",
        .summon-frame-button {
            width: 80px;
            height: 50px;
            display: flex;
            justify-content: center;
            align-items: center;
            box-sizing: border-box;
        }

        .summon-frame-button svg {
            height: 20px;
            transform: scaleX(-1);
        }
    };

    view! { class=styler_class,
        <div class="summon-frame-button">
            <svg viewBox="0 0 512 512">
                <path
                    fill="currentColor"
                    d="M416 32h-64c-17.67 0-32 14.33-32 32s14.33 32 32 32h64c17.67 0 32 14.33 32 32v256c0 17.67-14.33 32-32 32h-64c-17.67 0-32 14.33-32 32s14.33 32 32 32h64c53.02 0 96-42.98 96-96V128C512 74.98 469 32 416 32zM342.6 233.4l-128-128c-12.51-12.51-32.76-12.49-45.25 0c-12.5 12.5-12.5 32.75 0 45.25L242.8 224H32C14.31 224 0 238.3 0 256s14.31 32 32 32h210.8l-73.38 73.38c-12.5 12.5-12.5 32.75 0 45.25s32.75 12.5 45.25 0l128-128C355.1 266.1 355.1 245.9 342.6 233.4z"
                />
            </svg>
        </div>
    }
}
