import { writable } from "svelte/store"
import type { Writable } from "svelte/store"

import { languageTag } from "$paraglide/runtime"

const defaultLocale = languageTag()
export const currentLocale: Writable<"en" | "id"> = writable(defaultLocale)
