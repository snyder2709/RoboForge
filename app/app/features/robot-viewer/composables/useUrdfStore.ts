import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

// Глобальный кэш: имя модели → XML строка
const urdfMap = ref<Map<string, string>>(new Map())

export function useUrdfStore() {
  // Открыть file picker и импортировать URDF под указанным именем модели
  async function importUrdf(urdfModel: string): Promise<boolean> {
    const path = await open({
      title: `Загрузить URDF для "${urdfModel}"`,
      filters: [{ name: 'URDF', extensions: ['urdf', 'xml'] }],
      multiple: false,
    })
    if (!path || Array.isArray(path)) return false

    const xml: string = await invoke('read_urdf_file', { path })
    urdfMap.value = new Map(urdfMap.value).set(urdfModel, xml)
    return true
  }

  function getUrdf(urdfModel: string): string | null {
    return urdfMap.value.get(urdfModel) ?? null
  }

  function hasUrdf(urdfModel: string): boolean {
    return urdfMap.value.has(urdfModel)
  }

  return { urdfMap, importUrdf, getUrdf, hasUrdf }
}
