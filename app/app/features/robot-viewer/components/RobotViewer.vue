<script setup lang="ts">
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import URDFLoader from 'urdf-loader'
import type { RobotState, RobotDescriptor } from '../composables/useRobotState'
import { useUrdfStore } from '../composables/useUrdfStore'

const props = defineProps<{
  mode?: 'solo' | 'swarm'
  robots?: RobotState[]
  descriptors?: Map<number, RobotDescriptor>
  selectedId?: number | null
}>()

const mountRef = ref<HTMLDivElement | null>(null)
const pendingUrdfModels = ref<Set<string>>(new Set())

// Лейблы роботов — позиции в экранных координатах
type RobotLabel = { id: number; x: number; y: number; name: string; selected: boolean }
const labels = ref<RobotLabel[]>([])

const { getUrdf, importUrdf, urdfMap } = useUrdfStore()

async function requestImport(urdfModel: string) {
  const ok = await importUrdf(urdfModel)
  if (ok) pendingUrdfModels.value = new Set([...pendingUrdfModels.value].filter(m => m !== urdfModel))
}

onMounted(() => {
  const mount = mountRef.value!
  const w = mount.clientWidth, h = mount.clientHeight

  const scene = new THREE.Scene()
  scene.fog = new THREE.Fog(0x0F0E0D, 8, 28)
  const camera = new THREE.PerspectiveCamera(35, w / h, 0.1, 100)

  const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true, powerPreference: 'high-performance' })
  renderer.setPixelRatio(Math.min(devicePixelRatio, 2))
  renderer.setSize(w, h)
  renderer.setClearColor(0x000000, 0)
  renderer.shadowMap.enabled = true
  renderer.shadowMap.type = THREE.PCFSoftShadowMap
  mount.appendChild(renderer.domElement)

  // Lights — 3-point setup for product viewer
  // Ambient: soft warm fill, не даёт полностью чёрных теней
  scene.add(new THREE.AmbientLight(0xfff0e8, 0.28))

  // Key light: основной источник, сверху-справа-спереди
  const keyLight = new THREE.DirectionalLight(0xffe8d6, 1.4)
  keyLight.position.set(5, 10, 6)
  keyLight.castShadow = true
  keyLight.shadow.mapSize.set(2048, 2048)
  keyLight.shadow.camera.left   = -6
  keyLight.shadow.camera.right  =  6
  keyLight.shadow.camera.top    =  8
  keyLight.shadow.camera.bottom = -2
  keyLight.shadow.bias = -0.0005
  scene.add(keyLight)

  // Fill light: слева-спереди, мягкий нейтральный — убирает жёсткие тени
  const fillLight = new THREE.DirectionalLight(0xdde8f5, 0.45)
  fillLight.position.set(-6, 4, 5)
  scene.add(fillLight)

  // Rim light: сзади-сверху, холодный синий — даёт силуэт
  const rimLight = new THREE.DirectionalLight(0x7ab0e8, 0.55)
  rimLight.position.set(-2, 6, -8)
  scene.add(rimLight)

  // Ground bounce: слабый тёплый снизу, имитирует отражение от пола
  const bounceLight = new THREE.HemisphereLight(0x0F0E0D, 0x1a1714, 0.3)
  scene.add(bounceLight)

  // Accent: coral точечный свет далеко в стороне — брендовый оттенок на краях,
  // НЕ в центре сцены чтобы не давать пятен
  const accentLight = new THREE.PointLight(0xE8784A, 0.6, 20, 1.5)
  accentLight.position.set(4, 3, -6)
  scene.add(accentLight)

  // Floor + grid
  const floor = new THREE.Mesh(new THREE.PlaneGeometry(60, 60), new THREE.ShadowMaterial({ opacity: 0.5 }))
  floor.rotation.x = -Math.PI / 2; floor.receiveShadow = true; scene.add(floor)

  const gridGroup = new THREE.Group()
  const gm = new THREE.LineBasicMaterial({ color: 0x3A6FAF, transparent: true, opacity: 0.2 })
  const am = new THREE.LineBasicMaterial({ color: 0xE8784A, transparent: true, opacity: 0.18 })
  for (let x = -8; x <= 8; x++)
    gridGroup.add(new THREE.Line(new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(x, 0.001, 4), new THREE.Vector3(x, 0.001, -16)]), x === 0 ? am : gm))
  for (let z = -16; z <= 4; z++)
    gridGroup.add(new THREE.Line(new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(-8, 0.001, z), new THREE.Vector3(8, 0.001, z)]), gm))
  gridGroup.visible = false; scene.add(gridGroup)

  // Halo + pool
  function makeHalo(inner: string, outer: string, size: number) {
    const c = document.createElement('canvas'); c.width = c.height = 512
    const ctx = c.getContext('2d')!
    const g = ctx.createRadialGradient(256, 256, 30, 256, 256, 240)
    g.addColorStop(0, inner); g.addColorStop(0.35, outer); g.addColorStop(1, 'rgba(0,0,0,0)')
    ctx.fillStyle = g; ctx.fillRect(0, 0, 512, 512)
    const sp = new THREE.Sprite(new THREE.SpriteMaterial({ map: new THREE.CanvasTexture(c), blending: THREE.AdditiveBlending, depthWrite: false, transparent: true }))
    sp.scale.setScalar(size); return sp
  }
  const halo = makeHalo('rgba(232,120,74,0.85)', 'rgba(232,120,74,0.12)', 8)
  halo.position.set(0, 0.3, -1.5); scene.add(halo)

  const pc = document.createElement('canvas'); pc.width = pc.height = 512
  const pctx = pc.getContext('2d')!
  const pg = pctx.createRadialGradient(256, 256, 0, 256, 256, 220)
  pg.addColorStop(0, 'rgba(232,120,74,0.55)'); pg.addColorStop(0.6, 'rgba(232,120,74,0.12)'); pg.addColorStop(1, 'rgba(0,0,0,0)')
  pctx.fillStyle = pg; pctx.fillRect(0, 0, 512, 512)
  const pool = new THREE.Mesh(new THREE.PlaneGeometry(8, 8), new THREE.MeshBasicMaterial({ map: new THREE.CanvasTexture(pc), transparent: true, blending: THREE.AdditiveBlending, depthWrite: false }))
  pool.rotation.x = -Math.PI / 2; pool.position.y = 0.002; scene.add(pool)

  // ── URDF loader ────────────────────────────────────────────────────────────

  const bodyMat  = new THREE.MeshStandardMaterial({ color: 0xEAE7E1, roughness: 0.55, metalness: 0.05 })
  const jointMat = new THREE.MeshStandardMaterial({ color: 0x3A6FAF, roughness: 0.35, metalness: 0.4 })
  const darkMat  = new THREE.MeshStandardMaterial({ color: 0x2A2724, roughness: 0.7 })

  const loader = new URDFLoader()
  loader.packages = ''
  loader.loadMeshCb = (_p, _m, done) => { done(new THREE.Object3D(), null) }

  const templateCache = new Map<string, URDFRobot>()

  type URDFRobot = THREE.Object3D & {
    joints: Record<string, { setJointValue(v: number): void }>
  }

  function parseUrdf(xml: string, urdfModel: string): URDFRobot {
    const robot = loader.parse(xml) as URDFRobot
    robot.traverse(obj => {
      if (!(obj as THREE.Mesh).isMesh) return
      const mesh = obj as THREE.Mesh
      const name = (obj.parent?.name ?? obj.name).toLowerCase()
      if (name.includes('camera') || name.includes('dark')) mesh.material = darkMat.clone()
      else if (name.includes('silver') || name.includes('yaw_link')) mesh.material = jointMat.clone()
      else mesh.material = bodyMat.clone()
      mesh.castShadow = true; mesh.receiveShadow = true
    })
    templateCache.set(urdfModel, robot)
    return robot
  }

  function cloneUrdf(template: URDFRobot): URDFRobot {
    const clone = template.clone(true) as URDFRobot
    const joints: Record<string, { setJointValue(v: number): void }> = {}
    clone.traverse((obj: THREE.Object3D) => {
      const j = obj as any
      if (j.isURDFJoint && j.jointType !== 'fixed' && j.name) joints[j.name] = j
    })
    clone.joints = joints
    return clone
  }

  // ── Robot slots ────────────────────────────────────────────────────────────

  const URDF_SCALE = 10
  // Точная цепочка до нижней точки (bottom of left_foot):
  // base_link → left_hip_yaw z=-0.045 → left_hip_pitch z=-0.010
  //           → left_knee_pitch z=-0.060 → left_ankle_pitch z=-0.060
  //           → foot visual origin z=-0.005, half foot height=0.005
  // Итого: -(0.045+0.010+0.060+0.060+0.005+0.005) = -0.185м
  const ROBOT_FLOOR_OFFSET = 0.185 * URDF_SCALE
  // Верх: head joint z=+0.060 + head box half-height 0.040/2 = +0.080м
  const ROBOT_TOP_OFFSET   = 0.080 * URDF_SCALE
  // Лейбл рисуется выше макушки
  const LABEL_HEIGHT = (0.185 + 0.080 + 0.06) * URDF_SCALE

  // Все роботы одного масштаба — перспектива создаётся камерой и z-позицией
  const swarmPos = [
    { x: 0,    z: 0,    s: 1.0 },
    { x: -2.4, z: -3.0, s: 1.0 },
    { x:  2.4, z: -3.0, s: 1.0 },
    { x:  0,   z: -5.5, s: 1.0 },
  ]

  // slot → { root, robotId }
  const activeRobots = new Map<number, { mesh: URDFRobot; robotId: number }>()

  const placeholders = swarmPos.map((p, i) => {
    const g = new THREE.Group()
    g.position.set(p.x, 0, p.z)
    g.scale.setScalar(p.s)
    g.visible = false  // скрыты по умолчанию — показываем только когда есть реальный робот
    const ph = new THREE.Mesh(
      new THREE.BoxGeometry(0.06 * URDF_SCALE, 0.08 * URDF_SCALE, 0.04 * URDF_SCALE),
      new THREE.MeshStandardMaterial({ color: 0x3A6FAF, wireframe: true, opacity: 0.25, transparent: true })
    )
    ph.position.y = 0.04 * URDF_SCALE
    g.add(ph)
    scene.add(g)
    return g
  })

  // Вспомогательный вектор для проекции 3D → 2D (лейблы)
  const _v3 = new THREE.Vector3()

  function project3D(worldX: number, worldY: number, worldZ: number, w: number, h: number) {
    _v3.set(worldX, worldY, worldZ).project(camera)
    return {
      x: (_v3.x * 0.5 + 0.5) * w,
      y: (-_v3.y * 0.5 + 0.5) * h,
    }
  }

  function mountRobotInSlot(slotIdx: number, robotId: number, xml: string, urdfModel: string) {
    let template = templateCache.get(urdfModel)
    if (!template) template = parseUrdf(xml, urdfModel)

    const robot = cloneUrdf(template)
    const pos = swarmPos[slotIdx]
    robot.rotation.x = -Math.PI / 2   // URDF Z-up → Three.js Y-up
    robot.scale.setScalar(pos.s * URDF_SCALE)
    // Поднимаем так чтобы ступни оказались на y=0
    robot.position.set(pos.x, ROBOT_FLOOR_OFFSET * pos.s, pos.z)
    robot.visible = true

    placeholders[slotIdx].clear()
    scene.remove(placeholders[slotIdx])

    scene.add(robot)
    activeRobots.set(slotIdx, { mesh: robot, robotId })
  }

  function applyJoints(slotIdx: number, joints: Record<string, number>) {
    const entry = activeRobots.get(slotIdx)
    if (!entry) return
    for (const [name, rad] of Object.entries(joints)) {
      entry.mesh.joints[name]?.setJointValue(rad)
    }
  }

  // Обновить видимость: показывать только слоты с реальными роботами
  function syncVisibility() {
    const count = props.robots?.length ?? 0
    const isSwarm = props.mode === 'swarm'

    activeRobots.forEach(({ mesh }, slotIdx) => {
      mesh.visible = isSwarm ? slotIdx < count : slotIdx === 0
    })
    placeholders.forEach((ph, slotIdx) => {
      // placeholder видим только если слот занят роботом которого ещё нет в activeRobots
      ph.visible = false
    })
  }

  // Обновить лейблы поверх canvas
  function syncLabels(cw: number, ch: number) {
    const count = props.robots?.length ?? 0
    const isSwarm = props.mode === 'swarm'
    const newLabels: RobotLabel[] = []

    activeRobots.forEach(({ mesh, robotId }, slotIdx) => {
      const visible = isSwarm ? slotIdx < count : slotIdx === 0
      if (!visible) return

      const desc = props.descriptors?.get(robotId)
      const pos = swarmPos[slotIdx]
      const screen = project3D(pos.x, LABEL_HEIGHT * pos.s, pos.z, cw, ch)

      newLabels.push({
        id: robotId,
        x: screen.x,
        y: screen.y,
        name: desc?.model ?? `Robot #${robotId}`,
        selected: robotId === props.selectedId,
      })
    })

    labels.value = newLabels
  }

  // Main watch — роботы + urdfMap
  watch([() => props.robots, urdfMap], ([states]) => {
    if (!states) return
    for (let i = 0; i < Math.min(states.length, swarmPos.length); i++) {
      const state = states[i]
      const desc = props.descriptors?.get(state.id)
      const urdfModel = desc?.urdf_model || 'roboforge_humanoid_v1'

      if (!activeRobots.has(i)) {
        const xml = getUrdf(urdfModel)
        if (xml) {
          mountRobotInSlot(i, state.id, xml, urdfModel)
        } else {
          // Показываем placeholder и просим импортировать
          if (i < placeholders.length) placeholders[i].visible = true
          pendingUrdfModels.value = new Set([...pendingUrdfModels.value, urdfModel])
        }
      }
      applyJoints(i, state.joints ?? {})
    }
    syncVisibility()
  }, { deep: true })

  // ── Camera & controls ──────────────────────────────────────────────────────

  const controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.08
  controls.minDistance = 1
  controls.maxDistance = 22
  // Вертикальный диапазон: от почти горизонтального взгляда (5°) до взгляда сверху-вниз (85°)
  // Не пускаем под платформу и не пускаем смотреть прямо вертикально вниз
  controls.minPolarAngle = THREE.MathUtils.degToRad(5)
  controls.maxPolarAngle = THREE.MathUtils.degToRad(85)
  controls.screenSpacePanning = false

  let targetPos = new THREE.Vector3()
  let targetLook = new THREE.Vector3()
  let lerpingCamera = false

  // ── Сохранение/восстановление позиции камеры ───────────────────────────────

  type CamSnapshot = { px: number; py: number; pz: number; tx: number; ty: number; tz: number }

  function camKey(mode: string) { return `rf_cam_${mode}` }

  function saveCam(mode: string) {
    const snap: CamSnapshot = {
      px: camera.position.x, py: camera.position.y, pz: camera.position.z,
      tx: controls.target.x, ty: controls.target.y, tz: controls.target.z,
    }
    localStorage.setItem(camKey(mode), JSON.stringify(snap))
  }

  function loadCam(mode: string): CamSnapshot | null {
    try { return JSON.parse(localStorage.getItem(camKey(mode)) ?? '') }
    catch { return null }
  }

  // Сохраняем когда пользователь отпустил управление
  controls.addEventListener('end', () => {
    saveCam(props.mode ?? 'swarm')
  })

  // Любое взаимодействие пользователя отменяет автоматический перелёт камеры
  controls.addEventListener('start', () => { lerpingCamera = false })

  function applyMode(m: string, immediate = false) {
    const isSwarm = m === 'swarm'
    // Дефолтная позиция — используется только если нет сохранённой
    const defPos = isSwarm ? [0, 4.2, 7.5] : [0, 0.8, 2.5]
    const defLook = isSwarm ? [0, 0.5, -2] : [0, 0.5, 0]

    const saved = loadCam(m)
    if (saved) {
      targetPos.set(saved.px, saved.py, saved.pz)
      targetLook.set(saved.tx, saved.ty, saved.tz)
    } else {
      targetPos.set(...defPos as [number, number, number])
      targetLook.set(...defLook as [number, number, number])
    }

    camera.fov = isSwarm ? 50 : 40; camera.updateProjectionMatrix()
    gridGroup.visible = isSwarm
    halo.scale.setScalar(isSwarm ? 13 : 6)
    halo.position.set(0, 0.3, isSwarm ? -3 : -0.5)
    pool.scale.set(isSwarm ? 1.8 : 1, isSwarm ? 1.8 : 1, 1)
    pool.position.set(0, 0.002, isSwarm ? -2 : 0)
    syncVisibility()
    if (immediate) {
      camera.position.copy(targetPos); controls.target.copy(targetLook); controls.update()
    } else {
      lerpingCamera = true
    }
  }

  applyMode(props.mode ?? 'swarm', true)
  watch(() => props.mode, m => { applyMode(m ?? 'swarm', false) })
  watch(() => props.selectedId, () => { syncVisibility() })

  // ── Animation loop ─────────────────────────────────────────────────────────

  let animFrame = 0
  const animate = () => {

    if (lerpingCamera) {
      camera.position.lerp(targetPos, 0.06)
      controls.target.lerp(targetLook, 0.06)
      if (camera.position.distanceTo(targetPos) < 0.01) {
        camera.position.copy(targetPos); controls.target.copy(targetLook); lerpingCamera = false
      }
    }
    controls.update()

    // Обновляем лейблы каждый кадр (камера могла повернуться)
    syncLabels(mount.clientWidth, mount.clientHeight)

    renderer.render(scene, camera)
    animFrame = requestAnimationFrame(animate)
  }
  animate()

  const ro = new ResizeObserver(() => {
    const w2 = mount.clientWidth, h2 = mount.clientHeight
    camera.aspect = w2 / h2; camera.updateProjectionMatrix()
    renderer.setSize(w2, h2)
  })
  ro.observe(mount)

  onUnmounted(() => {
    cancelAnimationFrame(animFrame); ro.disconnect(); controls.dispose(); renderer.dispose()
    if (mount.contains(renderer.domElement)) mount.removeChild(renderer.domElement)
  })
})
</script>

<template>
  <div ref="mountRef" class="viewer-mount">

    <!-- Robot labels — CSS overlay, позиция считается из 3D каждый кадр -->
    <div
      v-for="label in labels"
      :key="label.id"
      class="robot-label"
      :class="{ 'robot-label--selected': label.selected }"
      :style="{ left: label.x + 'px', top: label.y + 'px' }"
    >
      <span class="robot-label__dot" />
      <span class="robot-label__text">{{ label.name }}</span>
      <span v-if="label.selected" class="robot-label__badge">selected</span>
    </div>

    <!-- URDF import prompt -->
    <div v-if="pendingUrdfModels.size > 0" class="urdf-prompt">
      <div v-for="model in pendingUrdfModels" :key="model" class="urdf-prompt-row">
        <span class="urdf-prompt-text">
          Модель <strong>{{ model }}</strong> не загружена
        </span>
        <button class="urdf-prompt-btn" @click="requestImport(model)">
          Загрузить URDF
        </button>
      </div>
    </div>

  </div>
</template>

<style scoped>
.viewer-mount {
  position: absolute;
  top: var(--top-bar-h, 0px);
  left: 0; right: 0;
  bottom: var(--bottom-bar-h, 0px);
  z-index: 1;
}
.viewer-mount :deep(canvas) { display: block; }

/* ── Robot labels ── */
.robot-label {
  position: absolute;
  transform: translate(-50%, -100%);
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 3px 8px 3px 6px;
  background: rgba(20, 18, 17, 0.82);
  border: 0.5px solid rgba(255,255,255,0.08);
  border-radius: 8px;
  backdrop-filter: blur(8px);
  pointer-events: none;
  white-space: nowrap;
  margin-bottom: 6px;
}

.robot-label--selected {
  border-color: rgba(232,120,74,0.35);
  background: rgba(232,120,74,0.10);
}

.robot-label__dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: #7AB87A;
  flex-shrink: 0;
}

.robot-label--selected .robot-label__dot {
  background: #E8784A;
  box-shadow: 0 0 4px #E8784A;
}

.robot-label__text {
  font-size: 10px;
  font-weight: 500;
  color: #9E9A93;
  letter-spacing: 0.03em;
}

.robot-label--selected .robot-label__text {
  color: #F5F0EB;
}

.robot-label__badge {
  font-size: 8px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: #E8784A;
}

/* ── URDF prompt ── */
.urdf-prompt {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 20;
  display: flex;
  flex-direction: column;
  gap: 6px;
  pointer-events: all;
}

.urdf-prompt-row {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 14px;
  background: rgba(20, 18, 17, 0.92);
  border: 0.5px solid rgba(232,120,74,0.3);
  border-radius: 10px;
  backdrop-filter: blur(12px);
}

.urdf-prompt-text {
  font-size: 12px;
  color: #9E9A93;
  white-space: nowrap;
}
.urdf-prompt-text strong { color: #F5F0EB; font-weight: 600; }

.urdf-prompt-btn {
  padding: 5px 12px;
  border-radius: 7px;
  border: 0.5px solid rgba(232,120,74,0.5);
  background: rgba(232,120,74,0.12);
  color: #E8784A;
  font-size: 11px;
  font-weight: 500;
  white-space: nowrap;
  transition: background 120ms;
}
.urdf-prompt-btn:hover { background: rgba(232,120,74,0.22); }
</style>
