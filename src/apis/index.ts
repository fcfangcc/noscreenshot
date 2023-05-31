import { invoke } from '@tauri-apps/api'

export const screenCaptureAccess = async (request?: boolean): Promise<boolean> => {
  return await invoke('screen_capture_access', { request: request || false })
}

export const openWindow = async (name?: 'helper'): Promise<boolean> => {
  return await invoke('open_window', { name: name || 'helper' })
}
