import { writable } from "svelte/store"
import type { Writable } from "svelte/store"

export const difficulty: Writable<string> = writable("easy")
export const numberOfQuestions: Writable<string> = writable("10")
