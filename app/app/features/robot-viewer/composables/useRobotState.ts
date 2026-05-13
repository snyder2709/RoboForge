import { listen } from '@tauri-apps/api/event'

export interface RobotState {
  id: number
  ts: number
  joints: Record<string, number>
  imu: { roll: number; pitch: number; yaw: number }
}

export interface RobotDescriptor {
  id: number
  model: string
  joints: Array<{ index: number; name: string; min_deg: number; max_deg: number }>
}

const robots = ref<Map<number, RobotState>>(new Map())
const descriptors = ref<Map<number, RobotDescriptor>>(new Map())
const selectedId = ref<number | null>(null)

let listening = false

export function useRobotState() {
  let unlistenState: (() => void) | null = null
  let unlistenDescriptor: (() => void) | null = null

  onMounted(async () => {
    if (listening) return
    listening = true

    unlistenState = await listen<RobotState>('robot-state', (event) => {
      const state = event.payload
      robots.value = new Map(robots.value).set(state.id, state)
      // Автовыбор первого появившегося робота
      if (selectedId.value === null) selectedId.value = state.id
    })

    unlistenDescriptor = await listen<RobotDescriptor>('robot-descriptor', (event) => {
      const desc = event.payload
      descriptors.value = new Map(descriptors.value).set(desc.id, desc)
    })
  })

  onUnmounted(() => {
    unlistenState?.()
    unlistenDescriptor?.()
    listening = false
  })

  const robotList = computed(() => Array.from(robots.value.values()).sort((a, b) => a.id - b.id))
  const selectedRobot = computed(() => selectedId.value !== null ? robots.value.get(selectedId.value) ?? null : null)

  function selectRobot(id: number) {
    selectedId.value = id
  }

  return { robots, descriptors, robotList, selectedId, selectedRobot, selectRobot }
}
