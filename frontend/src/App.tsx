import { BrowserRouter, Routes, Route } from 'react-router-dom'
import Layout from './components/Layout'
import ScenesPage from './pages/ScenesPage'
import SceneDetailPage from './pages/SceneDetailPage'
import RenderPage from './pages/RenderPage'

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<ScenesPage />} />
          <Route path="scenes/:id" element={<SceneDetailPage />} />
          <Route path="render/:sceneId" element={<RenderPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  )
}

export default App
