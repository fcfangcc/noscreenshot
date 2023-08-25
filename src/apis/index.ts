import { invoke } from '@tauri-apps/api'

export const screenCaptureAccess = async (request?: boolean): Promise<boolean> => {
  return await invoke('screen_capture_access', { request: request || false })
}

export const openWindow = async (name?: 'helper'): Promise<boolean> => {
  return await invoke('open_window', { name: name || 'helper' })
}

export interface ConfigData {
  shortcut: { screenshot: string }
}

export const config = {
  getConfig: async (): Promise<ConfigData> => {
    return await invoke('get_config')
  },
  setShortcutScreenshot: async (key: string): Promise<ConfigData> => {
    return await invoke('set_shortcut_screenshot', { key: key })
  }
}
