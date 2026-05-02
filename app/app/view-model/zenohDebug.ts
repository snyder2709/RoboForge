import { listen } from '@tauri-apps/api/event'

export type TestType = 'single' | 'swarm'
export type ScenarioName = 'wave' | 'walk' | 'jump'

export const SCENARIO_OPTIONS: { value: ScenarioName; label: string }[] = [
    { value: 'wave', label: 'Помахать рукой' },
    { value: 'walk', label: 'Ходьба' },
    { value: 'jump', label: 'Прыжок' },
]

export function useZenohDebugViewModel() {
    const endpoint = useState('zenoh.endpoint', () => 'tcp/127.0.0.1:7447')
    const robotId = useState<1 | 2 | 3 | 4>('zenoh.robotId', () => 1)
    const durationSecs = useState<5 | 10 | 30>('zenoh.duration', () => 5)
    const testType = useState<TestType>('zenoh.testType', () => 'single')
    const scenario = useState<ScenarioName>('zenoh.scenario', () => 'wave')
    const robotCount = useState<1 | 2 | 3 | 4>('zenoh.robotCount', () => 4)
    const isRunning = ref(false)

    const zenoh = useZenoh()

    if (import.meta.client) {
        listen<{ success: boolean; count: number }>('zenoh-test-done', () => {
            isRunning.value = false
        }).catch(console.error)
    }

    async function handleConnect() {
        try {
            const state = zenoh.connectionState.value
            if (state === 'connected' || state === 'testing') {
                await zenoh.disconnect()
            }
            else {
                await zenoh.connect(endpoint.value)
            }
        }
        catch (e) {
            console.error('Zenoh connect error:', e)
        }
    }

    async function handleRunTest() {
        if (isRunning.value) return
        isRunning.value = true
        try {
            await zenoh.runTest(robotId.value, durationSecs.value)
        }
        catch (e) {
            isRunning.value = false
            console.error('Zenoh test error:', e)
        }
    }

    async function handleRunSwarmTest() {
        if (isRunning.value) return
        isRunning.value = true
        try {
            await invoke('zenoh_run_swarm_test', {
                robotCount: robotCount.value,
                scenario: scenario.value,
            })
        }
        catch (e) {
            isRunning.value = false
            console.error('Zenoh swarm test error:', e)
        }
    }

    return {
        endpoint,
        robotId,
        durationSecs,
        testType,
        scenario,
        robotCount,
        isRunning,
        connectionState: zenoh.connectionState,
        logs: zenoh.logs,
        handleConnect,
        handleRunTest,
        handleRunSwarmTest,
        clearLogs: zenoh.clearLogs,
    }
}
