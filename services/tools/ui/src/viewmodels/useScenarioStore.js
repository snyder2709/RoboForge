/**
 * ViewModel: client-side scenario execution.
 * Keyframes play in the browser — sliders and 3D update reactively.
 */

import { ref } from 'vue'
import { SCENARIO_FRAMES, SCENARIO_OPTIONS } from '../models/Scenario'
import { useRobotStore } from './useRobotStore'

// Active scenario abort controllers per robot
const _abort = {}

const sleep = (ms) => new Promise((res) => setTimeout(res, ms))

export function useScenarioStore() {
  const { sendCmd, localServos } = useRobotStore()

  /** @type {import('vue').Ref<Record<number, boolean>>} */
  const running = ref({})

  /** @param {number} robotId @param {string} scenarioKey */
  async function execute(robotId, scenarioKey) {
    const frames = SCENARIO_FRAMES[scenarioKey]
    if (!frames) return

    // Cancel any running scenario for this robot
    if (_abort[robotId]) _abort[robotId].cancelled = true
    const ctrl = { cancelled: false }
    _abort[robotId] = ctrl

    running.value = { ...running.value, [robotId]: true }

    for (const frame of frames) {
      if (ctrl.cancelled) break

      // Update local sliders immediately — reactive, no need to wait for Zenoh round-trip
      localServos[robotId] = [...frame.servos]
      sendCmd(robotId, 'move_servos', frame.servos)

      await sleep(frame.duration)
    }

    if (!ctrl.cancelled) {
      running.value = { ...running.value, [robotId]: false }
    }
  }

  /** @param {number} robotId */
  function stop(robotId) {
    if (_abort[robotId]) _abort[robotId].cancelled = true
    running.value = { ...running.value, [robotId]: false }
  }

  return { scenarioOptions: SCENARIO_OPTIONS, running, execute, stop }
}
