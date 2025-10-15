import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Play, Square, RefreshCw, Trash2, Plus, Minus } from 'lucide-react'
import { motion, AnimatePresence } from 'framer-motion'
import { Button } from './components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './components/ui/card'
import { useTheme } from './hooks/useTheme'
import './styles/globals.css'

interface GuiResponse {
  success: boolean
  message: string
  pids: number[]
}

function App() {
  const [instanceCount, setInstanceCount] = useState(2)
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState('')
  const [runningPids, setRunningPids] = useState<number[]>([])
  const [appType, setAppType] = useState<'wecom' | 'wechat'>('wecom')
  const [isolationMode, setIsolationMode] = useState<'simple' | 'sandboxie'>('simple')
  const [sandboxieAvailable, setSandboxieAvailable] = useState(false)
  const [platform, setPlatform] = useState<'windows' | 'macos' | 'other'>('other')

  // 自动切换暗黑模式
  useTheme()

  // 检测平台并添加对应的 class
  useEffect(() => {
    const platformStr = navigator.platform.toLowerCase()
    if (platformStr.includes('mac')) {
      document.documentElement.classList.add('platform-macos')
      setPlatform('macos')
    } else if (platformStr.includes('win')) {
      document.documentElement.classList.add('platform-windows')
      setPlatform('windows')
      // Windows平台检测Sandboxie
      checkSandboxie()
    } else {
      setPlatform('other')
    }
  }, [])

  async function checkSandboxie() {
    try {
      const available = await invoke<boolean>('check_sandboxie_available')
      setSandboxieAvailable(available)
      if (!available) {
        console.log('Sandboxie-Plus 未安装或不可用')
      }
    } catch (error) {
      console.error('检测Sandboxie失败:', error)
      setSandboxieAvailable(false)
    }
  }

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
        appType: appType,
        isolationMode: isolationMode,
      })

      setMessage(response.message)
      await loadRunningInstances()
    } catch (error) {
      setMessage(`启动失败: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  async function handleKillInstance(pid: number) {
    try {
      const response = await invoke<GuiResponse>('kill_instance', { pid })
      setMessage(response.message)
      await loadRunningInstances()
    } catch (error) {
      setMessage(`关闭实例失败: ${error}`)
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
    <div className="min-h-screen bg-background">
      {/* 可拖动标题栏区域 - 暂时隐藏以使用系统默认标题栏 */}
      {/* <div data-tauri-drag-region className="titlebar" /> */}

      {/* 主内容区域 */}
      <div className="w-full h-full p-4">
        <AnimatePresence mode="wait">
          <motion.div
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            transition={{ duration: 0.2, ease: 'easeInOut' }}
          >
            {/* 控制面板 */}
            <Card className="mb-6 glass-effect border-border/50">
              <CardHeader className="pb-4">
                <CardTitle className="text-xl font-semibold">微信多开工具</CardTitle>
                <CardDescription className="text-xs">
                  同时运行多个微信/企业微信实例
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                {/* 应用类型选择 */}
                <div className="space-y-2">
                  <label className="text-xs font-medium text-muted-foreground">应用类型</label>
                  <div className="flex gap-2">
                    <Button
                      variant={appType === 'wecom' ? 'default' : 'outline'}
                      size="sm"
                      onClick={() => setAppType('wecom')}
                      disabled={loading}
                      className="flex-1 transition-all duration-200"
                    >
                      企业微信
                    </Button>
                    <Button
                      variant={appType === 'wechat' ? 'default' : 'outline'}
                      size="sm"
                      onClick={() => setAppType('wechat')}
                      disabled={loading}
                      className="flex-1 transition-all duration-200"
                    >
                      个人微信
                    </Button>
                  </div>
                </div>

                {/* 隔离模式选择 (仅Windows) */}
                {platform === 'windows' && (
                  <div className="space-y-2">
                    <label className="text-xs font-medium text-muted-foreground">
                      隔离模式
                      {!sandboxieAvailable && isolationMode === 'sandboxie' && (
                        <span className="ml-2 text-[10px] text-orange-500">
                          ⚠️ 需要安装Sandboxie-Plus
                        </span>
                      )}
                    </label>
                    <div className="flex gap-2">
                      <Button
                        variant={isolationMode === 'simple' ? 'default' : 'outline'}
                        size="sm"
                        onClick={() => setIsolationMode('simple')}
                        disabled={loading}
                        className="flex-1 transition-all duration-200"
                      >
                        <div className="text-left">
                          <div className="text-xs">简单模式</div>
                          <div className="text-[9px] opacity-70">共享数据</div>
                        </div>
                      </Button>
                      <Button
                        variant={isolationMode === 'sandboxie' ? 'default' : 'outline'}
                        size="sm"
                        onClick={() => setIsolationMode('sandboxie')}
                        disabled={loading}
                        className="flex-1 transition-all duration-200"
                      >
                        <div className="text-left">
                          <div className="text-xs">沙盒隔离</div>
                          <div className="text-[9px] opacity-70">
                            {sandboxieAvailable ? '✓ 已安装' : '未安装'}
                          </div>
                        </div>
                      </Button>
                    </div>
                  </div>
                )}

                {/* macOS 提示 */}
                {platform === 'macos' && (
                  <div className="rounded-lg bg-blue-500/10 border border-blue-500/20 px-3 py-2">
                    <p className="text-[10px] text-blue-600 dark:text-blue-400">
                      ✨ macOS 自动启用完全数据隔离
                    </p>
                  </div>
                )}

                {/* 操作区域 */}
                <div className="flex items-center gap-3">
                  {/* 数量控制 */}
                  <div className="flex items-center gap-2">
                    <Button
                      size="icon"
                      variant="outline"
                      onClick={() => setInstanceCount(Math.max(1, instanceCount - 1))}
                      disabled={loading || instanceCount <= 1}
                      className="h-9 w-9 transition-all duration-200"
                    >
                      <Minus className="w-4 h-4" strokeWidth={2} />
                    </Button>
                    <div className="min-w-[60px] text-center">
                      <div className="text-2xl font-semibold">{instanceCount}</div>
                      <div className="text-xs text-muted-foreground">实例</div>
                    </div>
                    <Button
                      size="icon"
                      variant="outline"
                      onClick={() => setInstanceCount(Math.min(20, instanceCount + 1))}
                      disabled={loading || instanceCount >= 20}
                      className="h-9 w-9 transition-all duration-200"
                    >
                      <Plus className="w-4 h-4" strokeWidth={2} />
                    </Button>
                  </div>

                  {/* 操作按钮 */}
                  <Button
                    size="lg"
                    onClick={handleSpawn}
                    disabled={loading}
                    className="flex-1 gap-2 transition-all duration-200"
                  >
                    <Play className="w-4 h-4" strokeWidth={2} />
                    启动
                  </Button>
                  <Button
                    size="icon"
                    variant="outline"
                    onClick={loadRunningInstances}
                    disabled={loading}
                    className="h-10 w-10 transition-all duration-200"
                  >
                    <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} strokeWidth={2} />
                  </Button>
                  <Button
                    size="lg"
                    variant="destructive"
                    onClick={handleKillAll}
                    disabled={loading || runningPids.length === 0}
                    className="gap-2 transition-all duration-200"
                  >
                    <Square className="w-4 h-4" strokeWidth={2} />
                    全部停止
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
                      className="rounded-lg bg-muted px-3 py-2 text-xs text-muted-foreground"
                    >
                      {message}
                    </motion.div>
                  )}
                </AnimatePresence>
              </CardContent>
            </Card>

            {/* 运行中的实例 */}
            <Card className="glass-effect border-border/50">
              <CardHeader className="pb-3">
                <CardTitle className="text-base font-medium flex items-center justify-between">
                  <span>运行中的实例</span>
                  {runningPids.length > 0 && (
                    <span className="text-xs font-normal text-muted-foreground">
                      {runningPids.length} 个
                    </span>
                  )}
                </CardTitle>
              </CardHeader>
              <CardContent>
                {runningPids.length === 0 ? (
                  <div className="text-center py-8 text-muted-foreground">
                    <p className="text-xs">暂无运行中的实例</p>
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
                          transition={{ duration: 0.2, delay: index * 0.05, ease: 'easeInOut' }}
                          className="flex items-center justify-between p-3 rounded-lg bg-muted/50 hover:bg-muted transition-colors duration-200"
                        >
                          <div className="flex items-center gap-2">
                            <div className="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse" />
                            <div>
                              <p className="text-xs font-medium">实例 {index + 1}</p>
                              <p className="text-[10px] text-muted-foreground">PID: {pid}</p>
                            </div>
                          </div>
                          <Button
                            size="icon"
                            variant="ghost"
                            className="h-7 w-7 transition-colors duration-200"
                            onClick={() => handleKillInstance(pid)}
                          >
                            <Trash2 className="w-3.5 h-3.5" strokeWidth={2} />
                          </Button>
                        </motion.div>
                      ))}
                    </AnimatePresence>
                  </div>
                )}
              </CardContent>
            </Card>

            {/* 底部提示 */}
            <div className="mt-4 text-center text-[10px] text-muted-foreground">
              <p>v0.5.0 · 微信/企业微信 · Windows 沙盒隔离 · macOS 完全隔离</p>
            </div>
          </motion.div>
        </AnimatePresence>
      </div>
    </div>
  )
}

export default App
