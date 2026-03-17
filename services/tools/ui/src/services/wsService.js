/**
 * WebSocket service — singleton bridge to ui_server.py (ws://localhost:8766).
 * Layer: Model (external data source in MVVM).
 */

import { ref } from 'vue'

const WS_URL = 'ws://localhost:8766'

export const wsStatus = ref('connecting')

let ws = null
const _listeners = new Set()

function connect() {
  ws = new WebSocket(WS_URL)

  ws.onopen = () => {
    wsStatus.value = 'connected'
  }
  ws.onclose = () => {
    wsStatus.value = 'disconnected'
    setTimeout(connect, 2000)
  }
  ws.onerror = () => {}
  ws.onmessage = (e) => {
    try {
      const msg = JSON.parse(e.data)
      _listeners.forEach((fn) => fn(msg))
    } catch (_) {}
  }
}

connect()

export function useWsService() {
  /** @param {(msg: object) => void} fn  @returns {() => void} unsubscribe */
  function onMessage(fn) {
    _listeners.add(fn)
    return () => _listeners.delete(fn)
  }

  /** @param {object} data */
  function send(data) {
    if (ws?.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(data))
    }
  }

  return { wsStatus, onMessage, send }
}
