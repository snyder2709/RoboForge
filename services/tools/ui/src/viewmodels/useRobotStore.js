/**
 * ViewModel: robot state store.
 * Receives Zenoh states via WS, exposes reactive state and commands.
 */

import { reactive, readonly, onUnmounted } from 'vue'
import { createRobotState } from '../models/RobotState'
import { useWsService } from '../services/wsService'

const ROBOT_IDS = [1, 2, 3, 4]

const _robots = reactive(
  Object.fromEntries(ROBOT_IDS.map((id) => [id, createRobotState(id)]))
)

// Slider values — always synced with live Zenoh state so they reflect any command source
const _localServos = reactive(
  Object.fromEntries(ROBOT_IDS.map((id) => [id, Array(20).fill(90)]))
)

// Wire WS listener once (module-level singleton)
const { onMessage, send } = useWsService()
onMessage((msg) => {
  if (msg.type !== 'state') return
  const r = _robots[msg.robot_id]
  if (!r) return
  r.servos = msg.servos
  r.imu    = msg.imu
  r.ts     = msg.ts
  r.live   = true
  // Always sync sliders with live state so scenarios/external commands are reflected
  _localServos[msg.robot_id] = [...msg.servos]
})

export function useRobotStore() {
  /** @param {number} robotId @param {string} action @param {number[]|null} servos */
  function sendCmd(robotId, action, servos = null) {
    const payload = { type: 'cmd', robot_id: robotId, action, ts: Date.now() / 1000 }
    if (servos) payload.servos = servos
    send(payload)
  }

  /** @param {number} robotId @param {number} servoIdx @param {number} angle */
  function setServo(robotId, servoIdx, angle) {
    _localServos[robotId][servoIdx] = angle
    sendCmd(robotId, 'move_servos', [..._localServos[robotId]])
  }

  return {
    robots:      readonly(_robots),
    localServos: _localServos,
    sendCmd,
    setServo,
  }
}
