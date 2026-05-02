export function useDevToolsViewModel() {
    const isOpen = useState('devtools.isOpen', () => false)

    return {
        isOpen,
        open: () => { isOpen.value = true },
        close: () => { isOpen.value = false },
    }
}
