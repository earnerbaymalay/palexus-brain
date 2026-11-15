import '@testing-library/jest-dom/vitest'
import { cleanup } from '@testing-library/react'
import { afterEach } from 'vitest'

// Testing Library only self-registers its cleanup when vitest runs with
// `globals: true`, which this project does not. Without it, every render in a
// file accumulates in the same document and queries match across tests.
afterEach(cleanup)

// Node 25+ defines its own localStorage/sessionStorage globals (undefined
// without --localstorage-file), and vitest does not overwrite existing globals
// with jsdom's. Point them back at jsdom's storage so tests run on any Node.
const dom = (globalThis as { jsdom?: { window: Window } }).jsdom
if (dom) {
  for (const key of ['localStorage', 'sessionStorage'] as const) {
    Object.defineProperty(globalThis, key, { configurable: true, value: dom.window[key] })
  }
}
