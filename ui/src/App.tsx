import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import './App.css'

interface GuiResponse {
  success: boolean
  message: string
  pids: number[]
}

function App() {
  const [instanceCount, setInstanceCount] = useState(3)
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState('')
  const [runningPids, setRunningPids] = useState<number[]>([])

  // 加载当前运行的实例
  useEffect(() => {
    loadRunningInstances()
    const interval = setInterval(loadRunningInstances, 3000)
    return () => clearInterval(interval)
  }, [])

  async function loadRunningInstances() {
    try {
      const response = await invoke<GuiResponse>('get_running_instances')
      setRunningPids(response.pids)
    } catch (error) {
      console.error('获取运行实例失败:', error)
    }
  }

  async function handleSpawn() {
    setLoading(true)
    setMessage('')

    try {
      const response = await invoke<GuiResponse>('spawn_instances', {
        count: instanceCount,
      })

      setMessage(response.message)
      if (response.success) {
        await loadRunningInstances()
      }
    } catch (error) {
      setMessage(`错误: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  async function handleKillAll() {
    setLoading(true)
    setMessage('')

    try {
      const response = await invoke<GuiResponse>('kill_all_instances')
      setMessage(response.message)
      await loadRunningInstances()
    } catch (error) {
      setMessage(`错误: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="container">
      <div className="header">
        <h1>企业微信多开工具</h1>
        <p className="version">v0.2.0</p>
      </div>

      <div className="content">
        <div className="control-group">
          <label htmlFor="instance-count">实例数量:</label>
          <div className="input-group">
            <input
              id="instance-count"
              type="number"
              min="1"
              max="10"
              value={instanceCount}
              onChange={(e) => setInstanceCount(parseInt(e.target.value) || 1)}
              disabled={loading}
            />
            <button
              onClick={handleSpawn}
              disabled={loading}
              className="btn btn-primary"
            >
              {loading ? '启动中...' : '启动实例'}
            </button>
          </div>
        </div>

        <div className="quick-actions">
          <button
            onClick={() => {
              setInstanceCount(2)
              setTimeout(handleSpawn, 100)
            }}
            disabled={loading}
            className="btn btn-secondary"
          >
            快速启动 2 个
          </button>
          <button
            onClick={() => {
              setInstanceCount(3)
              setTimeout(handleSpawn, 100)
            }}
            disabled={loading}
            className="btn btn-secondary"
          >
            快速启动 3 个
          </button>
          <button
            onClick={() => {
              setInstanceCount(5)
              setTimeout(handleSpawn, 100)
            }}
            disabled={loading}
            className="btn btn-secondary"
          >
            快速启动 5 个
          </button>
        </div>

        <div className="status-section">
          <div className="status-header">
            <h3>运行状态</h3>
            <button
              onClick={handleKillAll}
              disabled={loading || runningPids.length === 0}
              className="btn btn-danger"
            >
              关闭所有实例
            </button>
          </div>

          <div className="running-instances">
            <p>
              当前运行: <strong>{runningPids.length}</strong> 个实例
            </p>
            {runningPids.length > 0 && (
              <div className="pid-list">
                <small>进程 PID: {runningPids.join(', ')}</small>
              </div>
            )}
          </div>
        </div>

        {message && (
          <div className={`message ${message.includes('成功') ? 'success' : 'error'}`}>
            {message}
          </div>
        )}
      </div>

      <div className="footer">
        <p>💡 提示: 窗口关闭后程序会最小化到系统托盘</p>
        <p>🎯 推荐: 8GB 内存建议启动 3-5 个实例</p>
      </div>
    </div>
  )
}

export default App
