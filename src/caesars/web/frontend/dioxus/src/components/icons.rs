use dioxus::prelude::*;

pub fn heart(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 36 36",
            path {
                fill: "#DD2E44",
                d: "M35.885 11.833c0-5.45-4.418-9.868-9.867-9.868-3.308 0-6.227 1.633-8.018 4.129-1.791-2.496-4.71-4.129-8.017-4.129-5.45 0-9.868 4.417-9.868 9.868 0 .772.098 1.52.266 2.241C1.751 22.587 11.216 31.568 18 34.034c6.783-2.466 16.249-11.447 17.617-19.959.17-.721.268-1.469.268-2.242z"
            }
            path {
                fill: "#FDCB58",
                d: "M34.347 23.894l-3.824-1.416-1.416-3.824c-.145-.394-.52-.654-.938-.654-.418 0-.793.26-.938.653l-1.416 3.824-3.824 1.416c-.393.144-.653.519-.653.938 0 .418.261.793.653.938l3.824 1.416 1.416 3.824c.145.393.52.653.938.653.418 0 .793-.261.938-.653l1.416-3.824 3.824-1.416c.392-.145.653-.52.653-.938 0-.418-.261-.793-.653-.937zm-23-16.001l-2.365-.875-.875-2.365C7.961 4.26 7.587 4 7.169 4c-.419 0-.793.26-.938.653l-.876 2.365-2.364.875c-.393.146-.653.52-.653.938 0 .418.26.792.653.938l2.365.875.875 2.365c.146.393.52.653.938.653.418 0 .792-.26.938-.653l.875-2.365 2.365-.875c.393-.146.653-.519.653-.938 0-.418-.26-.792-.653-.938z"
            }
        }
    ))
}

pub fn padlock(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
    view_box: "0 0 128 128",
    xmlns: "http://www.w3.org/2000/svg",
    style: "enable-background:new 0 0 128 128",
    path {
        d: "M85.86 62.7h-.01c-.12-2.45-2.72-4.74-7.21-6.09-9.61-2.89-20.27-4.58-32.43-4.68-12.15.1-22.81 1.79-32.43 4.68-4.49 1.35-7.07 3.64-7.2 6.09h-.01v50.08c0 2.79 3.02 5.8 10.26 7.85 7.48 2.12 17.6 3.49 29.38 3.49s21.9-1.37 29.38-3.49c7.59-2.15 10.39-5.32 10.27-8.25V62.7z",
        style: "fill:#e2a610",
    }
    linearGradient {
        id: "a",
        y1: "84.429",
        y2: "84.429",
        x2: "78.997",
        gradientUnits: "userSpaceOnUse",
        x1: "89.177",
        stop {
            offset: "0",
            style: "stop-color:#9e740b",
        }
        stop {
            offset: ".306",
            style: "stop-color:#9e740b;stop-opacity:.963",
        }
    }
    path {
        style: "fill:url(#a)",
        d: "M85.85 62.7c-.12-2.45-2.72-4.74-7.21-6.09-9.61-2.89 11.29 9.47-.86 9.37 0 0-.32 25.13 0 38.43.21 8.82 8.12 8.72 8.08 7.96V62.7h-.01z",
    }
    path {
        style: "fill:#e2a610",
        d: "M6.57 63.43h79.29",
    }
    path {
        d: "m54.11 108.24-4.49-10.58a7.662 7.662 0 0 0 4.33-6.9c0-4.24-3.44-7.67-7.67-7.67s-7.67 3.44-7.67 7.67c0 2.85 1.56 5.34 3.87 6.66l-4.18 10.89c-.53 1.38.49 2.85 1.96 2.85h11.92c1.5 0 2.52-1.54 1.93-2.92z",
        style: "fill:#4e342e",
    }
    path {
        style: "fill:#9e740b",
        d: "M53.69 107.26H38.7l-.4 1.05c-.53 1.38.49 2.85 1.96 2.85h11.92c1.51 0 2.52-1.54 1.93-2.92l-.42-.98zM43.16 95.4c.18-.52-.01-1.08-.45-1.4-.6-.44-1.41-1.22-1.92-2.46-1.86-4.48.35-6.47.35-6.47a7.622 7.622 0 0 0-2.54 5.69c0 2.85 1.56 5.34 3.87 6.66l.69-2.02zM51.67 85.3s2.2 4.42-.32 7.25c-.86.96-1.59 1.47-2.13 1.72-.53.25-.77.88-.52 1.41l.93 1.96a7.662 7.662 0 0 0 4.33-6.9c-.01-3.25-2.29-5.44-2.29-5.44z",
    }
    path {
        d: "M76.96 55.95c-9.12-2.64-19.22-4.19-30.74-4.28-11.52.09-21.63 1.64-30.74 4.28-9.13 2.65-10.04 9.26 2.89 12.8 7.09 1.94 16.69 3.19 27.85 3.19s20.76-1.25 27.85-3.19c12.93-3.54 12.02-10.16 2.89-12.8z",
        style: "fill:#fdd835",
    }
    g {
        path {
            style: "fill:#84b0c1;stroke:#84b0c1;stroke-width:1.9584;stroke-miterlimit:10",
            d: "M74.37 62.21s-.37 1.72-5.22 1.72-5.88-1.72-5.88-1.72V40.16C63.27 30.14 55.62 22 46.21 22s-17.06 8.15-17.06 18.16v22.06s-2.11 1.72-5.95 1.72-5.14-1.72-5.14-1.72V40.16c0-16.14 12.63-29.26 28.15-29.26s28.15 13.13 28.15 29.26v22.05z",
        }
        path {
            style: "fill:#b9e4ea",
            d: "M33.52 21.56c-5.14 3.06-5.93 5.13-7.36 6.08-1.06.7-2.47.14-1.55-2.29.79-2.08 2.69-6.02 7.43-9.05 8.16-5.22 15.2-4.98 14.82-.99-.32 3.53-7.85 2.99-13.34 6.25z",
        }
    }
    path {
        style: "fill:#e2a610",
        d: "M76.96 55.95c-1.31-.38-6.24 7.12-4.38 8.53 2.14 1.62 4.66 3.19 5.2 2.98 11.27-4.34 8.31-8.87-.82-11.51z",
    }
    radialGradient {
        id: "b",
        gradientTransform: "matrix(-.077 -.997 .4971 -.0384 58.224 127.429)",
        cy: "49.066",
        cx: "76.707",
        r: "37.716",
        gradientUnits: "userSpaceOnUse",
        stop {
            offset: ".156",
            style: "stop-color:#3d8192",
        }
        stop {
            style: "stop-color:#3d8192",
            offset: ".277",
        }
    }
    path {
        style: "fill:url(#b)",
        d: "M72.65 27.68c-4.17 2.78-7.78 7.91-7.48 19.35.31 11.82 6.48 16.58 7.45 17.49 1.97-.63 3.03-1.94 3.03-1.94l-.4-22.53c.01-3-.12-7.82-2.6-12.37z",
    }
    g {
        path {
            d: "M93.76 20.22c-13.11.38-22.04 12.32-21.92 25.44.1 10.38 6.57 16.83 15.7 20.43l-.11 39.41c0 1.65.68 3.22 1.88 4.34l2.91 2.79 2.36 3.3c.68.23 3.61.16 4.18-.18l4.42-3.62c1.2-1.13 1.88-2.7 1.88-4.34v-2.89c0-1.8-.76-3.59-2.24-4.62-.92-.64-1.49-1.52-1.49-2.5 0-1.16.79-2.18 2.02-2.82 1.02-.54 1.72-1.54 1.72-2.69v-.39c.04-1.54-.75-2.49-1.63-3.07-1.37-.9-1.9-2.24-2.11-3.69-.18-1.26.54-2.59 1.42-3.49 1.58-1.63 2.28-3.02 2.28-5.3l.01-7.58c9.56-3.44 16.4-12.59 16.4-23.34-.01-14.12-11.49-25.65-27.68-25.19zm9.6 19.66c0 3.72-3.01 6.73-6.73 6.73-3.72 0-6.73-3.01-6.73-6.73 0-3.72 3.01-6.73 6.73-6.73 3.72 0 6.73 3.01 6.73 6.73z",
            style: "fill:#9e740b",
        }
        path {
            d: "M92.94 20.22c-13.7 0-24.8 11.1-24.8 24.8 0 10.48 6.51 19.44 15.7 23.07l-.11 37.01c0 1.65.68 3.22 1.88 4.34l5.61 5.2c1.56-.43 3.36 1.3 3.36 1.3l4.9-4.21c1.2-1.13 1.88-2.7 1.88-4.34v-2.89c0-1.8-.76-3.59-2.24-4.62-.92-.64-1.49-1.52-1.49-2.5 0-1.16.79-2.18 2.02-2.82 1.02-.54 1.72-1.54 1.72-2.69v-.39c.04-1.54-.75-2.49-1.63-3.07-1.37-.9-1.9-2.24-2.11-3.69-.18-1.26.54-2.59 1.42-3.49 1.58-1.63 2.28-3.02 2.28-5.3l.01-7.58c9.56-3.44 16.4-12.59 16.4-23.34-.01-13.69-11.11-24.79-24.8-24.79zm6.73 19.25c0 3.72-3.01 6.73-6.73 6.73-3.72 0-6.73-3.01-6.73-6.73 0-3.72 3.01-6.73 6.73-6.73 3.71 0 6.73 3.01 6.73 6.73z",
            style: "fill:#ffca28",
        }
        path {
            style: "fill:#dba010",
            d: "M91.22 114.64s.95-.18 3.36 1.3l-.36-44.07c-.07-1.5-2.86 2.11-2.87 4.02l-.13 38.75z",
        }
        path {
            d: "M74.22 39.47c1.08-9.13 8.5-12.71 11.45-13.65.99-.31 3-.83 3.46.89.41 1.52-.53 2.33-2.14 2.84-3.93 1.27-9.27 4.05-10.2 10.79-.18 1.29-.88 2.04-1.72 1.81-.57-.16-1.05-1.05-.85-2.68z",
            style: "fill:#fff59d",
        }
    }
}
    ))
}