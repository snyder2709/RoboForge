/**
 * Client-side scenario definitions.
 * Each frame: { duration: ms, servos: number[20] }
 *
 * Servo index map:
 *  0: head_yaw   1: head_pitch
 *  2: l_shoulder_pitch  3: l_shoulder_roll  4: l_elbow  5: l_wrist
 *  6: r_shoulder_pitch  7: r_shoulder_roll  8: r_elbow  9: r_wrist
 * 10: l_hip_pitch  11: l_hip_roll  12: l_knee  13: l_ankle_pitch  14: l_ankle_roll
 * 15: r_hip_pitch  16: r_hip_roll  17: r_knee  18: r_ankle_pitch  19: r_ankle_roll
 */

const N = Array(20).fill(90) // neutral

/** @param {Record<number,number>} updates @returns {number[]} */
function f(updates) {
  const frame = [...N]
  for (const [i, v] of Object.entries(updates)) frame[+i] = v
  return frame
}

/** @type {Record<string, Array<{ duration: number, servos: number[] }>>} */
export const SCENARIO_FRAMES = {
  wave: [
    { duration: 400,  servos: f({ 6: 45, 7: 70, 8: 90 }) },   // поднять руку
    { duration: 200,  servos: f({ 6: 45, 7: 70, 8: 130 }) },  // мах — согнуть
    { duration: 200,  servos: f({ 6: 45, 7: 70, 8: 55 }) },   // мах — разогнуть
    { duration: 200,  servos: f({ 6: 45, 7: 70, 8: 130 }) },  // мах — согнуть
    { duration: 200,  servos: f({ 6: 45, 7: 70, 8: 55 }) },   // мах — разогнуть
    { duration: 400,  servos: [...N] },                         // нейтраль
  ],
  walk: [
    { duration: 350, servos: f({ 10: 65, 12: 115, 13: 110, 15: 115, 17: 65, 18: 70,  2: 115, 6: 65 }) },
    { duration: 350, servos: f({ 10: 115, 12: 65, 13: 70,  15: 65,  17: 115, 18: 110, 2: 65, 6: 115 }) },
    { duration: 350, servos: f({ 10: 65, 12: 115, 13: 110, 15: 115, 17: 65, 18: 70,  2: 115, 6: 65 }) },
    { duration: 350, servos: f({ 10: 115, 12: 65, 13: 70,  15: 65,  17: 115, 18: 110, 2: 65, 6: 115 }) },
    { duration: 350, servos: [...N] },
  ],
  jump: [
    { duration: 300, servos: f({ 10: 120, 12: 140, 13: 65,  15: 120, 17: 140, 18: 65 }) },  // присесть
    { duration: 150, servos: f({ 10: 60,  12: 60,  13: 115, 15: 60,  17: 60,  18: 115 }) }, // прыжок
    { duration: 250, servos: [...N] },                                                         // полёт
    { duration: 200, servos: f({ 10: 110, 12: 120, 13: 75,  15: 110, 17: 120, 18: 75 }) },  // приземление
    { duration: 300, servos: [...N] },                                                         // восстановление
  ],
}

export const SCENARIO_OPTIONS = [
  { value: 'wave', label: 'Помахать рукой' },
  { value: 'walk', label: 'Ходьба' },
  { value: 'jump', label: 'Прыжок' },
]
