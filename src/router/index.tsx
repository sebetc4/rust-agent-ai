import { Routes, Route } from 'react-router-dom'
import { HomePage, ChatPage, ModelsPage, SettingsPage } from '@/pages'
import { Path } from '@/constants'

export const Router = () => {
    return (
        <Routes>
            <Route
                path={Path.HOME}
                element={<HomePage />}
            />
            <Route
                path={Path.CHAT}
                element={<ChatPage />}
            />
            <Route
                path={Path.MODELS}
                element={<ModelsPage />}
            />
            <Route
                path={Path.SETTINGS}
                element={<SettingsPage />}
            />
        </Routes>
    )
}
