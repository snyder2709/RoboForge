/** @typedef {{ roll: number, pitch: number, yaw: number }} Imu */

/**
 * @typedef {Object} RobotState
 * @property {number}   id
 * @property {number[]} servos  20 servo angles 0-180
 * @property {Imu}      imu
 * @property {number|null} ts  unix timestamp
 * @property {boolean}  live   true when Zenoh data received
 */

/** @param {number} id @returns {RobotState} */
export function createRobotState(id) {
  return {
    id,
    servos: Array(20).fill(90),
    imu:    { roll: 0, pitch: 0, yaw: 0 },
    ts:     null,
    live:   false,
  }
}
