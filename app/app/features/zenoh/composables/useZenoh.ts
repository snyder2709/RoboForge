import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'

export type ZenohFsmState = 'disconnected' | 'connecting' | 'connected' | 'testing' | 'error'

export interface ZenohLogEntry {
    level: string
    message: string
    ts: number
}

let _initialized = false
const _state = ref<ZenohFsmState>('disconnected')
const _logs = ref<ZenohLogEntry[]>([])

export function useZenoh() {
    if (!_initialized && import.meta.client) {
        _initialized = true
        listen<{ state: ZenohFsmState; error?: string }>('zenoh-state', (e) => {
            _state.value = e.payload.state
        }).catch(console.error)
        listen<ZenohLogEntry>('zenoh-log', (e) => {
            _logs.value.push(e.payload)
        }).catch(console.error)
    }

    return {
        connectionState: readonly(_state),
        logs: readonly(_logs),
        connect: (endpoint: string) =>
            invoke('zenoh_connect', { endpoint }),
        disconnect: () =>
            invoke('zenoh_disconnect'),
        getState: () =>
            invoke<ZenohFsmState>('zenoh_get_state'),
        runTest: (robotId: number, durationSecs: number) =>
            invoke('zenoh_run_test', { robotId, durationSecs }),
        clearLogs: () => {
            _logs.value = []
        },
    }
}
