import { defineNuxtModule, addImports } from "@nuxt/kit"

type ImportEntry = { name: string; as: string; from: string }

const tauriImports: ImportEntry[] = [
    // Core
    { name: "invoke",                    as: "invoke",                    from: "@tauri-apps/api/core" },
    // Events
    { name: "listen",                    as: "listen",                    from: "@tauri-apps/api/event" },
    { name: "once",                      as: "once",                      from: "@tauri-apps/api/event" },
    // App info
    { name: "getName",                   as: "getName",                   from: "@tauri-apps/api/app" },
    { name: "getVersion",                as: "getVersion",                from: "@tauri-apps/api/app" },
    { name: "getTauriVersion",           as: "getTauriVersion",           from: "@tauri-apps/api/app" },
    // Webview
    { name: "WebviewWindow",             as: "WebviewWindow",             from: "@tauri-apps/api/webviewWindow" },
    { name: "getCurrentWebviewWindow",   as: "getCurrentWebviewWindow",   from: "@tauri-apps/api/webviewWindow" },
    { name: "getAllWebviews",            as: "getAllWebviews",            from: "@tauri-apps/api/webviewWindow" },
    { name: "getCurrentWebview",         as: "getCurrentWebview",         from: "@tauri-apps/api/webviewWindow" },
    // Shell — open: открывает URL/путь в системном приложении
    { name: "Command",                   as: "Command",                   from: "@tauri-apps/plugin-shell" },
    { name: "open",                      as: "openUrl",                   from: "@tauri-apps/plugin-shell" },
    // OS
    { name: "platform",                  as: "platform",                  from: "@tauri-apps/plugin-os" },
    { name: "arch",                      as: "arch",                      from: "@tauri-apps/plugin-os" },
    { name: "version",                   as: "osVersion",                 from: "@tauri-apps/plugin-os" },
    { name: "type",                      as: "osType",                    from: "@tauri-apps/plugin-os" },
    { name: "family",                    as: "osFamily",                  from: "@tauri-apps/plugin-os" },
    { name: "hostname",                  as: "hostname",                  from: "@tauri-apps/plugin-os" },
    { name: "locale",                    as: "osLocale",                  from: "@tauri-apps/plugin-os" },
    { name: "eol",                       as: "eol",                       from: "@tauri-apps/plugin-os" },
    { name: "exeExtension",             as: "exeExtension",              from: "@tauri-apps/plugin-os" },
    // Notifications
    { name: "sendNotification",          as: "sendNotification",          from: "@tauri-apps/plugin-notification" },
    { name: "requestPermission",         as: "requestPermission",         from: "@tauri-apps/plugin-notification" },
    { name: "isPermissionGranted",       as: "isPermissionGranted",       from: "@tauri-apps/plugin-notification" },
    // FS — open/watch aliased to avoid collision with Vue's watch and shell's open
    { name: "readFile",                  as: "readFile",                  from: "@tauri-apps/plugin-fs" },
    { name: "readTextFile",              as: "readTextFile",              from: "@tauri-apps/plugin-fs" },
    { name: "writeFile",                 as: "writeFile",                 from: "@tauri-apps/plugin-fs" },
    { name: "writeTextFile",             as: "writeTextFile",             from: "@tauri-apps/plugin-fs" },
    { name: "readDir",                   as: "readDir",                   from: "@tauri-apps/plugin-fs" },
    { name: "mkdir",                     as: "mkdir",                     from: "@tauri-apps/plugin-fs" },
    { name: "remove",                    as: "remove",                    from: "@tauri-apps/plugin-fs" },
    { name: "rename",                    as: "rename",                    from: "@tauri-apps/plugin-fs" },
    { name: "copyFile",                  as: "copyFile",                  from: "@tauri-apps/plugin-fs" },
    { name: "stat",                      as: "stat",                      from: "@tauri-apps/plugin-fs" },
    { name: "exists",                    as: "exists",                    from: "@tauri-apps/plugin-fs" },
    { name: "BaseDirectory",             as: "BaseDirectory",             from: "@tauri-apps/plugin-fs" },
    { name: "open",                      as: "openFile",                  from: "@tauri-apps/plugin-fs" },
    { name: "watch",                     as: "watchFs",                   from: "@tauri-apps/plugin-fs" },
    { name: "unwatch",                   as: "unwatchFs",                 from: "@tauri-apps/plugin-fs" },
    // Store
    { name: "Store",                     as: "Store",                     from: "@tauri-apps/plugin-store" },
    { name: "load",                      as: "load",                      from: "@tauri-apps/plugin-store" },
]

export default defineNuxtModule({
    meta: {
        name: "nuxt-tauri",
        configKey: "tauri",
    },
    setup() {
        addImports(tauriImports)
    },
})
