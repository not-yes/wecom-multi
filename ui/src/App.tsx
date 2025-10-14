import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

interface GuiResponse {
  success: boolean
  message: string
  pids: number[]
}

interface InstanceInfo {
  pid: number
  startedAt: string
}

function App() {
  const [instanceCount, setInstanceCount] = useState(3)
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState('')
  const [runningInstances, setRunningInstances] = useState<InstanceInfo[]>([])

  // 加载当前运行的实例
  useEffect(() => {
    loadRunningInstances()
    const interval = setInterval(loadRunningInstances, 3000)
    return () => clearInterval(interval)
  }, [])

  async function loadRunningInstances() {
    try {
      const response = await invoke<GuiResponse>('get_running_instances')
      // 将 PID 转换为 InstanceInfo
      const instances = response.pids.map((pid, index) => ({
        pid,
        startedAt: new Date().toLocaleTimeString('zh-CN', {
          hour: '2-digit',
          minute: '2-digit'
        })
      }))
      setRunningInstances(instances)
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
        <p className="version">v0.3.0</p>
      </div>

      <div className="content">
        {/* 启动控制 */}
        <div className="launch-section">
          <h2>启动控制</h2>
          <div className="control-group">
            <label htmlFor="instance-count">启动数量:</label>
            <div className="input-group">
              <select
                id="instance-count"
                value={instanceCount}
                onChange={(e) => setInstanceCount(parseInt(e.target.value))}
                disabled={loading}
              >
                {[1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map(num => (
                  <option key={num} value={num}>{num}</option>
                ))}
              </select>
              <button
                onClick={handleSpawn}
                disabled={loading}
                className="btn btn-primary"
              >
                {loading ? '启动中...' : '启动多开'}
              </button>
            </div>
          </div>
        </div>

        {/* 运行中的实例 */}
        <div className="instances-section">
          <div className="section-header">
            <h2>运行中的实例 ({runningInstances.length})</h2>
            {runningInstances.length > 0 && (
              <button
                onClick={handleKillAll}
                disabled={loading}
                className="btn btn-danger btn-sm"
              >
                全部关闭
              </button>
            )}
          </div>

          <div className="instances-grid">
            {runningInstances.length === 0 ? (
              <div className="empty-state">
                <p>暂无运行中的实例</p>
                <p className="hint">点击上方"启动多开"按钮开始使用</p>
              </div>
            ) : (
              runningInstances.map((instance, index) => (
                <div key={instance.pid} className="instance-card">
                  <div className="instance-header">
                    <div className="instance-number">实例 #{index + 1}</div>
                    <div className="instance-status">运行中</div>
                  </div>
                  <div className="instance-info">
                    <div className="info-row">
                      <span className="label">进程 ID:</span>
                      <span className="value">{instance.pid}</span>
                    </div>
                    <div className="info-row">
                      <span className="label">启动时间:</span>
                      <span className="value">{instance.startedAt}</span>
                    </div>
                  </div>
                  <button
                    className="btn btn-danger btn-sm btn-block"
                    onClick={async () => {
                      // TODO: 实现单个实例关闭功能
                      setMessage('单个实例关闭功能开发中...')
                    }}
                  >
                    关闭
                  </button>
                </div>
              ))
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
      </div>
    </div>
  )
}

export default App
