import { Outlet } from 'react-router-dom'

export default function Layout() {
  return (
    <div className="app-container">
      <header className="header">
        <div className="header-logo">
          <div className="logo-icon">🎨</div>
          <h1>NeRF Studio</h1>
        </div>
        <nav>
          <a href="/" style={{ color: 'var(--text-secondary)', textDecoration: 'none' }}>
            Scenes
          </a>
        </nav>
      </header>
      <main className="main-content">
        <Outlet />
      </main>
    </div>
  )
}
