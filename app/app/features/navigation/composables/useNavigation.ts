export type NavSection = 'library' | 'robots' | 'mission' | 'simulation' | 'logs' | 'settings'

export function useNavigation() {
    const activeSection = useState<NavSection | null>('nav.activeSection', () => null)
    const isSettingsPanelOpen = computed(() => activeSection.value === 'settings')

    function activate(section: NavSection) {
        activeSection.value = activeSection.value === section ? null : section
    }

    return { activeSection, isSettingsPanelOpen, activate }
}
