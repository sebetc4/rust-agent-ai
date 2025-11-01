import './index.css'
import { useLLMStore } from '@/stores/llm'
import { useHydration } from '@/hooks/useHydration'
import { ErrorPage } from '@/pages'
import { AppLoader } from '@/components/common'
import { Router } from './router'

const App = () => {
    const { isLoading: isModelLoading, error: modelError } = useLLMStore()
    const { isHydrated, error: hydrationError } = useHydration()

    if (!isHydrated || isModelLoading) {
        return <AppLoader />
    }

    if (hydrationError) {
        return <ErrorPage error={hydrationError} />
    }

    if (modelError) {
        return <ErrorPage error={modelError} />
    }

    return <Router />
}

export default App
