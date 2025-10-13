import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface ProcessInfo {
  pid: number;
  started_at: string;
}

interface SpawnResponse {
  pids: number[];
  success: number;
  failed: number;
}

function App() {
  const [count, setCount] = useState<number>(2);
  const [wecomPath, setWecomPath] = useState<string>("");
  const [processes, setProcesses] = useState<ProcessInfo[]>([]);
  const [status, setStatus] = useState<{ type: "idle" | "success" | "error"; message: string }>({
    type: "idle",
    message: "",
  });
  const [isLoading, setIsLoading] = useState(false);

  // 获取默认企业微信路径
  useEffect(() => {
    invoke<string>("get_default_wecom_path")
      .then((path) => setWecomPath(path))
      .catch((err) => console.error("获取默认路径失败:", err));
  }, []);

  // 刷新进程列表
  const refreshProcesses = async () => {
    try {
      const procs = await invoke<ProcessInfo[]>("list_processes");
      setProcesses(procs);
    } catch (err) {
      console.error("刷新进程列表失败:", err);
    }
  };

  // 启动多个实例
  const handleSpawn = async () => {
    if (count < 1 || count > 10) {
      setStatus({ type: "error", message: "实例数量必须在 1-10 之间" });
      return;
    }

    setIsLoading(true);
    setStatus({ type: "idle", message: "" });

    try {
      const result = await invoke<SpawnResponse>("spawn_wecom", {
        count,
        wecomPath: wecomPath || undefined,
      });

      setStatus({
        type: "success",
        message: `成功启动 ${result.success} 个实例${result.failed > 0 ? `, 失败 ${result.failed} 个` : ""}`,
      });

      await refreshProcesses();
    } catch (err) {
      setStatus({
        type: "error",
        message: `启动失败: ${err}`,
      });
    } finally {
      setIsLoading(false);
    }
  };

  // 关闭单个进程
  const handleKillProcess = async (pid: number) => {
    try {
      await invoke("kill_process", { pid });
      setStatus({ type: "success", message: `已关闭进程 ${pid}` });
      await refreshProcesses();
    } catch (err) {
      setStatus({ type: "error", message: `关闭进程失败: ${err}` });
    }
  };

  // 关闭所有进程
  const handleKillAll = async () => {
    try {
      await invoke("kill_all_processes");
      setStatus({ type: "success", message: "已关闭所有进程" });
      setProcesses([]);
    } catch (err) {
      setStatus({ type: "error", message: `关闭所有进程失败: ${err}` });
    }
  };

  // 选择企业微信路径
  const handleSelectPath = async () => {
    try {
      const selected = await invoke<string>("select_wecom_path");
      if (selected) {
        setWecomPath(selected);
      }
    } catch (err) {
      console.error("选择路径失败:", err);
    }
  };

  useEffect(() => {
    refreshProcesses();
    const interval = setInterval(refreshProcesses, 5000); // 每5秒刷新
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="container">
      <h1>企业微信多开工具</h1>
      <p className="subtitle">WeWork Multi-Open - 100% 开源</p>

      <div className="card">
        <div className="form-group">
          <label>企业微信路径:</label>
          <div style={{ display: "flex", gap: "0.5rem" }}>
            <input
              type="text"
              value={wecomPath}
              onChange={(e) => setWecomPath(e.target.value)}
              placeholder="自动检测或手动输入路径"
            />
            <button onClick={handleSelectPath}>浏览</button>
          </div>
        </div>

        <div className="form-group">
          <label>启动实例数量:</label>
          <input
            type="number"
            min="1"
            max="10"
            value={count}
            onChange={(e) => setCount(parseInt(e.target.value) || 1)}
          />
        </div>

        <div className="button-group">
          <button onClick={handleSpawn} disabled={isLoading}>
            {isLoading ? "启动中..." : "启动多开"}
          </button>
          <button onClick={refreshProcesses} disabled={isLoading}>
            刷新列表
          </button>
          <button
            onClick={handleKillAll}
            disabled={isLoading || processes.length === 0}
            style={{ background: "#dc2626" }}
          >
            关闭所有
          </button>
        </div>

        {status.message && (
          <div className={`status ${status.type}`}>{status.message}</div>
        )}
      </div>

      {processes.length > 0 && (
        <div className="card">
          <h2>运行中的实例 ({processes.length})</h2>
          <ul className="process-list">
            {processes.map((proc) => (
              <li key={proc.pid} className="process-item">
                <span>
                  PID: {proc.pid} | 启动时间: {proc.started_at}
                </span>
                <button onClick={() => handleKillProcess(proc.pid)}>
                  关闭
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}

      <div className="card" style={{ textAlign: "left", fontSize: "0.9rem" }}>
        <h3>使用说明</h3>
        <ul>
          <li>本工具通过系统级 Mutex 管理实现企业微信多开</li>
          <li>不修改企业微信程序，不注入 DLL，不 Hook</li>
          <li>建议启动 2-5 个实例，过多可能影响性能</li>
          <li>如遇问题，请尝试关闭所有实例后重新启动</li>
        </ul>
      </div>
    </div>
  );
}

export default App;
