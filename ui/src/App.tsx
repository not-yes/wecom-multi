import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Play, Square, RefreshCw, Trash2 } from 'lucide-react'
import { motion, AnimatePresence } from 'framer-motion'
import { Button } from './components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './components/ui/card'
import './styles/globals.css'

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

  // 自动刷新运行的实例
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
      console.error('加载实例失败:', error)
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
      await loadRunningInstances()
    } catch (error) {
      setMessage(`启动失败: ${error}`)
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
      setMessage(`关闭失败: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-transparent">
      {/* 可拖动标题栏区域 */}
      <div data-tauri-drag-region className="titlebar" />

      {/* 主内容区域 */}
      <div className="container mx-auto p-6 pt-14 max-w-4xl">
        <AnimatePresence mode="wait">
          <motion.div
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            transition={{ duration: 0.2, ease: 'easeInOut' }}
          >
            {/* 标题卡片 */}
            <Card className="mb-6 glass-effect border-border/50">
              <CardHeader>
                <CardTitle className="text-2xl font-semibold tracking-tight">
                  企业微信多开工具
                </CardTitle>
                <CardDescription>
                  同时运行多个企业微信实例,支持不同账号登录
                </CardDescription>
              </CardHeader>
            </Card>

            {/* 控制面板 */}
            <Card className="mb-6 glass-effect border-border/50">
              <CardContent className="pt-6">
                <div className="space-y-6">
                  {/* 实例数量选择 */}
                  <div>
                    <label className="text-sm font-medium text-muted-foreground mb-3 block">
                      选择实例数量
                    </label>
                    <div className="flex gap-2">
                      {[2, 3, 5, 10].map((count) => (
                        <Button
                          key={count}
                          variant={instanceCount === count ? 'default' : 'outline'}
                          size="lg"
                          onClick={() => setInstanceCount(count)}
                          disabled={loading}
                          className="flex-1 transition-all duration-150"
                        >
                          {count} 个
                        </Button>
                      ))}
                    </div>
                  </div>

                  {/* 操作按钮 */}
                  <div className="flex gap-3">
                    <Button
                      size="lg"
                      onClick={handleSpawn}
                      disabled={loading}
                      className="flex-1 gap-2 transition-all duration-150"
                    >
                      <Play className="w-4 h-4" strokeWidth={2} />
                      启动实例
                    </Button>
                    <Button
                      size="lg"
                      variant="outline"
                      onClick={loadRunningInstances}
                      disabled={loading}
                      className="gap-2 transition-all duration-150"
                    >
                      <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} strokeWidth={2} />
                    </Button>
                    <Button
                      size="lg"
                      variant="destructive"
                      onClick={handleKillAll}
                      disabled={loading || runningPids.length === 0}
                      className="gap-2 transition-all duration-150"
                    >
                      <Square className="w-4 h-4" strokeWidth={2} />
                      停止全部
                    </Button>
                  </div>

                  {/* 状态消息 */}
                  <AnimatePresence>
                    {message && (
                      <motion.div
                        initial={{ opacity: 0, height: 0 }}
                        animate={{ opacity: 1, height: 'auto' }}
                        exit={{ opacity: 0, height: 0 }}
                        transition={{ duration: 0.2 }}
                        className="rounded-lg bg-muted px-4 py-3 text-sm text-muted-foreground"
                      >
                        {message}
                      </motion.div>
                    )}
                  </AnimatePresence>
                </div>
              </CardContent>
            </Card>

            {/* 运行中的实例 */}
            <Card className="glass-effect border-border/50">
              <CardHeader>
                <CardTitle className="text-lg font-medium">
                  运行中的实例
                  {runningPids.length > 0 && (
                    <span className="ml-2 text-sm font-normal text-muted-foreground">
                      ({runningPids.length} 个)
                    </span>
                  )}
                </CardTitle>
              </CardHeader>
              <CardContent>
                {runningPids.length === 0 ? (
                  <div className="text-center py-12 text-muted-foreground">
                    <p className="text-sm">暂无运行中的实例</p>
                    <p className="text-xs mt-2">点击"启动实例"开始</p>
                  </div>
                ) : (
                  <div className="space-y-2">
                    <AnimatePresence mode="popLayout">
                      {runningPids.map((pid, index) => (
                        <motion.div
                          key={pid}
                          initial={{ opacity: 0, x: -20 }}
                          animate={{ opacity: 1, x: 0 }}
                          exit={{ opacity: 0, x: 20 }}
                          transition={{ duration: 0.15, delay: index * 0.05 }}
                          className="flex items-center justify-between p-4 rounded-lg bg-muted/50 hover:bg-muted transition-colors duration-150"
                        >
                          <div className="flex items-center gap-3">
                            <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                            <div>
                              <p className="text-sm font-medium">实例 {index + 1}</p>
                              <p className="text-xs text-muted-foreground">PID: {pid}</p>
                            </div>
                          </div>
                          <Button
                            size="icon"
                            variant="ghost"
                            className="h-8 w-8 transition-colors duration-150"
                            onClick={async () => {
                              // TODO: 实现单个实例停止功能
                              console.log('停止实例:', pid)
                            }}
                          >
                            <Trash2 className="w-4 h-4" strokeWidth={2} />
                          </Button>
                        </motion.div>
                      ))}
                    </AnimatePresence>
                  </div>
                )}
              </CardContent>
            </Card>

            {/* 底部提示 */}
            <div className="mt-6 text-center text-xs text-muted-foreground">
              <p>企业微信多开工具 v0.3.0</p>
              <p className="mt-1">支持 Windows 和 macOS</p>
            </div>
          </motion.div>
        </AnimatePresence>
      </div>
    </div>
  )
}

export default App
