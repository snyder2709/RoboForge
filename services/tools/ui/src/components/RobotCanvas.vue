<template>
  <div ref="wrap" class="canvas-wrap">
    <canvas ref="canvas" />
    <div class="imu-label">
      r {{ fmt(imu.roll) }} &nbsp; p {{ fmt(imu.pitch) }} &nbsp; y {{ fmt(imu.yaw) }}
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/addons/controls/OrbitControls.js'

const props = defineProps({
  servos: { type: Array, required: true },
  imu:    { type: Object, required: true },
  live:   { type: Boolean, default: false },
})

const wrap   = ref(null)
const canvas = ref(null)
const fmt = (v) => v.toFixed(3)

const DEG = Math.PI / 180
const d = (s, i) => (s[i] - 90) * DEG

// ── Three.js state ─────────────────────────────────────────────────────────
let renderer, scene, camera, controls, joints, animId, rotY = 0

function box(w, h, d, color) {
  return new THREE.Mesh(
    new THREE.BoxGeometry(w, h, d),
    new THREE.MeshLambertMaterial({ color }),
  )
}
function piv() { return new THREE.Group() }

function buildHumanoid() {
  const root = piv()
  root.position.y = 0.05

  const torsoPivot = piv()
  const torso = box(0.24, 0.32, 0.13, 0x2a4a6a)
  torso.position.y = 0.16
  torsoPivot.add(torso)

  const headPivot = piv()
  const head = box(0.16, 0.16, 0.16, 0x3a6a8a)
  head.position.y = 0.08
  headPivot.add(head)
  headPivot.position.set(0, 0.34, 0)

  function arm(side) {
    const s = side === 'l' ? -1 : 1
    const sp = piv(); sp.position.set(s * 0.155, 0.30, 0)
    const ua = piv()
    const uaMesh = box(0.08, 0.20, 0.08, 0x1e3a55); uaMesh.position.y = -0.10; ua.add(uaMesh)
    const ep = piv(); ep.position.y = -0.20
    const la = piv()
    const laMesh = box(0.07, 0.18, 0.07, 0x1e3a55); laMesh.position.y = -0.09; la.add(laMesh)
    const wp = piv(); wp.position.y = -0.18
    la.add(wp); ep.add(la); ua.add(ep); sp.add(ua)
    return { sp, ep, wp }
  }

  function leg(side) {
    const s = side === 'l' ? -1 : 1
    const hp = piv(); hp.position.set(s * 0.07, 0, 0)
    const ul = piv()
    const ulMesh = box(0.09, 0.24, 0.09, 0x1e3a55); ulMesh.position.y = -0.12; ul.add(ulMesh)
    const kp = piv(); kp.position.y = -0.24
    const ll = piv()
    const llMesh = box(0.08, 0.22, 0.08, 0x1e3a55); llMesh.position.y = -0.11; ll.add(llMesh)
    const ap = piv(); ap.position.y = -0.22
    const foot = piv()
    const footMesh = box(0.11, 0.05, 0.16, 0x2a4a6a); footMesh.position.set(s * 0.01, -0.025, 0.04)
    foot.add(footMesh); ap.add(foot); ll.add(ap); kp.add(ll); ul.add(kp); hp.add(ul)
    return { hp, kp, ap }
  }

  const lA = arm('l'), rA = arm('r'), lL = leg('l'), rL = leg('r')
  torsoPivot.add(headPivot, lA.sp, rA.sp)
  root.add(torsoPivot, lL.hp, rL.hp)
  scene.add(root)

  return {
    root, headPivot,
    lSP: lA.sp, lEP: lA.ep, lWP: lA.wp,
    rSP: rA.sp, rEP: rA.ep, rWP: rA.wp,
    lHP: lL.hp, lKP: lL.kp, lAP: lL.ap,
    rHP: rL.hp, rKP: rL.kp, rAP: rL.ap,
  }
}

function applyServos(s) {
  const j = joints
  j.headPivot.rotation.y = d(s, 0);  j.headPivot.rotation.x = d(s, 1)
  j.lSP.rotation.x = d(s, 2);  j.lSP.rotation.z = d(s, 3)
  j.lEP.rotation.x = d(s, 4);  j.lWP.rotation.y = d(s, 5)
  j.rSP.rotation.x = d(s, 6);  j.rSP.rotation.z = d(s, 7)
  j.rEP.rotation.x = d(s, 8);  j.rWP.rotation.y = d(s, 9)
  j.lHP.rotation.x = d(s, 10); j.lHP.rotation.z = d(s, 11)
  j.lKP.rotation.x = d(s, 12); j.lAP.rotation.x = d(s, 13); j.lAP.rotation.z = d(s, 14)
  j.rHP.rotation.x = d(s, 15); j.rHP.rotation.z = d(s, 16)
  j.rKP.rotation.x = d(s, 17); j.rAP.rotation.x = d(s, 18); j.rAP.rotation.z = d(s, 19)
}

function resize() {
  const w = wrap.value.clientWidth
  const h = wrap.value.clientHeight
  renderer.setSize(w, h, false)
  camera.aspect = w / h
  camera.updateProjectionMatrix()
}

let ro
onMounted(() => {
  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x060610)

  camera = new THREE.PerspectiveCamera(42, 1, 0.1, 50)
  camera.position.set(0, 0.45, 2.0)
  camera.lookAt(0, 0.35, 0)

  renderer = new THREE.WebGLRenderer({ canvas: canvas.value, antialias: true })
  renderer.setPixelRatio(window.devicePixelRatio)

  scene.add(new THREE.AmbientLight(0xffffff, 0.5))
  const d1 = new THREE.DirectionalLight(0x7eb8f7, 0.9); d1.position.set(1.5, 2, 2); scene.add(d1)
  const d2 = new THREE.DirectionalLight(0x2a4a6a, 0.4); d2.position.set(-1, -1, 1); scene.add(d2)
  const grid = new THREE.GridHelper(2, 10, 0x1a1a3a, 0x0f0f1e); grid.position.y = -0.55; scene.add(grid)

  joints = buildHumanoid()

  controls = new OrbitControls(camera, renderer.domElement)
  controls.target.set(0, 0.35, 0)
  controls.enableDamping = true
  controls.dampingFactor = 0.08
  controls.minDistance = 0.8
  controls.maxDistance = 5
  controls.update()

  resize()

  ro = new ResizeObserver(resize)
  ro.observe(wrap.value)

  const animate = () => {
    animId = requestAnimationFrame(animate)
    if (!props.live) { rotY += 0.005; joints.root.rotation.y = rotY }
    controls.update()
    renderer.render(scene, camera)
  }
  animate()
})

onUnmounted(() => {
  cancelAnimationFrame(animId)
  ro?.disconnect()
  renderer?.dispose()
})

watch(() => props.servos, (s) => { if (joints) applyServos(s) }, { deep: true })
watch(() => props.imu,    (imu) => {
  if (joints && props.live) joints.root.rotation.set(imu.pitch, imu.yaw, imu.roll)
})
</script>

<style scoped>
.canvas-wrap {
  position: relative;
  height: 200px;
  background: #060610;
}
.canvas-wrap canvas { width: 100% !important; height: 100% !important; display: block; }
.imu-label {
  position: absolute;
  bottom: 6px;
  left: 8px;
  color: #3a5a7a;
  font-size: 10px;
  font-family: 'Courier New', monospace;
  pointer-events: none;
}
</style>
