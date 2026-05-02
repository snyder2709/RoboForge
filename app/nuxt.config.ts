/// <reference types="node" />
import { resolve } from "node:path"
import { pathToFileURL } from "node:url"
import { useNitro } from "@nuxt/kit"

const tauriDevHost = process.env.TAURI_DEV_HOST

// Workaround for Nuxt 4.4.4 bug: ssr:false + vite-node IPC socket
// https://github.com/nuxt/nuxt/issues/34957
function spaDevManifestWorkaround(_options: Record<string, unknown>, nuxt: Parameters<typeof useNitro>[0] extends never ? never : any) {
    nuxt.hook("vite:extendConfig", (_config: any, context: { isClient: boolean }) => {
        if (!nuxt.options.dev || nuxt.options.ssr || !context.isClient) return

        const nitro = useNitro()
        const clientManifestPath = pathToFileURL(
            resolve(nuxt.options.buildDir, "dist/server/client.manifest.mjs"),
        ).href

        nitro.options.virtual ||= {}
        nitro.options._config.virtual ||= {}

        for (const virtual of [nitro.options.virtual, nitro.options._config.virtual]) {
            virtual["#build/dist/server/server.mjs"] = "export default () => {}"
            virtual["#build/dist/server/client.manifest.mjs"] =
                `export { default } from ${JSON.stringify(clientManifestPath)}`
        }
    })
}

export default defineNuxtConfig({
    compatibilityDate: "2026-01-01",

    ssr: false,

    devtools: { enabled: false },

    modules: [
        "@nuxt/ui",
        "@nuxt/eslint",
        "~/modules/tauri",
        spaDevManifestWorkaround,
    ],

    components: [
        { path: "~/features", pathPrefix: false },
        "~/components",
    ],

    css: ["~/assets/css/main.css"],

    devServer: {
        port: 3001,
        host: tauriDevHost ?? "localhost",
    },

    vite: {
        clearScreen: false,
        envPrefix: ["VITE_", "TAURI_"],
        server: {
            strictPort: true,
            hmr: tauriDevHost
                ? {
                    protocol: "ws",
                    host: tauriDevHost,
                    port: 3001,
                }
                : undefined,
            watch: {
                ignored: ["**/src-tauri/**"],
            },
        },
    },

    experimental: {
        typedPages: true,
    },

    imports: {
        dirs: ["features/*/composables", "view-model"],
        presets: [
            {
                from: "zod",
                imports: ["z"],
            },
        ],
    },
})
