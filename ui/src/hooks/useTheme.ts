import { useEffect, useState } from 'react'

export function useTheme() {
  const [isDark, setIsDark] = useState(false)

  useEffect(() => {
    // 检测系统主题
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

    // 初始设置
    setIsDark(mediaQuery.matches)
    updateTheme(mediaQuery.matches)

    // 监听主题变化
    const handler = (e: MediaQueryListEvent) => {
      setIsDark(e.matches)
      updateTheme(e.matches)
    }

    mediaQuery.addEventListener('change', handler)

    return () => mediaQuery.removeEventListener('change', handler)
  }, [])

  const updateTheme = (isDark: boolean) => {
    if (isDark) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }

  return { isDark }
}
