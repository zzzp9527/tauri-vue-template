import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";

/**
 * 后续随项目更新做更详细更完善的日志记录
 */
export function useLogger(moduleName = '') {

  return {
    warn: (msg: string) => warn(`${moduleName} ${msg}`),
    debug: (msg: string) => debug(`${moduleName} ${msg}`),
    trace: (msg: string) => trace(`${moduleName} ${msg}`),
    info: (msg: string) => info(`${moduleName} ${msg}`),
    error: (msg: string) => error(`${moduleName} ${msg}`),
  };
}
